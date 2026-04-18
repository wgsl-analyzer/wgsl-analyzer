use std::collections::BTreeSet;

use parser::{SyntaxKind, SyntaxNode};
use syntax::AstNode as _;

use crate::{
    generators::{
        attributes::{
            gen_align_attribute, gen_attribute, gen_binding_attribute, gen_blend_src_attribute,
            gen_builtin_attribute, gen_builtin_value_name, gen_compute_attribute,
            gen_const_attribute, gen_diagnostic_attribute, gen_fragment_attribute,
            gen_group_attribute, gen_id_attribute, gen_interpolate_attribute,
            gen_interpolate_sampling_name, gen_interpolate_type_name, gen_invariant_attribute,
            gen_location_attribute, gen_must_use_attribute, gen_other_attribute,
            gen_size_attribute, gen_vertex_attribute, gen_workgroup_size_attribute,
        },
        diagnostic_directive::{
            gen_diagnostic_control, gen_diagnostic_rule_name, gen_severity_control_name,
        },
        directives::{
            gen_diagnostic_directive, gen_enable_directive, gen_enable_extension_name,
            gen_language_extension_name, gen_requires_directive,
        },
        expressions::{
            field_expression::gen_field_expression, ident_expression::gen_ident_expression,
            index_expression::gen_index_expression, infix_expression::gen_infix_expression,
            literal_expression::gen_literal_expression,
            parenthesis_expression::gen_parenthesis_expression,
            prefix_expression::gen_prefix_expression,
        },
        function_declaration::{
            gen_fn_parameter, gen_fn_parameters, gen_fn_return_type, gen_function_declaration,
        },
        name::gen_name,
        path::gen_path,
        source_file::gen_source_file,
        statements::{
            assignment_statement::{
                gen_assignment_statement, gen_compound_assignment_statement,
                gen_phony_assignment_statement,
            },
            break_if_statement::gen_break_if_statement,
            break_statement::gen_break_statement,
            compound_statement::gen_compound_statement,
            const_assert_statement::gen_const_assert_statement,
            continue_statement::gen_continue_statement,
            continuing_statement::gen_continuing_statement,
            discard_statement::gen_discard_statement,
            for_statement::{
                gen_for_statement, gen_for_statement_condition, gen_for_statement_continuing_part,
                gen_for_statement_initializer,
            },
            function_call_statement::{
                gen_function_call, gen_function_call_arguments, gen_function_call_statement,
            },
            if_statement::{
                gen_if_statement, gen_if_statement_else_clause, gen_if_statement_else_if_clause,
                gen_if_statement_if_clause,
            },
            import_statement::{
                gen_import_collection, gen_import_item, gen_import_package_relative,
                gen_import_path, gen_import_statement, gen_import_super_relative,
            },
            increment_decrement_statement::gen_increment_decrement_statement,
            loop_statement::gen_loop_statement,
            return_statement::gen_return_statement,
            switch_statement::{
                gen_switch_body, gen_switch_body_case, gen_switch_case_default_selector,
                gen_switch_case_selectors, gen_switch_statement,
            },
            var_let_const_override_statement::{
                gen_const_declaration_statement, gen_let_declaration_statement,
                gen_override_declaration_statement, gen_var_declaration_statement,
            },
            while_statement::gen_while_statement,
        },
        struct_declaration::{gen_struct_body, gen_struct_declaration, gen_struct_member},
        type_alias_declaration::gen_type_alias_declaration,
        types::{gen_template_list, gen_type_specifier},
    },
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_node_no_newlines(node: &SyntaxNode) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::new();
    formatted.request(Request::Unconditional {
        expected: BTreeSet::new(),
        discouraged: BTreeSet::from([RequestItem::LineBreak]),
        forced: BTreeSet::new(),
        suggest_linebreak: false,
    });
    formatted.extend(gen_node(node)?);
    formatted.request(Request::Unconditional {
        expected: BTreeSet::new(),
        discouraged: BTreeSet::from([RequestItem::LineBreak]),
        forced: BTreeSet::new(),
        suggest_linebreak: false,
    });
    Ok(formatted)
}

macro_rules! match_ast_exhaustive {
    (match $node:ident {
        $( SyntaxKind::$ast:ident($name:ident) => $result:expr, )*
        -
        $( SyntaxKind::$special_ast:ident($special_name:ident as SyntaxNode) => $special_result:expr, )*
        -
        SyntaxKind::$ignored_first:ident $( | SyntaxKind::$ignored:ident)* => $ignored_result:expr
    }) => {{
        match $node.kind() {
            $(syntax::ast::SyntaxKind::$ast => {
                let $name = syntax::ast::$ast::cast($node.clone()).unwrap();
                $result
            },)*
            $(syntax::ast::SyntaxKind::$special_ast => {
                let $special_name = $node.clone();
                $special_result
            },)*
            syntax::ast::SyntaxKind::$ignored_first $(| syntax::ast::SyntaxKind::$ignored)*  => $ignored_result
        }
    }};
}

#[expect(
    clippy::too_many_lines,
    reason = "It does not make sense to split this up"
)]
pub fn gen_node(node: &SyntaxNode) -> FormatDocumentResult<PrintItemBuffer> {
    // TODO Move the semicolon logic into the gen_* functions. They should not take "include semicolon" but instead some
    // `context` and then invoke needs_semicolon(context);
    fn needs_semicolon(parent_node: Option<SyntaxNode>) -> bool {
        match parent_node {
            Some(parent_node) => !matches!(
                parent_node.kind(),
                SyntaxKind::ForInitializer
                    | SyntaxKind::ForCondition
                    | SyntaxKind::ForContinuingPart
            ),
            None => false,
        }
    }

    match_ast_exhaustive! {
        match node {
             SyntaxKind::SourceFile(node) => gen_source_file(&node),
             SyntaxKind::FunctionDeclaration(node) => gen_function_declaration(&node),
             SyntaxKind::TemplateList(node) => gen_template_list(&node),
             SyntaxKind::FunctionParameters(node) => gen_fn_parameters(&node),
             SyntaxKind::Parameter(node) => gen_fn_parameter(&node),
             SyntaxKind::ReturnType(node) => gen_fn_return_type(&node),
             SyntaxKind::AssertStatement(node) => gen_const_assert_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::CompoundStatement(node) => gen_compound_statement(&node),
             SyntaxKind::AssignmentStatement(node) => gen_assignment_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::PhonyAssignmentStatement(node) => gen_phony_assignment_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::CompoundAssignmentStatement(node) => gen_compound_assignment_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::FunctionCallStatement(node) => gen_function_call_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::BreakIfStatement(node) => gen_break_if_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::LoopStatement(node) => gen_loop_statement(&node),
             SyntaxKind::WhileStatement(node) => gen_while_statement(&node),
             SyntaxKind::IfStatement(node) => gen_if_statement(&node),
             SyntaxKind::SwitchStatement(node) => gen_switch_statement(&node),
             SyntaxKind::SwitchBody(node) => gen_switch_body(&node),
             SyntaxKind::SwitchBodyCase(node) => gen_switch_body_case(&node),
             SyntaxKind::SwitchCaseSelectors(node) => gen_switch_case_selectors(&node),
             SyntaxKind::SwitchDefaultSelector(node) => gen_switch_case_default_selector(&node),
             SyntaxKind::IncrementDecrementStatement(node) => gen_increment_decrement_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::IfClause(node) => gen_if_statement_if_clause(&node),
             SyntaxKind::ElseIfClause(node) => gen_if_statement_else_if_clause(&node),
             SyntaxKind::ElseClause(node) => gen_if_statement_else_clause(&node),
             SyntaxKind::ForStatement(node) => gen_for_statement(&node),
             SyntaxKind::FieldExpression(node) => gen_field_expression(&node),
             SyntaxKind::FunctionCall(node) => gen_function_call(&node),
             SyntaxKind::Arguments(node) => gen_function_call_arguments(&node),
             SyntaxKind::IdentExpression(node) => gen_ident_expression(&node),
             SyntaxKind::Path(node) => gen_path(&node),
             SyntaxKind::IndexExpression(node) => gen_index_expression(&node),
             SyntaxKind::ReturnStatement(node) => gen_return_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::InfixExpression(node) => gen_infix_expression(&node),
             SyntaxKind::PrefixExpression(node) => gen_prefix_expression(&node),
             SyntaxKind::Literal(node) => gen_literal_expression(&node),
             SyntaxKind::ParenthesisExpression(node) => gen_parenthesis_expression(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::TypeSpecifier(node) => gen_type_specifier(&node),
             SyntaxKind::Attribute(node) => gen_attribute(&node),
             SyntaxKind::StructDeclaration(node) => gen_struct_declaration(&node),
             SyntaxKind::StructBody(node) => gen_struct_body(&node),
             SyntaxKind::StructMember(node) => gen_struct_member(&node),
             SyntaxKind::ConstantDeclaration(node) => gen_const_declaration_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::VariableDeclaration(node) => gen_var_declaration_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::LetDeclaration(node) => gen_let_declaration_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::OverrideDeclaration(node) => gen_override_declaration_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::ContinuingStatement(node) => gen_continuing_statement(&node),
             SyntaxKind::TypeAliasDeclaration(node) => gen_type_alias_declaration(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::EnableDirective(node) => gen_enable_directive(&node),
             SyntaxKind::EnableExtensionName(node) => gen_enable_extension_name(&node),
             SyntaxKind::RequiresDirective(node) => gen_requires_directive(&node),
             SyntaxKind::LanguageExtensionName(node) => gen_language_extension_name(&node),
             SyntaxKind::ImportStatement(node) => gen_import_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::DiagnosticControl(node) => gen_diagnostic_control(&node),
             SyntaxKind::DiagnosticAttribute(node) => gen_diagnostic_attribute(&node),
             SyntaxKind::DiagnosticDirective(node) => gen_diagnostic_directive(&node),
             SyntaxKind::DiagnosticRuleName(node) => gen_diagnostic_rule_name(&node),
             SyntaxKind::SeverityControlName(node) => gen_severity_control_name(&node),
             SyntaxKind::InterpolateSamplingName(node) => gen_interpolate_sampling_name(&node),
             SyntaxKind::InterpolateTypeName(node) => gen_interpolate_type_name(&node),
             SyntaxKind::ImportPackageRelative(node) => gen_import_package_relative(&node),
             SyntaxKind::ImportSuperRelative(node) => gen_import_super_relative(&node),
             SyntaxKind::ImportItem(node) => gen_import_item(&node),
             SyntaxKind::ImportPath(node) => gen_import_path(&node),
             SyntaxKind::ImportCollection(node) => gen_import_collection(&node),
             SyntaxKind::Name(node) => gen_name(&node),
             SyntaxKind::OtherAttribute(node) => gen_other_attribute(&node),
             SyntaxKind::AlignAttribute(node) => gen_align_attribute(&node),
             SyntaxKind::BindingAttribute(node) => gen_binding_attribute(&node),
             SyntaxKind::BlendSrcAttribute(node) => gen_blend_src_attribute(&node),
             SyntaxKind::BuiltinAttribute(node) => gen_builtin_attribute(&node),
             SyntaxKind::ConstantAttribute(node) => gen_const_attribute(&node),
             SyntaxKind::GroupAttribute(node) => gen_group_attribute(&node),
             SyntaxKind::IdAttribute(node) => gen_id_attribute(&node),
             SyntaxKind::InterpolateAttribute(node) => gen_interpolate_attribute(&node),
             SyntaxKind::InvariantAttribute(node) => gen_invariant_attribute(&node),
             SyntaxKind::LocationAttribute(node) => gen_location_attribute(&node),
             SyntaxKind::MustUseAttribute(node) => gen_must_use_attribute(&node),
             SyntaxKind::SizeAttribute(node) => gen_size_attribute(&node),
             SyntaxKind::WorkgroupSizeAttribute(node) => gen_workgroup_size_attribute(&node),
             SyntaxKind::VertexAttribute(node) => gen_vertex_attribute(&node),
             SyntaxKind::FragmentAttribute(node) => gen_fragment_attribute(&node),
             SyntaxKind::ComputeAttribute(node) => gen_compute_attribute(&node),
             SyntaxKind::BuiltinValueName(node) => gen_builtin_value_name(&node),
             SyntaxKind::BreakStatement(node) => gen_break_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::ContinueStatement(node) => gen_continue_statement(&node, needs_semicolon(node.syntax().parent())),
             SyntaxKind::DiscardStatement(node) => gen_discard_statement(&node, needs_semicolon(node.syntax().parent())),

             -

             SyntaxKind::ForInitializer(node as SyntaxNode) => gen_for_statement_initializer(&node),
             SyntaxKind::ForCondition(node as SyntaxNode) => gen_for_statement_condition(&node),
             SyntaxKind::ForContinuingPart(node as SyntaxNode) => gen_for_statement_continuing_part(&node),

             -

             // TODO(MonaMayrhofer) all of these
             SyntaxKind::EmptyStatement |
             SyntaxKind::LineEndingComment |
             SyntaxKind::BlockComment |
             // Tokens
             SyntaxKind::Blankspace |
             SyntaxKind::Identifier |
             SyntaxKind::FloatLiteral |
             SyntaxKind::IntLiteral |
             SyntaxKind::StringLiteral |
             SyntaxKind::Alias |
             SyntaxKind::Break |
             SyntaxKind::Case |
             SyntaxKind::Const |
             SyntaxKind::ConstantAssert |
             SyntaxKind::Continue |
             SyntaxKind::Continuing |
             SyntaxKind::Default |
             SyntaxKind::Diagnostic |
             SyntaxKind::Discard |
             SyntaxKind::Align |
             SyntaxKind::Builtin |
             SyntaxKind::Binding |
             SyntaxKind::BlendSrc |
             SyntaxKind::Group |
             SyntaxKind::Id |
             SyntaxKind::Interpolate |
             SyntaxKind::Invariant |
             SyntaxKind::Location |
             SyntaxKind::MustUse |
             SyntaxKind::Size |
             SyntaxKind::WorkgroupSize |
             SyntaxKind::Vertex |
             SyntaxKind::Fragment |
             SyntaxKind::Compute |
             SyntaxKind::Perspective |
             SyntaxKind::Linear |
             SyntaxKind::Flat |
             SyntaxKind::Center |
             SyntaxKind::Centroid |
             SyntaxKind::Sample |
             SyntaxKind::First |
             SyntaxKind::Either |
             SyntaxKind::Else |
             SyntaxKind::Enable |
             SyntaxKind::False |
             SyntaxKind::Fn |
             SyntaxKind::For |
             SyntaxKind::If |
             SyntaxKind::Let |
             SyntaxKind::Loop |
             SyntaxKind::Override |
             SyntaxKind::Requires |
             SyntaxKind::Return |
             SyntaxKind::Struct |
             SyntaxKind::Switch |
             SyntaxKind::True |
             SyntaxKind::Var |
             SyntaxKind::While |
             SyntaxKind::And |
             SyntaxKind::AndAnd |
             SyntaxKind::Arrow |
             SyntaxKind::AttributeOperator |
             SyntaxKind::ForwardSlash |
             SyntaxKind::Bang |
             SyntaxKind::BracketLeft |
             SyntaxKind::BracketRight |
             SyntaxKind::BraceLeft |
             SyntaxKind::BraceRight |
             SyntaxKind::Colon |
             SyntaxKind::ColonColon |
             SyntaxKind::Comma |
             SyntaxKind::Equal |
             SyntaxKind::EqualEqual |
             SyntaxKind::NotEqual |
             SyntaxKind::GreaterThan |
             SyntaxKind::GreaterThanEqual |
             SyntaxKind::LessThan |
             SyntaxKind::LessThanEqual |
             SyntaxKind::Modulo |
             SyntaxKind::Minus |
             SyntaxKind::MinusMinus |
             SyntaxKind::Period |
             SyntaxKind::Plus |
             SyntaxKind::PlusPlus |
             SyntaxKind::Or |
             SyntaxKind::OrOr |
             SyntaxKind::ParenthesisLeft |
             SyntaxKind::ParenthesisRight |
             SyntaxKind::Semicolon |
             SyntaxKind::Star |
             SyntaxKind::Tilde |
             SyntaxKind::Underscore |
             SyntaxKind::Xor |
             SyntaxKind::Import |
             SyntaxKind::Package |
             SyntaxKind::Super |
             SyntaxKind::As |
             SyntaxKind::PlusEqual |
             SyntaxKind::MinusEqual |
             SyntaxKind::TimesEqual |
             SyntaxKind::DivisionEqual |
             SyntaxKind::ModuloEqual |
             SyntaxKind::AndEqual |
             SyntaxKind::OrEqual |
             SyntaxKind::XorEqual |
             SyntaxKind::ShiftRightEqual |
             SyntaxKind::ShiftLeftEqual |
             SyntaxKind::ShiftLeft |
             SyntaxKind::ShiftRight |
             SyntaxKind::TemplateStart |
             SyntaxKind::TemplateEnd |
             SyntaxKind::EOF |
             SyntaxKind::EOFAttribute |
             SyntaxKind::EOFExpression |
             SyntaxKind::EOFStatement |
             SyntaxKind::EOFTypeSpecifier |
             SyntaxKind::TOMBSTONE |
             SyntaxKind::Error => {
                 todo!("gen_node not implemented for {:?}", node.kind())
             }
        }
    }
}
