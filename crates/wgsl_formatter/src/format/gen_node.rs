use std::collections::BTreeSet;

use parser::{SyntaxKind, SyntaxNode};
use syntax::AstNode as _;

use crate::format::{
    gen_assignment_statement::{
        gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement,
    },
    gen_attributes::{gen_attribute, gen_diagnostic_attribute},
    gen_diagnostic::{gen_diagnostic_control, gen_diagnostic_rule_name, gen_severity_control_name},
    gen_directive::{
        gen_diagnostic_directive, gen_enable_directive, gen_enable_extension_name,
        gen_language_extension_name, gen_requires_directive,
    },
    gen_expression::{
        gen_field_expression, gen_ident_expression, gen_index_expression, gen_infix_expression,
        gen_literal_expression, gen_parenthesis_expression, gen_prefix_expression,
    },
    gen_function::{
        gen_fn_parameter, gen_fn_parameters, gen_fn_return_type, gen_function_declaration,
    },
    gen_function_call::{gen_function_call, gen_function_call_arguments},
    gen_if_statement::{
        gen_if_statement, gen_if_statement_else_clause, gen_if_statement_else_if_clause,
        gen_if_statement_if_clause,
    },
    gen_path::gen_path,
    gen_source_file::gen_source_file,
    gen_statement::{
        gen_break_if_statement, gen_const_assert_statement, gen_continuing_statement,
        gen_for_statement, gen_function_call_statement, gen_increment_decrement_statement,
        gen_loop_statement, gen_return_statement, gen_while_statement,
    },
    gen_statement_compound::gen_compound_statement,
    gen_statement_import::gen_import_statement,
    gen_struct::{gen_struct_body, gen_struct_declaration, gen_struct_member},
    gen_switch_statement::{
        gen_switch_body, gen_switch_body_case, gen_switch_case_default_selector,
        gen_switch_case_selectors, gen_switch_statement,
    },
    gen_type_alias_declaration::gen_type_alias_declaration,
    gen_types::{gen_template_list, gen_type_specifier},
    gen_var_let_const_override_statement::{
        gen_const_declaration_statement, gen_let_declaration_statement,
        gen_override_declaration_statement, gen_var_declaration_statement,
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
        SyntaxKind::$ignored_first:ident $( | SyntaxKind::$ignored:ident)* => $ignored_result:expr
    }) => {{
        match $node.kind() {
            $(syntax::ast::SyntaxKind::$ast => {
                let $name = syntax::ast::$ast::cast($node.clone()).unwrap();
                $result
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
             SyntaxKind::ImportStatement(node) => gen_import_statement(&node),
             SyntaxKind::DiagnosticControl(node) => gen_diagnostic_control(&node),
             SyntaxKind::DiagnosticAttribute(node) => gen_diagnostic_attribute(&node),
             SyntaxKind::DiagnosticDirective(node) => gen_diagnostic_directive(&node),
             SyntaxKind::DiagnosticRuleName(node) => gen_diagnostic_rule_name(&node),
             SyntaxKind::SeverityControlName(node) => gen_severity_control_name(&node),

             -
             // TODO(MonaMayrhofer) all of these
             SyntaxKind::Name |
             SyntaxKind::BreakStatement |
             SyntaxKind::ContinueStatement |
             SyntaxKind::DiscardStatement |
             SyntaxKind::EmptyStatement |
             SyntaxKind::ForInitializer |
             SyntaxKind::ForCondition |
             SyntaxKind::ForContinuingPart |
             SyntaxKind::ImportPackageRelative |
             SyntaxKind::ImportSuperRelative |
             SyntaxKind::ImportItem |
             SyntaxKind::ImportPath |
             SyntaxKind::ImportCollection |
             SyntaxKind::LineEndingComment |
             SyntaxKind::BlockComment |
             SyntaxKind::OtherAttribute |
             SyntaxKind::AlignAttribute |
             SyntaxKind::BindingAttribute |
             SyntaxKind::BlendSrcAttribute |
             SyntaxKind::BuiltinAttribute |
             SyntaxKind::ConstantAttribute |
             SyntaxKind::GroupAttribute |
             SyntaxKind::IdAttribute |
             SyntaxKind::InterpolateAttribute |
             SyntaxKind::InvariantAttribute |
             SyntaxKind::LocationAttribute |
             SyntaxKind::MustUseAttribute |
             SyntaxKind::SizeAttribute |
             SyntaxKind::WorkgroupSizeAttribute |
             SyntaxKind::VertexAttribute |
             SyntaxKind::FragmentAttribute |
             SyntaxKind::ComputeAttribute |
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
             SyntaxKind::BuiltinValueName |
             SyntaxKind::InterpolateSamplingName |
             SyntaxKind::InterpolateTypeName |
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
