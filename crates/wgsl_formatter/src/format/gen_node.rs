use std::collections::BTreeSet;

use parser::SyntaxNode;
use syntax::{
    AstNode as _,
    ast::{
        Arguments, AssertStatement, AssignmentStatement, Attribute, CompoundAssignmentStatement,
        CompoundStatement, ConstantDeclaration, Expression, FieldExpression, FunctionCall,
        FunctionDeclaration, FunctionParameters, IdentExpression, IfStatement, IndexExpression,
        InfixExpression, LetDeclaration, Literal, OverrideDeclaration, Parameter,
        ParenthesisExpression, Path, PhonyAssignmentStatement, PrefixExpression, ReturnType,
        SourceFile, Statement, StructDeclaration, SwitchBody, SwitchBodyCase, SwitchCaseSelector,
        SwitchCaseSelectors, SwitchDefaultSelector, SwitchStatement, TemplateList,
        TypeAliasDeclaration, TypeSpecifier, VariableDeclaration,
    },
};

use crate::format::{
    gen_assignment_statement::{
        gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement,
    },
    gen_attributes::gen_attribute,
    gen_expression::{
        gen_expression, gen_field_expression, gen_ident_expression, gen_index_expression,
        gen_infix_expression, gen_literal_expression, gen_parenthesis_expression,
        gen_prefix_expression,
    },
    gen_function::{
        gen_fn_parameter, gen_fn_parameters, gen_fn_return_type, gen_function_declaration,
    },
    gen_function_call::{gen_function_call, gen_function_call_arguments},
    gen_if_statement::gen_if_statement,
    gen_path::gen_path,
    gen_source_file::gen_source_file,
    gen_statement::{gen_const_assert_statement, gen_statement_maybe_semicolon},
    gen_statement_compound::gen_compound_statement,
    gen_struct::gen_struct_declaration,
    gen_switch_statement::{
        gen_switch_body, gen_switch_body_case, gen_switch_case_default_selector,
        gen_switch_case_selector, gen_switch_case_selectors, gen_switch_statement,
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
    formatted.request_request(Request::Unconditional {
        expected: BTreeSet::new(),
        discouraged: BTreeSet::from([RequestItem::LineBreak]),
        forced: BTreeSet::new(),
    });
    formatted.extend(gen_node(node)?);
    formatted.request_request(Request::Unconditional {
        expected: BTreeSet::new(),
        discouraged: BTreeSet::from([RequestItem::LineBreak]),
        forced: BTreeSet::new(),
    });
    Ok(formatted)
}
pub fn gen_node(node: &SyntaxNode) -> FormatDocumentResult<PrintItemBuffer> {
    // TODO These clones are all unnecessary.
    // TODO This is very brittle, someone can easily write a gen_* function and not register it here... maybe some proc macro magic?

    // TODO Move the semicolon logic into the gen_* functions. They should not take "include semicolon" but instead some
    // `context` and then invoke needs_semicolon(context);
    fn needs_semicolon(parent_node: Option<SyntaxNode>) -> bool {
        match parent_node {
            Some(parent_node) => match parent_node.kind() {
                _ => true,
            },
            None => false,
        }
    }

    if let Some(source_file) = SourceFile::cast(node.clone()) {
        gen_source_file(&source_file)
    } else if let Some(node) = CompoundStatement::cast(node.clone()) {
        gen_compound_statement(&node)
    } else if let Some(node) = AssignmentStatement::cast(node.clone()) {
        gen_assignment_statement(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = Attribute::cast(node.clone()) {
        gen_attribute(&node)
    } else if let Some(node) = CompoundAssignmentStatement::cast(node.clone()) {
        gen_compound_assignment_statement(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = AssertStatement::cast(node.clone()) {
        gen_const_assert_statement(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = ConstantDeclaration::cast(node.clone()) {
        gen_const_declaration_statement(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = Expression::cast(node.clone()) {
        gen_expression(&node, false)
    } else if let Some(node) = FieldExpression::cast(node.clone()) {
        gen_field_expression(&node)
    } else if let Some(node) = Parameter::cast(node.clone()) {
        gen_fn_parameter(&node)
    } else if let Some(node) = FunctionParameters::cast(node.clone()) {
        gen_fn_parameters(&node)
    } else if let Some(node) = ReturnType::cast(node.clone()) {
        gen_fn_return_type(&node)
    } else if let Some(node) = FunctionCall::cast(node.clone()) {
        gen_function_call(&node)
    } else if let Some(node) = Arguments::cast(node.clone()) {
        gen_function_call_arguments(&node)
    } else if let Some(node) = FunctionDeclaration::cast(node.clone()) {
        gen_function_declaration(&node)
    } else if let Some(node) = IdentExpression::cast(node.clone()) {
        gen_ident_expression(&node)
    } else if let Some(node) = IfStatement::cast(node.clone()) {
        gen_if_statement(&node)
    } else if let Some(node) = IndexExpression::cast(node.clone()) {
        gen_index_expression(&node)
    } else if let Some(node) = InfixExpression::cast(node.clone()) {
        gen_infix_expression(&node)
    } else if let Some(node) = LetDeclaration::cast(node.clone()) {
        gen_let_declaration_statement(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = Literal::cast(node.clone()) {
        gen_literal_expression(&node)
    } else if let Some(node) = OverrideDeclaration::cast(node.clone()) {
        gen_override_declaration_statement(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = ParenthesisExpression::cast(node.clone()) {
        gen_parenthesis_expression(&node, false)
    } else if let Some(node) = Path::cast(node.clone()) {
        gen_path(&node)
    } else if let Some(node) = PhonyAssignmentStatement::cast(node.clone()) {
        gen_phony_assignment_statement(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = PrefixExpression::cast(node.clone()) {
        gen_prefix_expression(&node)
    } else if let Some(node) = Statement::cast(node.clone()) {
        gen_statement_maybe_semicolon(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = StructDeclaration::cast(node.clone()) {
        gen_struct_declaration(&node)
    } else if let Some(node) = SwitchBody::cast(node.clone()) {
        gen_switch_body(&node)
    } else if let Some(node) = SwitchBodyCase::cast(node.clone()) {
        gen_switch_body_case(&node)
    } else if let Some(node) = SwitchDefaultSelector::cast(node.clone()) {
        gen_switch_case_default_selector(&node)
    } else if let Some(node) = SwitchCaseSelector::cast(node.clone()) {
        gen_switch_case_selector(&node)
    } else if let Some(node) = SwitchCaseSelectors::cast(node.clone()) {
        gen_switch_case_selectors(&node)
    } else if let Some(node) = SwitchStatement::cast(node.clone()) {
        gen_switch_statement(&node)
    } else if let Some(node) = TemplateList::cast(node.clone()) {
        gen_template_list(&node)
    } else if let Some(node) = TypeAliasDeclaration::cast(node.clone()) {
        gen_type_alias_declaration(&node, needs_semicolon(node.syntax().parent()))
    } else if let Some(node) = TypeSpecifier::cast(node.clone()) {
        gen_type_specifier(&node)
    } else if let Some(node) = VariableDeclaration::cast(node.clone()) {
        gen_var_declaration_statement(&node, needs_semicolon(node.syntax().parent()))
    } else {
        todo!("Implement gen_node for {:?}", node.kind());
    }
}
