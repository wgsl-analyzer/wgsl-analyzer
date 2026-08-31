use std::{num::NonZeroU32, ops::Index};

use base_db::{Intern as _, Lookup as _};
use either::Either;
use hir_def::{
    HasSource as _,
    body::{BindingId, Body, scope::ExprScopes},
    db::{DefinitionWithBodyId, ModuleDefinitionId, StructId},
    expression::{
        BinaryOperation, Expression, ExpressionId, Statement, StatementId, SwitchCaseSelector,
        UnaryOperator,
    },
    expression_store::{ExpressionStore, ExpressionStoreSource},
    item_tree::Name,
    resolver::Resolver,
    signature::{
        ConstantSignature, FieldId, FunctionSignature, OverrideSignature, StructSignature,
        TypeAliasSignature, VariableSignature,
    },
    type_specifier::{IdentExpression, TypeSpecifierId},
};
use itertools::Itertools as _;
use la_arena::ArenaMap;
use rustc_hash::FxHashMap;
use wgsl_types::{
    syntax::{AccessMode, AddressSpace, Enumerant},
    tplt::TpltParam,
    ty::Ty as _,
};

use crate::{
    db::HirDatabase,
    diagnostics::{InferenceDiagnostic, InferenceDiagnosticKind},
    function::FunctionDetails,
    lower::{
        ConstructibleTypeGenerator, DefaultTypes, Lowered, LoweredKind, ResolvedCall,
        TemplateParameter, TemplateParameters, TypeContainer, TypeLoweringContext,
        TypeLoweringError, WgslTypeConverter, to_wgsl_binary_operator, to_wgsl_unary_operator,
    },
    ty::{
        ArraySize, ArrayType, BuiltinStruct, IndexList, MatrixType, ParseIndexListError, Pointer,
        Reference, ScalarType, SwizzleView, Type, TypeKind, VecIndex, VectorType,
    },
};

#[salsa::tracked]
impl InferenceResult {
    /// Infers the type of a global item.
    /// For `const`s and co, it first uses the specified type,
    /// and then uses the body (expression) to infer the return type.
    pub fn of(
        db: &dyn HirDatabase,
        definition: DefinitionWithBodyId,
    ) -> &Self {
        infer_query(db, definition)
    }
}

#[salsa::tracked(returns(ref), cycle_result = infer_cycle_result)]
fn infer_query(
    db: &dyn HirDatabase,
    definition: DefinitionWithBodyId,
) -> InferenceResult {
    let resolver = definition.resolver(db);
    let body = Body::of(db, definition);
    let mut context = InferenceContext::new(db, definition.into(), resolver);

    match definition {
        DefinitionWithBodyId::Function(function) => {
            let data = FunctionSignature::of(db, function);
            let return_type = context.collect_fn(data, body);
            context.infer_body(body, return_type, AbstractHandling::Concretize);
        },
        DefinitionWithBodyId::GlobalVariable(variable) => {
            let data = VariableSignature::of(db, variable);
            let return_type = context.collect_global_variable(data, body);
            context.infer_body(body, return_type, AbstractHandling::Concretize);
            context.infer_global_variable(data, body);
        },
        DefinitionWithBodyId::GlobalConstant(constant) => {
            let data = ConstantSignature::of(db, constant);
            let return_type = context.collect_global_constant(data, body);
            context.infer_body(body, return_type, AbstractHandling::Abstract);
        },
        DefinitionWithBodyId::Override(override_declaration) => {
            let data = OverrideSignature::of(db, override_declaration);
            let return_type = context.collect_override(data, body);
            context.infer_body(body, return_type, AbstractHandling::Concretize);
        },
        DefinitionWithBodyId::GlobalAssertStatement(_global_assert_statement) => {
            let expression = body.root.and_then(Either::right);
            if let Some(expression) = expression {
                let expected_type =
                    TypeExpectation::from_type(TypeKind::Scalar(ScalarType::Bool).intern(db));
                context.infer_expression_expect(expression, expected_type, &body.store);
            }
        },
    }

    context.resolve_all()
}

fn infer_cycle_result(
    db: &dyn HirDatabase,
    _: salsa::Id,
    definition: DefinitionWithBodyId,
) -> InferenceResult {
    let mut inference_result = InferenceResult::new(TypeKind::Error.intern(db));
    let (name, range) = get_name_and_range(db, ModuleDefinitionId::from(definition));

    inference_result.diagnostics.push(InferenceDiagnostic {
        source: ExpressionStoreSource::Body,
        kind: InferenceDiagnosticKind::CyclicType { name, range },
    });

    inference_result
}

pub fn get_name_and_range(
    db: &dyn HirDatabase,
    definition: ModuleDefinitionId,
) -> (Name, base_db::TextRange) {
    match definition {
        ModuleDefinitionId::Function(id) => (
            FunctionSignature::of(db, id).name.clone(),
            id.lookup(db).source(db).original_file_range(db).range,
        ),
        ModuleDefinitionId::GlobalVariable(id) => (
            VariableSignature::of(db, id).name.clone(),
            id.lookup(db).source(db).original_file_range(db).range,
        ),
        ModuleDefinitionId::GlobalConstant(id) => (
            ConstantSignature::of(db, id).name.clone(),
            id.lookup(db).source(db).original_file_range(db).range,
        ),
        ModuleDefinitionId::Override(id) => (
            OverrideSignature::of(db, id).name.clone(),
            id.lookup(db).source(db).original_file_range(db).range,
        ),
        ModuleDefinitionId::Struct(id) => (
            StructSignature::of(db, id).name.clone(),
            id.lookup(db).source(db).original_file_range(db).range,
        ),
        ModuleDefinitionId::TypeAlias(id) => (
            TypeAliasSignature::of(db, id).name.clone(),
            id.lookup(db).source(db).original_file_range(db).range,
        ),
        ModuleDefinitionId::GlobalAssertStatement(id) => (
            Name::from("const_assert"),
            id.lookup(db).source(db).original_file_range(db).range,
        ),
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InferenceResult {
    pub(crate) type_of_expression: ArenaMap<ExpressionId, Type>,
    pub(crate) type_of_binding: ArenaMap<BindingId, Type>,
    diagnostics: Vec<InferenceDiagnostic>,
    return_type: Type,
    call_resolutions: FxHashMap<ExpressionId, ResolvedCall>,
    field_resolutions: FxHashMap<ExpressionId, FieldId>,
    error_type: Type,
}

impl InferenceResult {
    fn new(error_type: Type) -> Self {
        Self {
            type_of_expression: ArenaMap::default(),
            type_of_binding: ArenaMap::default(),
            diagnostics: Vec::default(),
            return_type: error_type,
            call_resolutions: FxHashMap::default(),
            field_resolutions: FxHashMap::default(),
            error_type,
        }
    }

    #[must_use]
    pub fn field_resolution(
        &self,
        expression: ExpressionId,
    ) -> Option<FieldId> {
        self.field_resolutions.get(&expression).copied()
    }

    #[must_use]
    pub fn call_resolution(
        &self,
        expression: ExpressionId,
    ) -> Option<ResolvedCall> {
        self.call_resolutions.get(&expression).copied()
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[InferenceDiagnostic] {
        &self.diagnostics
    }

    #[must_use]
    pub const fn return_type(&self) -> Type {
        self.return_type
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.type_of_expression.values().next().is_none()
            && self.type_of_binding.values().next().is_none()
            && self.diagnostics.is_empty()
            && self.call_resolutions.is_empty()
            && self.field_resolutions.is_empty()
    }
}

impl Index<ExpressionId> for InferenceResult {
    type Output = Type;

    fn index(
        &self,
        index: ExpressionId,
    ) -> &Type {
        self.type_of_expression
            .get(index)
            .unwrap_or(&self.error_type)
    }
}

impl Index<BindingId> for InferenceResult {
    type Output = Type;

    fn index(
        &self,
        index: BindingId,
    ) -> &Type {
        self.type_of_binding.get(index).unwrap_or(&self.error_type)
    }
}

/// Runs inference for items that have a body, such as functions.
pub struct InferenceContext<'db> {
    db: &'db dyn HirDatabase,
    owner: ModuleDefinitionId,
    /// Root resolver for the entire module.
    resolver: Resolver<'db>,
    result: InferenceResult, // set in collect_* calls
    return_type: Type,
    converter: WgslTypeConverter<'db>,
}

impl<'db> InferenceContext<'db> {
    pub fn new(
        db: &'db dyn HirDatabase,
        owner: ModuleDefinitionId,
        resolver: Resolver<'db>,
    ) -> Self {
        let types = DefaultTypes::new(db);
        Self {
            db,
            owner,
            resolver,
            result: InferenceResult::new(types.error),
            return_type: types.error,
            converter: WgslTypeConverter::new(db),
        }
    }

    // pub fn with_store<T>(
    //     &mut self,
    //     store: &'db ExpressionStore,
    //     f: impl FnOnce(&mut InferenceContext<'_>) -> T,
    // ) -> T {
    //     let old_store = std::mem::replace(&mut self.store, store);
    //     let result = f(self);
    //     self.store = old_store;
    //     result
    // }

    fn set_expression_type(
        &mut self,
        expression: ExpressionId,
        r#type: Type,
    ) {
        self.result.type_of_expression.insert(expression, r#type);
    }

    fn set_binding_type(
        &mut self,
        binding: BindingId,
        r#type: Type,
    ) {
        self.result.type_of_binding.insert(binding, r#type);
    }

    fn bind_return_type(
        &mut self,
        r#type: Option<Type>,
        body: &Body,
    ) {
        if let Some(r#type) = r#type
            && let Some(binding) = body.main_binding
        {
            self.set_binding_type(binding, r#type);
        }

        self.return_type = r#type.unwrap_or_else(|| self.error_type());
    }

    fn set_field_resolution(
        &mut self,
        expression: ExpressionId,
        field: FieldId,
    ) {
        self.result.field_resolutions.insert(expression, field);
    }

    fn push_diagnostic(
        &mut self,
        source: ExpressionStoreSource,
        diagnostic: InferenceDiagnosticKind,
    ) {
        self.result.diagnostics.push(InferenceDiagnostic {
            source,
            kind: diagnostic,
        });
    }

    fn push_lowering_diagnostics(
        &mut self,
        diagnostics: Vec<TypeLoweringError>,
        store: &ExpressionStore,
    ) {
        for diagnostic in diagnostics {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::InvalidType { error: diagnostic },
            );
        }
    }

    fn resolve_all(mut self) -> InferenceResult {
        self.result.return_type = self.return_type;
        self.result
    }

    fn collect_global_variable(
        &mut self,
        variable: &VariableSignature,
        body: &Body,
    ) -> Option<Type> {
        let r#type = variable
            .r#type
            .map(|r#type| self.lower_type(r#type, &self.resolver.clone(), &variable.store));

        self.bind_return_type(r#type, body);
        r#type
    }

    fn infer_global_variable(
        &mut self,
        variable: &VariableSignature,
        body: &Body,
    ) {
        let (address_space, access_mode) =
            self.infer_variable_template(&variable.template_parameters, &variable.store);
        if address_space == AddressSpace::Function {
            // Function address space is not allowed at the module level
            self.push_diagnostic(
                variable.store.store_source,
                InferenceDiagnosticKind::UnexpectedTemplateArgument {
                    expression: variable.template_parameters[0],
                },
            );
        }

        self.bind_return_type(
            Some(self.make_ref(self.return_type, address_space, access_mode)),
            body,
        );
    }

    fn infer_variable_template(
        &mut self,
        template: &[ExpressionId],
        store: &ExpressionStore,
    ) -> (AddressSpace, AccessMode) {
        let mut context = TypeLoweringContext::new(self.db, &self.resolver, store);
        let template_args: Vec<_> = template
            .iter()
            .map(|argument| context.evaluate_template_argument(*argument))
            .collect();
        self.push_lowering_diagnostics(context.diagnostics, store);

        let default_address_space = match store.store_source {
            ExpressionStoreSource::Body => AddressSpace::Function,
            ExpressionStoreSource::Signature => AddressSpace::Handle,
        };

        let address_space = match template_args.first() {
            Some(TemplateParameter::Enumerant(Enumerant::AddressSpace(address_space))) => {
                *address_space
            },
            None => default_address_space,
            _ => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::UnexpectedTemplateArgument {
                        expression: template[0],
                    },
                );
                default_address_space
            },
        };
        let access_mode = match template_args.get(1) {
            Some(TemplateParameter::Enumerant(Enumerant::AccessMode(access_mode))) => {
                if address_space == AddressSpace::Storage {
                    *access_mode
                } else {
                    // Only the storage address space allows for an access mode
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::UnexpectedTemplateArgument {
                            expression: template[0],
                        },
                    );
                    address_space.default_access_mode()
                }
            },
            None => address_space.default_access_mode(),
            _ => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::UnexpectedTemplateArgument {
                        expression: template[0],
                    },
                );
                address_space.default_access_mode()
            },
        };

        // Mark extra template arguments as errors
        if template.len() > 2 {
            for expression in &template[2..] {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::UnexpectedTemplateArgument {
                        expression: *expression,
                    },
                );
            }
        }
        (address_space, access_mode)
    }

    fn collect_global_constant(
        &mut self,
        constant: &ConstantSignature,
        body: &Body,
    ) -> Option<Type> {
        let r#type = constant
            .r#type
            .map(|r#type| self.lower_type(r#type, &self.resolver.clone(), &constant.store));

        self.bind_return_type(r#type, body);
        r#type
    }

    fn collect_override(
        &mut self,
        override_data: &OverrideSignature,
        body: &Body,
    ) -> Option<Type> {
        let r#type = override_data
            .r#type
            .map(|r#type| self.lower_type(r#type, &self.resolver.clone(), &override_data.store));

        self.bind_return_type(r#type, body);
        r#type
    }

    fn collect_fn(
        &mut self,
        function_data: &FunctionSignature,
        body: &Body,
    ) -> Option<Type> {
        for ((_, parameter), &binding_id) in function_data.parameters.iter().zip(&body.parameters) {
            let parameter_type = self.lower_type(
                parameter.r#type,
                &self.resolver.clone(),
                &function_data.store,
            );
            self.set_binding_type(binding_id, parameter_type);
        }
        let r#type = function_data.return_type.map(|type_ref| {
            self.lower_type(type_ref, &self.resolver.clone(), &function_data.store)
        });
        self.return_type = r#type.unwrap_or_else(|| self.error_type());
        r#type
    }

    /// Runs type inference on the body and infer the type for `const`s, `var`s and `override`s.
    fn infer_body(
        &mut self,
        body: &Body,
        return_type: Option<Type>,
        abstract_handling: AbstractHandling,
    ) {
        match body.root {
            Some(Either::Left(statement)) => {
                self.infer_statement(statement, body, return_type);
            },
            Some(Either::Right(expression)) => {
                let r#type =
                    self.infer_initializer(body, Some(expression), return_type, abstract_handling);

                if return_type.is_none() {
                    self.bind_return_type(Some(r#type), body);
                }
            },
            None => (),
        }
    }

    fn resolver_for_expression(
        &self,
        expression: ExpressionId,
    ) -> Option<Resolver<'db>> {
        let ModuleDefinitionId::Function(function) = self.owner else {
            return None;
        };
        let expression_scopes = ExprScopes::of(self.db, DefinitionWithBodyId::Function(function));

        let scope_id = expression_scopes.scope_for_expression(expression)?;

        Some(
            self.resolver
                .clone()
                .push_expression_scope(function, expression_scopes, scope_id),
        )
    }

    fn resolver_for_statement(
        &self,
        statement: StatementId,
    ) -> Resolver<'db> {
        let ModuleDefinitionId::Function(function) = self.owner else {
            return self.resolver.clone();
        };

        let expression_scopes = ExprScopes::of(self.db, DefinitionWithBodyId::Function(function));

        if let Some(scope_id) = expression_scopes.scope_for_statement(statement) {
            self.resolver
                .clone()
                .push_expression_scope(function, expression_scopes, scope_id)
        } else {
            self.resolver.clone()
        }
    }

    #[expect(clippy::too_many_lines, reason = "match with many small cases")]
    fn infer_statement(
        &mut self,
        statement: StatementId,
        body: &Body,
        return_type: Option<Type>,
    ) {
        let resolver = self.resolver_for_statement(statement);

        match &body.statements[statement] {
            Statement::Compound { statements } | Statement::ConditionalCompound { statements } => {
                for statement in statements {
                    self.infer_statement(*statement, body, return_type);
                }
            },
            Statement::Variable {
                binding_id,
                type_ref,
                initializer,
                template_parameters,
            } => {
                // The store type is the effective-value-type of the variable’s declaration.
                let mut r#type =
                    self.get_effective_value_type(body, &resolver, *type_ref, *initializer);
                if let Some(initializer_expression) = initializer
                    && !r#type.kind(self.db).is_storable()
                    && !r#type.is_err(self.db)
                {
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::StoreTypeMustBeStorable {
                            actual: r#type,
                            expression: *initializer_expression,
                        },
                    );
                    // this ensures that make_ref has a valid input and analysis can continue
                    r#type = TypeKind::Error.intern(self.db);
                }

                let (address_space, access_mode) =
                    self.infer_variable_template(template_parameters, body);
                if address_space != AddressSpace::Function {
                    // Only function address space is allowed
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::UnexpectedTemplateArgument {
                            expression: template_parameters[0],
                        },
                    );
                }
                let reference_type = self.make_ref(r#type, address_space, access_mode);
                self.set_binding_type(*binding_id, reference_type);
            },
            Statement::Const {
                binding_id,
                type_ref,
                initializer,
            } => {
                let r#type = type_ref.map(|r#type| self.lower_type(r#type, &resolver, body));
                let r#type =
                    self.infer_initializer(body, *initializer, r#type, AbstractHandling::Abstract);
                self.set_binding_type(*binding_id, r#type);
            },
            Statement::Let {
                binding_id,
                type_ref,
                initializer,
            } => {
                let r#type = type_ref.map(|r#type| self.lower_type(r#type, &resolver, body));
                let r#type = self.infer_initializer(
                    body,
                    *initializer,
                    r#type,
                    AbstractHandling::Concretize,
                );
                self.set_binding_type(*binding_id, r#type);
            },

            Statement::Return { expression } => match (expression, return_type) {
                (Some(expression), Some(return_type)) => {
                    self.infer_expression_expect(
                        *expression,
                        TypeExpectation::from_type(return_type),
                        body,
                    );
                },
                (Some(expression), None) => {
                    let actual = self.infer_expression_expect(
                        *expression,
                        TypeExpectation::from_type(self.return_type),
                        body,
                    );
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::UnexpectedReturnValue {
                            expression: *expression,
                            actual,
                        },
                    );
                },
                _ => (),
            },
            Statement::Assignment {
                left_side,
                right_side,
            } => {
                let left_type = self.infer_expression(*left_side, body);
                let left_loaded = match left_type.kind(self.db) {
                    // apply the load rule
                    TypeKind::Reference(reference) => reference.inner,
                    // apply the swizzle load rule
                    TypeKind::SwizzleView(swizzle_view) => swizzle_view.loaded().intern(self.db),
                    TypeKind::Error
                    | TypeKind::Scalar(_)
                    | TypeKind::Atomic(_)
                    | TypeKind::Vector(_)
                    | TypeKind::Matrix(_)
                    | TypeKind::Struct(_)
                    | TypeKind::BuiltinStruct(_)
                    | TypeKind::Array(_)
                    | TypeKind::Texture(_)
                    | TypeKind::Sampler(_)
                    | TypeKind::Pointer(_)
                    | TypeKind::AccelerationStructure(_) => {
                        if !left_type.is_err(self.db) {
                            self.push_diagnostic(
                                body.store_source,
                                InferenceDiagnosticKind::AssignmentNotAReference {
                                    left_side: *left_side,
                                    actual: left_type,
                                },
                            );
                        }
                        // helpful instead of full error
                        left_type
                    },
                };
                self.infer_expression_expect(
                    *right_side,
                    TypeExpectation::from_type(left_loaded),
                    body,
                );
            },
            Statement::CompoundAssignment {
                left_side,
                right_side,
                operator,
            } => {
                let left_type = self.infer_expression(*left_side, body);

                let left_kind = left_type.kind(self.db);
                let left_inner = if let TypeKind::Reference(reference) = left_kind {
                    reference.inner
                } else {
                    if !left_type.is_err(self.db) {
                        self.push_diagnostic(
                            body.store_source,
                            InferenceDiagnosticKind::AssignmentNotAReference {
                                left_side: *left_side,
                                actual: left_type,
                            },
                        );
                    }
                    // helpful instead of full error
                    left_type
                };

                let r#type = self.infer_binary_op(
                    *right_side,
                    *left_side,
                    *right_side,
                    (*operator).into(),
                    body,
                );

                if !r#type.is_convertible_to(left_inner, self.db) {
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression: *right_side,
                            actual: r#type,
                            expected: TypeExpectation::Type(TypeExpectationInner::Exact(
                                left_inner,
                            )),
                        },
                    );
                }
            },
            Statement::PhonyAssignment { right_side } => {
                self.infer_expression(*right_side, body);
            },
            Statement::IncrDecr {
                expression,
                operator: _,
            } => {
                let left_type = self.infer_expression(*expression, body);

                let left_kind = left_type.kind(self.db);
                let left_inner = if let TypeKind::Reference(reference) = left_kind {
                    reference.inner
                } else {
                    if !left_type.is_err(self.db) {
                        self.push_diagnostic(
                            body.store_source,
                            InferenceDiagnosticKind::AssignmentNotAReference {
                                left_side: *expression,
                                actual: left_type,
                            },
                        );
                    }
                    // helpful instead of full error
                    left_type
                };

                if self
                    .expect_type_inner(left_inner, TypeExpectationInner::IntegerScalar)
                    .is_err()
                {
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression: *expression,
                            actual: left_inner,
                            expected: TypeExpectation::Type(TypeExpectationInner::IntegerScalar),
                        },
                    );
                }
            },
            Statement::If {
                condition,
                block,
                else_if_blocks,
                else_block,
            } => {
                self.infer_statement(*block, body, return_type);
                for else_if_block in else_if_blocks {
                    self.infer_statement(*else_if_block, body, return_type);
                }
                if let Some(else_block) = else_block {
                    self.infer_statement(*else_block, body, return_type);
                }
                self.infer_expression_expect(
                    *condition,
                    TypeExpectation::from_type(self.bool_type()),
                    body,
                );
            },
            Statement::While { condition, block } => {
                self.infer_statement(*block, body, return_type);
                self.infer_expression_expect(
                    *condition,
                    TypeExpectation::from_type(self.bool_type()),
                    body,
                );
            },
            Statement::Switch {
                expression,
                case_blocks,
            } => {
                let r#type = self.infer_expression(*expression, body).loaded(self.db);

                for (selectors, case) in case_blocks {
                    for selector in selectors {
                        if let SwitchCaseSelector::Expression(selector) = selector {
                            self.infer_expression_expect(
                                *selector,
                                TypeExpectation::from_type(r#type),
                                body,
                            );
                        }
                    }
                    self.infer_statement(*case, body, return_type);
                }
            },
            Statement::For {
                initializer,
                condition,
                continuing_part,
                block,
            } => {
                if let Some(init) = initializer {
                    self.infer_statement(*init, body, return_type);
                }
                if let Some(cont) = continuing_part {
                    self.infer_statement(*cont, body, return_type);
                }

                if let Some(condition) = condition {
                    self.infer_expression_expect(
                        *condition,
                        TypeExpectation::from_type(self.bool_type()),
                        body,
                    );
                }

                self.infer_statement(*block, body, return_type);
            },
            Statement::Loop { body: loop_body } => {
                self.infer_statement(*loop_body, body, return_type);
            },
            Statement::Assert { expression } => {
                self.infer_expression_expect(
                    *expression,
                    TypeExpectation::from_type(self.bool_type()),
                    body,
                );
            },
            Statement::Discard | Statement::Break | Statement::Continue | Statement::Missing => {},
            Statement::Continuing { block } => self.infer_statement(*block, body, return_type),
            Statement::BreakIf { condition } => {
                self.infer_expression_expect(
                    *condition,
                    TypeExpectation::from_type(self.bool_type()),
                    body,
                );
            },
            Statement::FunctionCall { expression } => {
                self.infer_expression(*expression, body);
                // check must_use and report diagnostic here
            },
        }
    }

    /// Each such declaration must have an explicitly specified type or an initializer.
    /// Both a type and an initializer may be specified.
    /// Each such declaration determines the type for the associated data value, known as the effective-value-type for the declaration.
    /// The effective-value-type of the declaration is:
    /// - The declared type, if explicitly specified.
    /// - Otherwise, if the initializer expression has type T:
    ///   - For a const declaration, the effective-value-type is T itself.
    ///   - For a override, let, or var declaration, the effective-value-type is the concretization of T.
    ///
    /// Each kind of value or variable declaration may place additional constraints on the form of the initializer expression, if present, and on the effective-value-type.
    fn get_effective_value_type(
        &mut self,
        body: &Body,
        resolver: &Resolver<'db>,
        type_ref: Option<la_arena::Idx<hir_def::type_specifier::TypeSpecifier>>,
        initializer: Option<ExpressionId>,
    ) -> Type {
        let r#type = type_ref.map(|r#type| self.lower_type(r#type, resolver, body));
        let r#type =
            self.infer_initializer(body, initializer, r#type, AbstractHandling::Concretize);
        r#type.loaded(self.db).concretize(self.db)
    }

    fn infer_initializer(
        &mut self,
        store: &ExpressionStore,
        initializer: Option<ExpressionId>,
        r#type: Option<Type>,
        abstract_handling: AbstractHandling,
    ) -> Type {
        match (r#type, initializer) {
            (Some(r#type), Some(initializer)) => {
                self.infer_expression_expect(
                    initializer,
                    TypeExpectation::from_type(r#type),
                    store,
                );
                r#type
            },
            (Some(r#type), None) => r#type,
            (None, Some(initializer)) => {
                let r#type = self.infer_expression(initializer, store).loaded(self.db);
                if abstract_handling == AbstractHandling::Concretize {
                    r#type.concretize(self.db)
                } else {
                    r#type
                }
            },
            (None, None) => self.error_type(),
        }
    }

    fn expect_type_inner(
        &self,
        r#type: Type,
        expectation: TypeExpectationInner,
    ) -> Result<(), ()> {
        match expectation {
            TypeExpectationInner::Exact(expected_type) => {
                if expected_type.kind(self.db) == TypeKind::Error
                    || r#type.is_convertible_to(expected_type, self.db)
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
            TypeExpectationInner::IntegerScalar => {
                if let TypeKind::Scalar(
                    ScalarType::I32 | ScalarType::U32 | ScalarType::I64 | ScalarType::U64,
                ) = r#type.kind(self.db).unref(self.db).as_ref()
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
            TypeExpectationInner::IntegerIndex => {
                if let TypeKind::Scalar(
                    ScalarType::I32 | ScalarType::U32 | ScalarType::AbstractInt,
                ) = r#type.kind(self.db).unref(self.db).as_ref()
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
        }
    }

    fn infer_expression_expect(
        &mut self,
        expression: ExpressionId,
        expected: TypeExpectation,
        store: &ExpressionStore,
    ) -> Type {
        let r#type = self.infer_expression(expression, store);
        match expected {
            TypeExpectation::Type(expected_type) => {
                if !r#type.is_err(self.db)
                    && self.expect_type_inner(r#type, expected_type) != Ok(())
                {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression,
                            actual: r#type,
                            expected,
                        },
                    );
                }
            },
            TypeExpectation::Any => {},
        }
        r#type
    }

    #[expect(clippy::too_many_lines, reason = "match with many small cases")]
    fn infer_expression(
        &mut self,
        expression: ExpressionId,
        store: &ExpressionStore,
    ) -> Type {
        let r#type = match &store[expression] {
            Expression::Missing => self.error_type(), // this would be a parser error
            Expression::BinaryOperation {
                left_side,
                right_side,
                operation,
            } => self.infer_binary_op(expression, *left_side, *right_side, *operation, store),
            Expression::UnaryOperator {
                expression: inner_expression,
                operator,
            } => self.infer_unary_op(expression, *inner_expression, *operator, store),
            Expression::Field {
                expression: field_expression,
                name,
            } => self.infer_field_expression(expression, store, *field_expression, name),
            Expression::Call {
                ident_expression,
                arguments,
            } => {
                let arguments: Vec<_> = arguments
                    .iter()
                    .map(|&argument| {
                        (
                            argument,
                            self.infer_expression(argument, store).loaded(self.db),
                        )
                    })
                    .collect();
                self.infer_function_call(expression, ident_expression, &arguments, store)
            },
            Expression::Index { left_side, index } => {
                let left_side = self.infer_expression(*left_side, store);
                let left_kind = left_side.kind(self.db);
                let index_type = self.infer_expression(*index, store).loaded(self.db);
                let index_kind = index_type.kind(self.db);
                let index_inner = index_kind.unref(self.db);
                if !index_inner.is_index() {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression: *index,
                            expected: TypeExpectation::Type(TypeExpectationInner::IntegerIndex),
                            actual: index_type,
                        },
                    );
                }
                // The base may be a vector, matrix, or fixed-size array type, or a memory view to a vector, matrix, fixed-size array, or runtime-sized array type.
                match left_kind {
                    TypeKind::Reference(Reference {
                        address_space,
                        inner,
                        access_mode,
                    })
                    | TypeKind::Pointer(Pointer {
                        address_space,
                        inner,
                        access_mode,
                    }) if let TypeKind::Vector(vec) = inner.kind(self.db) => {
                        self.make_ref(vec.component_type, address_space, access_mode)
                    },
                    TypeKind::Vector(vec) => vec.component_type,
                    TypeKind::Reference(Reference {
                        address_space,
                        inner,
                        access_mode,
                    })
                    | TypeKind::Pointer(Pointer {
                        address_space,
                        inner,
                        access_mode,
                    }) if let TypeKind::Matrix(matrix_type) = inner.kind(self.db) => self.make_ref(
                        TypeKind::Vector(VectorType {
                            size: matrix_type.rows,
                            component_type: matrix_type.inner,
                        })
                        .intern(self.db),
                        address_space,
                        access_mode,
                    ),
                    TypeKind::Matrix(matrix_type) => TypeKind::Vector(VectorType {
                        size: matrix_type.rows,
                        component_type: matrix_type.inner,
                    })
                    .intern(self.db),
                    TypeKind::Reference(Reference {
                        address_space,
                        inner,
                        access_mode,
                    })
                    | TypeKind::Pointer(Pointer {
                        address_space,
                        inner,
                        access_mode,
                    }) if let TypeKind::Array(array) = inner.kind(self.db) => {
                        self.make_ref(array.inner, address_space, access_mode)
                    },
                    TypeKind::Array(array) => array.inner,
                    TypeKind::Reference(Reference {
                        address_space: _,
                        inner,
                        access_mode: _,
                    })
                    | TypeKind::Pointer(Pointer {
                        address_space: _,
                        inner,
                        access_mode: _,
                    }) if inner.kind(self.db) == TypeKind::Error => self.error_type(),
                    TypeKind::SwizzleView(SwizzleView {
                        address_space,
                        component_type,
                        vector_size: _,
                        index_list: _,
                    }) => {
                        // Swizzle views do not directly support indexing expressions.
                        // When an indexing expression clause appears after a swizzle view,
                        // the Swizzle View Load Rule is applied first to yield a vector value,
                        // and then the indexing expression is applied to that vector value.

                        // HOWEVER, we can shortcut that here since we know how that will go.
                        // Also, the spec implies that this returns a vector type rather than a reference to a vector.
                        self.make_ref(
                            component_type,
                            address_space,
                            // https://www.w3.org/TR/WGSL/#swizzle-view
                            // `p` is of type `ptr<AS,vecN<S>,read_write>`
                            AccessMode::ReadWrite,
                        )
                    },
                    TypeKind::Scalar(_)
                    | TypeKind::Atomic(_)
                    | TypeKind::Struct(_)
                    | TypeKind::BuiltinStruct(_)
                    | TypeKind::Texture(_)
                    | TypeKind::Sampler(_)
                    | TypeKind::RayQuery(_)
                    | TypeKind::AccelerationStructure(_)
                    | TypeKind::Reference(_)
                    | TypeKind::Pointer(_) => {
                        self.push_diagnostic(
                            store.store_source,
                            InferenceDiagnosticKind::ArrayAccessInvalidType {
                                expression,
                                r#type: left_side,
                            },
                        );
                        // nothing we can return here is useful
                        self.error_type()
                    },
                    // No need to create extra diagnostics for problems upstream
                    TypeKind::Error => self.error_type(),
                }
            },
            Expression::Literal(literal) => {
                use hir_def::expression::{BuiltinFloat, BuiltinInt, Literal};
                let type_kind = match literal {
                    Literal::Int(_, BuiltinInt::I32) => TypeKind::Scalar(ScalarType::I32),
                    Literal::Int(_, BuiltinInt::U32) => TypeKind::Scalar(ScalarType::U32),
                    Literal::Int(_, BuiltinInt::I64) => TypeKind::Scalar(ScalarType::I64),
                    Literal::Int(_, BuiltinInt::U64) => TypeKind::Scalar(ScalarType::U64),
                    Literal::Int(_, BuiltinInt::Abstract) => {
                        TypeKind::Scalar(ScalarType::AbstractInt)
                    },
                    Literal::Float(_, BuiltinFloat::F16) => TypeKind::Scalar(ScalarType::F16),
                    Literal::Float(_, BuiltinFloat::F32) => TypeKind::Scalar(ScalarType::F32),
                    Literal::Float(_, BuiltinFloat::Abstract) => {
                        TypeKind::Scalar(ScalarType::AbstractFloat)
                    },
                    Literal::Bool(_) => TypeKind::Scalar(ScalarType::Bool),
                };
                type_kind.intern(self.db)
            },
            Expression::IdentExpression(ident_expression) => {
                self.infer_identifier_expression(expression, ident_expression, store)
            },
        };
        self.set_expression_type(expression, r#type);
        r#type
    }

    fn infer_field_expression(
        &mut self,
        expression: ExpressionId,
        store: &ExpressionStore,
        field_expression: ExpressionId,
        name: &Name,
    ) -> Type {
        let expression_type = self.infer_expression(field_expression, store);
        if expression_type.is_err(self.db) {
            // the problem is upstream, so do not push a superfluous diagnostic
            // no more useful type to return here
            return self.error_type();
        }
        let (kind, ref_info) = match expression_type.kind(self.db) {
            TypeKind::Reference(Reference {
                address_space,
                inner,
                access_mode,
            })
            | TypeKind::Pointer(Pointer {
                address_space,
                inner,
                access_mode,
            }) => (inner.kind(self.db), Some((address_space, access_mode))),
            kind @ (TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::SwizzleView(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::BuiltinStruct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::RayQuery(_)
            | TypeKind::AccelerationStructure(_)
            | TypeKind::Sampler(_)) => (kind, None),
        };

        match kind {
            TypeKind::Struct(r#struct) => {
                let r#type = self.infer_struct_field_expression(
                    expression,
                    store,
                    field_expression,
                    name,
                    expression_type,
                    r#struct,
                );
                match ref_info {
                    Some((address_space, access_mode)) => {
                        self.make_ref(r#type, address_space, access_mode)
                    },
                    None => r#type,
                }
            },
            // there are no storable builtin structs
            TypeKind::BuiltinStruct(builtin_struct) => self.infer_builtin_struct_field_expression(
                store,
                field_expression,
                name,
                expression_type,
                builtin_struct,
            ),
            TypeKind::Vector(vector_type) => self.infer_vector_access_expression(
                store,
                field_expression,
                name,
                expression_type,
                &vector_type,
                ref_info,
            ),
            // swizzling a swizzle is allowed!
            TypeKind::SwizzleView(swizzle_view) => self.infer_swizzle_view_expression(
                store,
                field_expression,
                name,
                expression_type,
                &swizzle_view,
            ),
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Matrix(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::RayQuery(_)
            | TypeKind::AccelerationStructure(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::NoSuchField {
                        expression: field_expression,
                        name: name.clone(),
                        r#type: expression_type,
                    },
                );
                // no more useful type to return here
                self.error_type()
            },
        }
    }

    fn infer_function(
        &mut self,
        function: &FunctionDetails,
        arguments: &[(ExpressionId, Type)],
        store: &ExpressionStore,
        expression: ExpressionId,
    ) -> Type {
        if function.parameters.len() == arguments.len() {
            for (expected, (actual_expression, actual_type)) in
                function.parameters().zip(arguments.iter().copied())
            {
                if !actual_type.is_convertible_to(expected, self.db) {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression: actual_expression,
                            actual: actual_type,
                            expected: TypeExpectation::Type(TypeExpectationInner::Exact(expected)),
                        },
                    );
                }
            }
            // if the function being called does not have a return type, that is a full-on error to try to use it as an expression
            function.return_type.unwrap_or_else(|| self.error_type())
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                    expression,
                    n_expected: function.parameters.len(),
                    n_actual: arguments.len(),
                },
            );
            self.error_type()
        }
    }

    fn infer_unary_op(
        &mut self,
        expression: ExpressionId,
        operand: ExpressionId,
        operator: UnaryOperator,
        store: &ExpressionStore,
    ) -> Type {
        let operand_type = self.infer_expression(operand, store);
        if operand_type.is_err(self.db) {
            return self.error_type();
        }
        // Load rule does not apply to this specific operator because it has precondition `r: ref<AS,T,AM>`
        let expression_type = if operator == UnaryOperator::AddressOf {
            operand_type
        } else {
            operand_type.loaded(self.db)
        };
        match wgsl_types::builtin::type_unary_op(
            to_wgsl_unary_operator(operator),
            &self.converter.to_wgsl_types(expression_type),
        ) {
            Ok(r#type) => self.converter.from_wgsl_types(r#type),
            Err(error) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::WgslError {
                        expression,
                        message: error.to_string(),
                    },
                );
                self.error_type()
            },
        }
    }

    fn infer_binary_op(
        &mut self,
        expression: ExpressionId,
        left_side: ExpressionId,
        right_side: ExpressionId,
        operation: BinaryOperation,
        store: &ExpressionStore,
    ) -> Type {
        let left_type = self.infer_expression(left_side, store);
        let right_type = self.infer_expression(right_side, store);

        if left_type.is_err(self.db) || right_type.is_err(self.db) {
            // debug_assert!(
            //     !self.result.diagnostics.is_empty(),
            //     "there should already be a diagnostic"
            // );
            // no more useful type to return here
            return self.error_type();
        }
        match wgsl_types::builtin::type_binary_op(
            to_wgsl_binary_operator(operation),
            &self.converter.to_wgsl_types(left_type.loaded(self.db)),
            &self.converter.to_wgsl_types(right_type.loaded(self.db)),
        ) {
            Ok(r#type) => self.converter.from_wgsl_types(r#type),
            Err(error) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::WgslError {
                        expression,
                        message: error.to_string(),
                    },
                );
                self.error_type()
            },
        }
    }

    fn infer_identifier_expression(
        &mut self,
        expression: ExpressionId,
        ident_expression: &IdentExpression,
        store: &ExpressionStore,
    ) -> Type {
        let resolver = self.resolver_for_expression(expression);
        let mut context =
            TypeLoweringContext::new(self.db, resolver.as_ref().unwrap_or(&self.resolver), store);
        let lowered = match context.lower_expression(
            TypeContainer::Expression(expression),
            &ident_expression.path,
            &ident_expression.template_parameters,
        ) {
            Ok(lowered) => {
                self.push_lowering_diagnostics(context.diagnostics, store);
                lowered
            },
            Err(error) => {
                context.diagnostics.push(error);
                self.push_lowering_diagnostics(context.diagnostics, store);
                return self.error_type();
            },
        };
        match lowered {
            Lowered::GlobalConstant(id) => {
                InferenceResult::of(self.db, DefinitionWithBodyId::GlobalConstant(id)).return_type
            },
            Lowered::GlobalVariable(id) => {
                InferenceResult::of(self.db, DefinitionWithBodyId::GlobalVariable(id)).return_type
            },
            Lowered::Override(id) => {
                InferenceResult::of(self.db, DefinitionWithBodyId::Override(id)).return_type
            },
            Lowered::Local(id) => self.result.type_of_binding[id],
            Lowered::BuiltinDeclaration(_, value) => self.converter.from_wgsl_types(value.ty()),
            Lowered::Type(_)
            | Lowered::ConstructibleTypeGenerator(_)
            | Lowered::Function(_)
            | Lowered::BuiltinFunction(_, _)
            // | Lowered::BuiltinConstructor(_, _)
            | Lowered::Enumerant(_) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::UnexpectedLoweredKind {
                        expression,
                        expected: LoweredKind::Variable,
                        actual: lowered.kind(),
                        path: ident_expression.path.clone(),
                    },
                );
                self.error_type()
            },
        }
    }

    fn infer_vector_access_expression(
        &mut self,
        store: &ExpressionStore,
        field_expression: ExpressionId,
        name: &Name,
        expression_type: Type,
        vector_type: &VectorType,
        is_ref: Option<(AddressSpace, AccessMode)>,
    ) -> Type {
        let index_out_of_bounds = |index: VecIndex| index.as_u8() > vector_type.size.as_u8();
        let mut error = || {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoSuchField {
                    expression: field_expression,
                    name: name.clone(),
                    r#type: expression_type,
                },
            );
            self.error_type()
        };
        match IndexList::parse_name(name) {
            Err(
                crate::ty::ParseIndexListError::InvalidLetter
                | crate::ty::ParseIndexListError::MixingSwizzles
                | crate::ty::ParseIndexListError::MoreThanFour,
            ) => error(),
            Err(ParseIndexListError::One(index)) => {
                if index_out_of_bounds(index) {
                    error()
                } else if let Some((address_space, access_mode)) = is_ref {
                    self.make_ref(vector_type.component_type, address_space, access_mode)
                } else {
                    vector_type.component_type
                }
            },
            Ok(index_list) => {
                if index_list.iter().copied().any(index_out_of_bounds) {
                    error()
                } else if let Some((address_space, access_mode)) = is_ref {
                    if access_mode == AccessMode::ReadWrite {
                        TypeKind::SwizzleView(SwizzleView {
                            address_space,
                            component_type: vector_type.component_type,
                            vector_size: vector_type.size,
                            index_list,
                        })
                        .intern(self.db)
                    } else if let Some((address_space, access_mode)) = is_ref {
                        self.make_ref(
                            TypeKind::Vector(VectorType {
                                size: index_list.length,
                                component_type: vector_type.component_type,
                            })
                            .intern(self.db),
                            address_space,
                            access_mode,
                        )
                    } else {
                        TypeKind::Vector(VectorType {
                            size: index_list.length,
                            component_type: vector_type.component_type,
                        })
                        .intern(self.db)
                    }
                } else {
                    TypeKind::Vector(VectorType {
                        size: index_list.length,
                        component_type: vector_type.component_type,
                    })
                    .intern(self.db)
                }
            },
        }
    }

    fn infer_swizzle_view_expression(
        &mut self,
        store: &ExpressionStore,
        field_expression: ExpressionId,
        name: &Name,
        expression_type: Type,
        swizzle_view: &SwizzleView,
    ) -> Type {
        let index_out_of_bounds =
            |index: VecIndex| index.as_u8() > swizzle_view.vector_size.as_u8();
        let mut error = || {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoSuchField {
                    expression: field_expression,
                    name: name.clone(),
                    r#type: expression_type,
                },
            );
            self.error_type()
        };
        match IndexList::parse_name(name) {
            Err(
                crate::ty::ParseIndexListError::InvalidLetter
                | crate::ty::ParseIndexListError::MixingSwizzles
                | crate::ty::ParseIndexListError::MoreThanFour,
            ) => error(),
            Err(ParseIndexListError::One(index)) => {
                if index_out_of_bounds(index) {
                    return error();
                }
                self.make_ref(
                    swizzle_view.component_type,
                    swizzle_view.address_space,
                    AccessMode::ReadWrite,
                )
            },
            Ok(index_list) => {
                if index_list.iter().copied().any(index_out_of_bounds) {
                    return error();
                }
                TypeKind::SwizzleView(SwizzleView {
                    address_space: swizzle_view.address_space,
                    component_type: swizzle_view.component_type,
                    vector_size: u8::try_from(name.as_str().len())
                        .unwrap()
                        .try_into()
                        .unwrap(),
                    index_list,
                })
                .intern(self.db)
            },
        }
    }

    /// Lowers the field expression: `struct_value.name`.
    fn infer_struct_field_expression(
        &mut self,
        expression: ExpressionId,
        store: &ExpressionStore,
        field_expression: ExpressionId,
        name: &Name,
        expression_type: Type,
        r#struct: StructId,
    ) -> Type {
        let struct_data = StructSignature::of(self.db, r#struct);
        let field_types = &self.db.field_types(r#struct).0;
        if let Some(field) = struct_data.field(name) {
            self.set_field_resolution(expression, FieldId { r#struct, field });
            field_types[field]
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoSuchField {
                    expression: field_expression,
                    name: name.clone(),
                    r#type: expression_type,
                },
            );
            self.error_type()
        }
    }

    /// Lowers the field expression: `builtin_struct_value.name`.
    fn infer_builtin_struct_field_expression(
        &mut self,
        store: &ExpressionStore,
        field_expression: ExpressionId,
        name: &Name,
        expression_type: Type,
        builtin_struct: BuiltinStruct,
    ) -> Type {
        if let Some((_, field_type)) = builtin_struct
            .fields
            .into_iter()
            .find(|(field_name, _)| field_name.as_str() == name.as_str())
        {
            field_type
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoSuchField {
                    expression: field_expression,
                    name: name.clone(),
                    r#type: expression_type,
                },
            );
            self.error_type()
        }
    }

    /// Lowers the function call expression: `identifier(arguments)`.
    fn infer_function_call(
        &mut self,
        function_call_expression: ExpressionId,
        // https://www.w3.org/TR/WGSL/#callee
        callee: &IdentExpression,
        argument_value_expressions: &[(ExpressionId, Type)],
        store: &ExpressionStore,
    ) -> Type {
        let resolver = self
            .resolver_for_expression(function_call_expression)
            .unwrap_or_else(|| self.resolver.clone());
        let mut context = TypeLoweringContext::new(self.db, &resolver, store);
        let lowered = match context.lower_expression(
            TypeContainer::Expression(function_call_expression),
            &callee.path,
            &callee.template_parameters,
        ) {
            Ok(lowered) => {
                self.push_lowering_diagnostics(context.diagnostics, store);
                lowered
            },
            Err(error) => {
                context.diagnostics.push(error);
                self.push_lowering_diagnostics(context.diagnostics, store);
                return TypeKind::Error.intern(self.db);
            },
        };
        let argument_types = argument_value_expressions
            .iter()
            .map(|(_, r#type)| *r#type)
            .collect_vec();

        match lowered {
            Lowered::Type(r#type) => self.infer_type_constructor(
                store,
                function_call_expression,
                r#type,
                argument_value_expressions,
            ),
            Lowered::ConstructibleTypeGenerator(generator) => self.infer_type_generator(
                store,
                function_call_expression,
                generator,
                argument_value_expressions,
            ),
            // Lowered::BuiltinConstructor(name, template) => {
            //     if argument_types
            //         .iter()
            //         .any(|r#type| r#type.is_err(self.db))
            //     {
            //         // cancel inference if an error is already known
            //         debug_assert!(
            //             !self.result.diagnostics.is_empty(),
            //             "if an argument is an [error], then there should be a diagnostic already"
            //         );
            //         return self.error_type();
            //     }
            //     self.infer_builtin_constructor(expression, argument_types, store, template, name)
            // },
            Lowered::Function(id) => {
                let details = id.lookup(self.db);
                self.result
                    .call_resolutions
                    .insert(function_call_expression, ResolvedCall::Function(id));
                self.infer_function(
                    details,
                    argument_value_expressions,
                    store,
                    function_call_expression,
                )
            },
            Lowered::BuiltinFunction(name, template) => {
                if argument_types.iter().any(|r#type| r#type.is_err(self.db)) {
                    // debug_assert!(
                    //     !self.result.diagnostics().is_empty(),
                    //     "error instance should have a diagnostic associated with it already"
                    // );
                    return self.error_type();
                }
                self.infer_builtin_function(
                    function_call_expression,
                    &argument_types,
                    store,
                    template,
                    &name,
                )
            },
            // uncallable as `identifier()`
            Lowered::Enumerant(_)
            | Lowered::GlobalConstant(_)
            | Lowered::BuiltinDeclaration(_, _)
            | Lowered::GlobalVariable(_)
            | Lowered::Override(_)
            | Lowered::Local(_) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::UnexpectedLoweredKind {
                        expression: function_call_expression,
                        expected: LoweredKind::Function,
                        actual: lowered.kind(),
                        path: callee.path.clone(),
                    },
                );
                self.error_type()
            },
        }
    }

    // fn infer_builtin_constructor(
    //     &mut self,
    //     expression: ExpressionId,
    //     argument_types: Vec<Type>,
    //     store: &ExpressionStore,
    //     template_parameters: Option<TemplateParameters>,
    //     name: Name,
    // ) -> Type {
    //     let wgsl_arguments = self.converter.to_wt_vec(&argument_types);
    //     let Ok(template) = self.converter.to_maybe_vec_template(template_parameters) else {
    //         debug_assert!(
    //             !self.result.diagnostics().is_empty(),
    //             "error instance should have a diagnostic associated with it already"
    //         );
    //         return self.error_type();
    //     };
    //     if let Ok(value) =
    //         wgsl_types::builtin::type_ctor(name.as_str(), template.as_deref(), &wgsl_arguments)
    //     {
    //         self.converter.from_wgsl_types(value)
    //     } else {
    //         self.push_diagnostic(
    //             store.store_source,
    //             InferenceDiagnosticKind::NoBuiltinOverload {
    //                 expression,
    //                 name: Some(name),
    //                 parameters: argument_types,
    //             },
    //         );
    //         self.error_type()
    //     }
    // }

    fn infer_builtin_function(
        &mut self,
        expression: ExpressionId,
        argument_types: &[Type],
        store: &ExpressionStore,
        template_parameters: Option<TemplateParameters>,
        name: &Name,
    ) -> Type {
        let wgsl_arguments = self.converter.to_wt_vec(argument_types);
        let Ok(template) = self.converter.to_maybe_vec_template(template_parameters) else {
            // assert fails with something like `sqrt<&y>(1)`
            // debug_assert!(
            //     !self.result.diagnostics().is_empty(),
            //     "error instance should have a diagnostic associated with it already"
            // );
            return self.error_type();
        };
        let return_type = wgsl_types::builtin::type_builtin_fn(
            name.as_str(),
            template.as_deref(),
            &wgsl_arguments,
        );
        match return_type {
            Ok(Some(r#type)) => self.converter.from_wgsl_types(r#type),
            Ok(None) => self.error_type(), // just a function that doesn't have a return value
            Err(error) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::WgslError {
                        expression,
                        message: error.to_string(),
                    },
                );
                self.error_type()
            },
        }
    }

    /// Constructor for a type with a fully specified template.
    fn infer_type_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        r#type: Type,
        arguments: &[(ExpressionId, Type)],
    ) -> Type {
        if !r#type.is_constructible(self.db) && !r#type.is_err(self.db) {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NotConstructible { expression, r#type },
            );
            // continue with inference just to give me helpful diagnostics and types
        }
        match r#type.kind(self.db) {
            TypeKind::Scalar(scalar_type) => {
                self.infer_scalar_constructor(store, expression, r#type, arguments, scalar_type)
            },
            TypeKind::Array(array_type) => {
                self.infer_array_constructor(store, expression, r#type, arguments, &array_type)
            },
            TypeKind::Vector(vector_type) => {
                self.infer_vector_constructor(store, expression, r#type, arguments, &vector_type)
            },
            TypeKind::Matrix(matrix_type) => {
                self.infer_matrix_constructor(store, expression, r#type, arguments, &matrix_type)
            },
            TypeKind::Struct(struct_id) => {
                self.infer_struct_constructor(store, expression, r#type, arguments, struct_id)
            },
            TypeKind::BuiltinStruct(builtin_struct) => self.infer_builtin_struct_constructor(
                store,
                expression,
                r#type,
                arguments,
                &builtin_struct,
            ),

            // Never constructible
            TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Pointer(_)
            | TypeKind::Atomic(_)
            | TypeKind::RayQuery(_)
            | TypeKind::AccelerationStructure(_)
            | TypeKind::SwizzleView(_)
            | TypeKind::Reference(_) => {
                debug_assert!(
                    !self.result.diagnostics.is_empty(),
                    "there is already a diagnostic above"
                );
                r#type // doesn't hurt to be helpful
            },
            TypeKind::Error => {
                debug_assert!(
                    !self.result.diagnostics.is_empty(),
                    "there should already be a diagnostic if we have an error"
                );
                r#type
            },
        }
    }

    fn infer_array_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: la_arena::Idx<Expression>,
        r#type: Type,
        arguments: &[(ExpressionId, Type)],
        array_type: &ArrayType,
    ) -> Type {
        if arguments.is_empty() {
            return r#type;
        }
        // validate types
        for (argument_expression, argument_type) in arguments {
            if !argument_type.is_convertible_to(array_type.inner, self.db) {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::TypeMismatch {
                        expression: *argument_expression,
                        expected: TypeExpectation::Type(TypeExpectationInner::Exact(
                            array_type.inner,
                        )),
                        actual: *argument_type,
                    },
                );
            }
        }
        // validate arity
        #[expect(
            clippy::as_conversions,
            reason = "constructing an array with too many parameters is an error anyway"
        )]
        if let ArraySize::Constant(size) = array_type.size
            && arguments.len() != size.get() as usize
        {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                    expression,
                    n_expected: size.get() as usize,
                    n_actual: arguments.len(),
                },
            );
        }
        r#type
    }

    fn infer_vector_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: la_arena::Idx<Expression>,
        r#type: Type,
        arguments: &[(la_arena::Idx<Expression>, Type)],
        vec: &VectorType,
    ) -> Type {
        let template = &[TpltParam::Type(
            self.converter.to_wgsl_types(vec.component_type),
        )];
        let argument_types = arguments.iter().map(|(_, r#type)| *r#type).collect_vec();
        if argument_types.iter().any(|r#type| r#type.is_err(self.db)) {
            debug_assert!(
                !self.result.diagnostics.is_empty(),
                "an error type should have a diagnostic already"
            );
            return r#type;
        }
        let wgsl_arguments = self.converter.to_wt_vec(&argument_types);
        let construction_result =
            wgsl_types::builtin::type_ctor(vec.name(), Some(template), &wgsl_arguments);

        if construction_result.is_err() {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoConstructor {
                    expression,
                    r#type,
                    parameters: arguments.iter().map(|(_, r#type)| *r#type).collect(),
                },
            );
        }
        r#type
    }

    fn infer_matrix_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: la_arena::Idx<Expression>,
        r#type: Type,
        arguments: &[(la_arena::Idx<Expression>, Type)],
        matrix: &MatrixType,
    ) -> Type {
        // https://www.w3.org/TR/WGSL/#zero-value-builtin-function
        if arguments.is_empty() {
            return r#type;
        }
        let template = &[TpltParam::Type(self.converter.to_wgsl_types(matrix.inner))];
        let argument_types = arguments.iter().map(|(_, r#type)| *r#type).collect_vec();
        if argument_types.iter().any(|r#type| r#type.is_err(self.db)) {
            debug_assert!(
                !self.result.diagnostics.is_empty(),
                "an error type should have a diagnostic already"
            );
            return r#type;
        }
        let wgsl_arguments = self.converter.to_wt_vec(&argument_types);
        let construction_result =
            wgsl_types::builtin::type_ctor(matrix.name(), Some(template), &wgsl_arguments);
        if construction_result.is_err() {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoConstructor {
                    expression,
                    r#type,
                    parameters: argument_types,
                },
            );
        }
        r#type
    }

    /// Constructor for just a type name.
    fn infer_type_generator(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        generator: ConstructibleTypeGenerator,
        arguments: &[(ExpressionId, Type)],
    ) -> Type {
        match generator {
            ConstructibleTypeGenerator::Array(array) => {
                self.infer_array_generator(store, expression, arguments, &array)
            },
            ConstructibleTypeGenerator::Vector(vector) => {
                self.infer_vector_generator(store, expression, arguments, &vector)
            },
            ConstructibleTypeGenerator::Matrix(matrix) => {
                self.infer_matrix_generator(store, expression, arguments, &matrix)
            },
        }
    }

    fn infer_array_generator(
        &mut self,
        store: &ExpressionStore,
        expression: la_arena::Idx<Expression>,
        arguments: &[(ExpressionId, Type)],
        array_type: &ArrayType,
    ) -> Type {
        let incomplete_type = || TypeKind::Array(array_type.clone()).intern(self.db);
        let Some((_, mut first_argument_type)) = arguments.first().copied() else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoOverload {
                    expression,
                    name: array_type.name().into(),
                    parameters: vec![],
                },
            );
            return incomplete_type();
        };

        // all of the following arguments must be the same type as the first argument
        for (argument_expression, argument_type) in &arguments[1..] {
            if argument_type.is_convertible_to(first_argument_type, self.db) {
                // Everything is as intended
            } else if first_argument_type.is_convertible_to(*argument_type, self.db) {
                // Narrowing the expected type
                first_argument_type = *argument_type;
            } else {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::TypeMismatch {
                        expression: *argument_expression,
                        expected: TypeExpectation::Type(TypeExpectationInner::Exact(
                            first_argument_type,
                        )),
                        actual: *argument_type,
                    },
                );
            }
        }
        if let Ok(length) = u32::try_from(arguments.len())
            && let Ok(array_size) = NonZeroU32::try_from(length)
        {
            TypeKind::Array(ArrayType {
                inner: first_argument_type,
                binding_array: false,
                size: ArraySize::Constant(array_size),
            })
            .intern(self.db)
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                    expression,
                    #[expect(clippy::as_conversions, reason = "usize always holds a u32")]
                    n_expected: ArraySize::MAX.get() as usize,
                    n_actual: arguments.len(),
                },
            );
            TypeKind::Array(ArrayType {
                inner: first_argument_type,
                binding_array: false,
                size: ArraySize::Constant(ArraySize::MAX),
            })
            .intern(self.db)
        }
    }

    fn infer_vector_generator(
        &mut self,
        store: &ExpressionStore,
        expression: la_arena::Idx<Expression>,
        arguments: &[(ExpressionId, Type)],
        vector_type: &VectorType,
    ) -> Type {
        let incomplete_type = || TypeKind::Vector(vector_type.clone()).intern(self.db);
        // See note in WGSL reference:
        // Note: Zero-filled vectors of AbstractInt can be written as vec2(), vec3(), and vec4().
        // https://www.w3.org/TR/WGSL/#zero-value-builtin-function
        if arguments.is_empty() {
            return TypeKind::Vector(VectorType {
                size: vector_type.size,
                component_type: TypeKind::Scalar(ScalarType::AbstractInt).intern(self.db),
            })
            .intern(self.db);
        }
        let argument_types = arguments.iter().map(|(_, r#type)| *r#type).collect_vec();
        if argument_types.iter().any(|r#type| r#type.is_err(self.db)) {
            // debug_assert!(
            //     !self.result.diagnostics.is_empty(),
            //     "an error type should have a diagnostic already"
            // );
            return incomplete_type();
        }
        let wgsl_arguments = self.converter.to_wt_vec(&argument_types);
        if let Ok(inferred_type) =
            wgsl_types::builtin::type_ctor(vector_type.name(), None, &wgsl_arguments)
        {
            self.converter.from_wgsl_types(inferred_type)
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoOverload {
                    expression,
                    name: vector_type.name().into(),
                    parameters: argument_types,
                },
            );
            incomplete_type()
        }
    }

    fn infer_matrix_generator(
        &mut self,
        store: &ExpressionStore,
        expression: la_arena::Idx<Expression>,
        arguments: &[(ExpressionId, Type)],
        matrix_type: &MatrixType,
    ) -> Type {
        let incomplete_type = || TypeKind::Matrix(matrix_type.clone()).intern(self.db);
        // using the zero-value constructor is impossible because the template wasn't written
        if arguments.is_empty() {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoOverload {
                    expression,
                    name: matrix_type.name().into(),
                    parameters: vec![],
                },
            );
            return incomplete_type();
        }
        let name = matrix_type.name();
        let argument_types = arguments.iter().map(|(_, r#type)| *r#type).collect_vec();
        if argument_types.iter().any(|r#type| r#type.is_err(self.db)) {
            // debug_assert!(
            //     !self.result.diagnostics.is_empty(),
            //     "an error type should have a diagnostic already"
            // );
            return incomplete_type();
        }
        let wgsl_arguments = self.converter.to_wt_vec(&argument_types);
        if let Ok(inferred_type) = wgsl_types::builtin::type_ctor(name, None, &wgsl_arguments) {
            self.converter.from_wgsl_types(inferred_type)
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoOverload {
                    expression,
                    name: matrix_type.name().into(),
                    parameters: argument_types,
                },
            );
            incomplete_type()
        }
    }

    fn infer_scalar_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        r#type: Type,
        arguments: &[(ExpressionId, Type)],
        scalar_type: ScalarType,
    ) -> Type {
        // https://www.w3.org/TR/WGSL/#zero-value-builtin-function
        if arguments.is_empty() {
            return r#type;
        }
        let argument_types = arguments.iter().map(|(_, r#type)| *r#type).collect_vec();
        if argument_types.iter().any(|r#type| r#type.is_err(self.db)) {
            // debug_assert!(
            //     !self.result.diagnostics.is_empty(),
            //     "an error type should have a diagnostic already"
            // );
            return r#type;
        }
        let wgsl_arguments = self.converter.to_wt_vec(&argument_types);
        if let Ok(inferred_type) =
            wgsl_types::builtin::type_ctor(scalar_type.name(), None, &wgsl_arguments)
        {
            self.converter.from_wgsl_types(inferred_type)
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoConstructor {
                    expression,
                    r#type,
                    parameters: argument_types,
                },
            );
            r#type
        }
    }

    fn infer_struct_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        r#type: Type,
        arguments: &[(ExpressionId, Type)],
        struct_id: StructId,
    ) -> Type {
        // https://www.w3.org/TR/WGSL/#zero-value-builtin-function
        if arguments.is_empty() {
            return r#type;
        }
        let signature = StructSignature::of(self.db, struct_id);
        if arguments.len() != signature.fields.len() {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                    expression,
                    n_expected: signature.fields.len(),
                    n_actual: arguments.len(),
                },
            );
            return self.error_type();
        }
        let argument_types = arguments.iter().map(|(_, r#type)| *r#type).collect_vec();
        if argument_types.iter().any(|r#type| r#type.is_err(self.db)) {
            // debug_assert!(
            //     !self.result.diagnostics.is_empty(),
            //     "an error type should have a diagnostic already"
            // );
            return r#type;
        }

        let field_types = &self.db.field_types(struct_id).0;
        let mut has_errors = false;
        for ((_, field_type), (argument_expression, argument_type)) in
            field_types.iter().zip(arguments.iter())
        {
            if !argument_type.is_convertible_to(*field_type, self.db) {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::TypeMismatch {
                        expression: *argument_expression,
                        expected: TypeExpectation::from_type(*field_type),
                        actual: *argument_type,
                    },
                );
                has_errors = true;
            }
        }

        if has_errors {
            self.error_type()
        } else {
            r#type
        }
    }

    fn infer_builtin_struct_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        r#type: Type,
        arguments: &[(ExpressionId, Type)],
        builtin_struct: &BuiltinStruct,
    ) -> Type {
        // https://www.w3.org/TR/WGSL/#zero-value-builtin-function
        if arguments.is_empty() {
            return r#type;
        }
        if arguments.len() != builtin_struct.fields.len() {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                    expression,
                    n_expected: builtin_struct.fields.len(),
                    n_actual: arguments.len(),
                },
            );
            return self.error_type();
        }
        let argument_types = arguments.iter().map(|(_, r#type)| *r#type).collect_vec();
        if argument_types.iter().any(|r#type| r#type.is_err(self.db)) {
            // debug_assert!(
            //     !self.result.diagnostics.is_empty(),
            //     "an error type should have a diagnostic already"
            // );
            return r#type;
        }

        let mut has_errors = false;
        for ((_, field_type), (argument_expression, argument_type)) in
            builtin_struct.fields.iter().zip(arguments.iter())
        {
            if !argument_type.is_convertible_to(*field_type, self.db) {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::TypeMismatch {
                        expression: *argument_expression,
                        expected: TypeExpectation::from_type(*field_type),
                        actual: *argument_type,
                    },
                );
                has_errors = true;
            }
        }

        if has_errors {
            self.error_type()
        } else {
            r#type
        }
    }

    fn lower_type(
        &mut self,
        type_ref: TypeSpecifierId,
        resolver: &Resolver<'db>,
        store: &ExpressionStore,
    ) -> Type {
        let mut context = TypeLoweringContext::new(self.db, resolver, store);
        let r#type = context.lower_type(type_ref);
        self.push_lowering_diagnostics(context.diagnostics, store);
        r#type
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum AbstractHandling {
    Concretize,
    Abstract,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TypeExpectationInner {
    Exact(Type),
    IntegerScalar,
    IntegerIndex,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TypeExpectation {
    Type(TypeExpectationInner),
    Any,
}

impl TypeExpectation {
    const fn from_type(r#type: Type) -> Self {
        Self::Type(TypeExpectationInner::Exact(r#type))
    }
}

impl InferenceContext<'_> {
    fn make_ref(
        &self,
        r#type: Type,
        address_space: AddressSpace,
        access_mode: AccessMode,
    ) -> Type {
        debug_assert!(!matches!(
            r#type.kind(self.db),
            TypeKind::Reference(_) | TypeKind::Pointer(_)
        ));
        TypeKind::Reference(Reference {
            address_space,
            inner: r#type,
            access_mode,
        })
        .intern(self.db)
    }

    const fn error_type(&self) -> Type {
        self.result.error_type
    }

    fn bool_type(&self) -> Type {
        TypeKind::Scalar(ScalarType::Bool).intern(self.db)
    }
}
