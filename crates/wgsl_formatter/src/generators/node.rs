use dprint_core_macros::sc;
use parser::{
    SyntaxKind::{self},
    SyntaxNode, SyntaxToken,
};
use rowan::NodeOrToken;
use syntax::{AstNode as _, ast};

use crate::{
    generators::{
        attributes::{
            gen_align_attribute, gen_attr_standard_with_args, gen_attribute, gen_attribute_list,
            gen_binding_attribute, gen_blend_src_attribute, gen_builtin_attribute,
            gen_builtin_value_name, gen_compute_attribute, gen_const_attribute,
            gen_diagnostic_attribute, gen_early_depth_test_mode, gen_elif_attribute,
            gen_else_attribute, gen_fragment_attribute, gen_group_attribute, gen_id_attribute,
            gen_if_attribute, gen_interpolate_attribute, gen_interpolate_sampling_name,
            gen_interpolate_type_name, gen_invariant_attribute, gen_location_attribute,
            gen_must_use_attribute, gen_other_attribute, gen_size_attribute, gen_vertex_attribute,
            gen_workgroup_size_attribute,
        },
        comments::{gen_comment, read_comment},
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
        global_compound_declaration::gen_global_compound_declaration,
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
        verbatim::gen_node_syntax_verbatim,
    },
    helpers::{gen_line_spacing, read_blankspace},
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::{FormatDocumentError, FormatDocumentResult},
    trivia::{NodeTriviaItem, NodeWithTrivia, NodeWithTriviaContent},
};

macro_rules! with_sc {
    ($text:expr) => {{
        let mut buffer = PrintItemBuffer::default();
        buffer.push_sc(sc!($text));
        Ok(buffer)
    }};
}

macro_rules! with_cast {
    ($generator:ident, $ast_type:ty, $node:ident) => {
        $generator(
            &$node
                .clone()
                .into_node()
                .and_then(<$ast_type>::cast)
                .expect("We just matched on the SyntaxKind"),
        )
    };
}

macro_rules! with_node {
    ($generator:ident, $node:ident) => {
        $generator(
            &($node
                .as_node()
                .expect("We just matched on the SyntaxKind")
                .clone()),
        )
    };
}

#[expect(
    clippy::too_many_lines,
    reason = "It does not make sense to split this up"
)]
#[rustfmt::skip]
pub fn gen_node(
    with_trivia: &NodeWithTrivia,
    node: &NodeOrToken<SyntaxNode, SyntaxToken>
) -> FormatDocumentResult<PrintItemBuffer> {
    match node.kind() {
        SyntaxKind::SourceFile => with_cast!(gen_source_file, ast::SourceFile, node),
        SyntaxKind::GlobalCompoundDeclaration => with_cast!(gen_global_compound_declaration, ast::GlobalCompoundDeclaration, node),
        SyntaxKind::FunctionDeclaration => with_cast!(gen_function_declaration, ast::FunctionDeclaration, node),
        SyntaxKind::TemplateList => with_cast!(gen_template_list, ast::TemplateList, node),
        SyntaxKind::FunctionParameters => with_cast!(gen_fn_parameters, ast::FunctionParameters, node),
        SyntaxKind::Parameter => with_cast!(gen_fn_parameter, ast::Parameter, node),
        SyntaxKind::ReturnType => with_cast!(gen_fn_return_type, ast::ReturnType, node),
        SyntaxKind::AssertStatement => with_cast!(gen_const_assert_statement, ast::AssertStatement, node),
        SyntaxKind::CompoundStatement => gen_compound_statement(
            with_trivia,
            &node.clone().into_node().and_then(<ast::CompoundStatement>::cast).expect("We just matched on the SyntaxKind"),
        ),
        SyntaxKind::AssignmentStatement => with_cast!(gen_assignment_statement, ast::AssignmentStatement, node),
        SyntaxKind::PhonyAssignmentStatement => with_cast!(gen_phony_assignment_statement, ast::PhonyAssignmentStatement, node),
        SyntaxKind::CompoundAssignmentStatement => with_cast!(gen_compound_assignment_statement, ast::CompoundAssignmentStatement, node),
        SyntaxKind::FunctionCallStatement => with_cast!(gen_function_call_statement, ast::FunctionCallStatement, node),
        SyntaxKind::BreakIfStatement => with_cast!(gen_break_if_statement, ast::BreakIfStatement, node),
        SyntaxKind::LoopStatement => with_cast!(gen_loop_statement, ast::LoopStatement, node),
        SyntaxKind::WhileStatement => with_cast!(gen_while_statement, ast::WhileStatement, node),
        SyntaxKind::IfStatement => with_cast!(gen_if_statement, ast::IfStatement, node),
        SyntaxKind::SwitchStatement => with_cast!(gen_switch_statement, ast::SwitchStatement, node),
        SyntaxKind::SwitchBody => with_cast!(gen_switch_body, ast::SwitchBody, node),
        SyntaxKind::SwitchBodyCase => with_cast!(gen_switch_body_case, ast::SwitchBodyCase, node),
        SyntaxKind::SwitchCaseSelectors => with_cast!(gen_switch_case_selectors, ast::SwitchCaseSelectors, node),
        SyntaxKind::SwitchDefaultSelector => with_cast!(gen_switch_case_default_selector, ast::SwitchDefaultSelector, node),
        SyntaxKind::IncrementDecrementStatement => with_cast!(gen_increment_decrement_statement, ast::IncrementDecrementStatement, node),
        SyntaxKind::IfClause => with_cast!(gen_if_statement_if_clause, ast::IfClause, node),
        SyntaxKind::ElseIfClause => with_cast!(gen_if_statement_else_if_clause, ast::ElseIfClause, node),
        SyntaxKind::ElseClause => with_cast!(gen_if_statement_else_clause, ast::ElseClause, node),
        SyntaxKind::ForStatement => with_cast!(gen_for_statement, ast::ForStatement, node),
        SyntaxKind::FieldExpression => with_cast!(gen_field_expression, ast::FieldExpression, node),
        SyntaxKind::FunctionCall => with_cast!(gen_function_call, ast::FunctionCall, node),
        SyntaxKind::Arguments => with_cast!(gen_function_call_arguments, ast::Arguments, node),
        SyntaxKind::IdentExpression => with_cast!(gen_ident_expression, ast::IdentExpression, node),
        SyntaxKind::Path => with_cast!(gen_path, ast::Path, node),
        SyntaxKind::IndexExpression => with_cast!(gen_index_expression, ast::IndexExpression, node),
        SyntaxKind::ReturnStatement => with_cast!(gen_return_statement, ast::ReturnStatement, node),
        SyntaxKind::InfixExpression => with_cast!(gen_infix_expression, ast::InfixExpression, node),
        SyntaxKind::PrefixExpression => with_cast!(gen_prefix_expression, ast::PrefixExpression, node),
        SyntaxKind::Literal => with_cast!(gen_literal_expression, ast::Literal, node),
        SyntaxKind::ParenthesisExpression => with_cast!(gen_parenthesis_expression, ast::ParenthesisExpression, node),
        SyntaxKind::TypeSpecifier => with_cast!(gen_type_specifier, ast::TypeSpecifier, node),
        SyntaxKind::Attribute => with_cast!(gen_attribute, ast::Attribute, node),
        SyntaxKind::StructDeclaration => with_cast!(gen_struct_declaration, ast::StructDeclaration, node),
        SyntaxKind::StructBody => with_cast!(gen_struct_body, ast::StructBody, node),
        SyntaxKind::StructMember => with_cast!(gen_struct_member, ast::StructMember, node),
        SyntaxKind::ConstantDeclaration => with_cast!(gen_const_declaration_statement, ast::ConstantDeclaration, node),
        SyntaxKind::VariableDeclaration => with_cast!(gen_var_declaration_statement, ast::VariableDeclaration, node),
        SyntaxKind::LetDeclaration => with_cast!(gen_let_declaration_statement, ast::LetDeclaration, node),
        SyntaxKind::OverrideDeclaration => with_cast!(gen_override_declaration_statement, ast::OverrideDeclaration, node),
        SyntaxKind::ContinuingStatement => with_cast!(gen_continuing_statement, ast::ContinuingStatement, node),
        SyntaxKind::TypeAliasDeclaration => with_cast!(gen_type_alias_declaration, ast::TypeAliasDeclaration, node),
        SyntaxKind::EnableDirective => with_cast!(gen_enable_directive, ast::EnableDirective, node),
        SyntaxKind::EnableExtensionName => with_cast!(gen_enable_extension_name, ast::EnableExtensionName, node),
        SyntaxKind::RequiresDirective => with_cast!(gen_requires_directive, ast::RequiresDirective, node),
        SyntaxKind::LanguageExtensionName => with_cast!(gen_language_extension_name, ast::LanguageExtensionName, node),
        SyntaxKind::ImportStatement => with_cast!(gen_import_statement, ast::ImportStatement, node),
        SyntaxKind::DiagnosticControl => with_cast!(gen_diagnostic_control, ast::DiagnosticControl, node),
        SyntaxKind::DiagnosticAttribute => with_cast!(gen_diagnostic_attribute, ast::DiagnosticAttribute, node),
        SyntaxKind::DiagnosticDirective => with_cast!(gen_diagnostic_directive, ast::DiagnosticDirective, node),
        SyntaxKind::DiagnosticRuleName => with_cast!(gen_diagnostic_rule_name, ast::DiagnosticRuleName, node),
        SyntaxKind::SeverityControlName => with_cast!(gen_severity_control_name, ast::SeverityControlName, node),
        SyntaxKind::InterpolateSamplingName => with_cast!(gen_interpolate_sampling_name, ast::InterpolateSamplingName, node),
        SyntaxKind::InterpolateTypeName => with_cast!(gen_interpolate_type_name, ast::InterpolateTypeName, node),
        SyntaxKind::ImportPackageRelative => with_cast!(gen_import_package_relative, ast::ImportPackageRelative, node),
        SyntaxKind::ImportSuperRelative => with_cast!(gen_import_super_relative, ast::ImportSuperRelative, node),
        SyntaxKind::ImportItem => with_cast!(gen_import_item, ast::ImportItem, node),
        SyntaxKind::ImportPath => with_cast!(gen_import_path, ast::ImportPath, node),
        SyntaxKind::ImportCollection => with_cast!(gen_import_collection, ast::ImportCollection, node),
        SyntaxKind::Name => with_cast!(gen_name, ast::Name, node),
        SyntaxKind::OtherAttribute => with_cast!(gen_other_attribute, ast::OtherAttribute, node),
        SyntaxKind::AlignAttribute => with_cast!(gen_align_attribute, ast::AlignAttribute, node),
        SyntaxKind::BindingAttribute => with_cast!(gen_binding_attribute, ast::BindingAttribute, node),
        SyntaxKind::BlendSrcAttribute => with_cast!(gen_blend_src_attribute, ast::BlendSrcAttribute, node),
        SyntaxKind::BuiltinAttribute => with_cast!(gen_builtin_attribute, ast::BuiltinAttribute, node),
        SyntaxKind::ConstantAttribute => with_cast!(gen_const_attribute, ast::ConstantAttribute, node),
        SyntaxKind::GroupAttribute => with_cast!(gen_group_attribute, ast::GroupAttribute, node),
        SyntaxKind::IdAttribute => with_cast!(gen_id_attribute, ast::IdAttribute, node),
        SyntaxKind::InterpolateAttribute => with_cast!(gen_interpolate_attribute, ast::InterpolateAttribute, node),
        SyntaxKind::InvariantAttribute => with_cast!(gen_invariant_attribute, ast::InvariantAttribute, node),
        SyntaxKind::LocationAttribute => with_cast!(gen_location_attribute, ast::LocationAttribute, node),
        SyntaxKind::MustUseAttribute => with_cast!(gen_must_use_attribute, ast::MustUseAttribute, node),
        SyntaxKind::IfAttribute => with_cast!(gen_if_attribute, ast::IfAttribute, node),
        SyntaxKind::ElifAttribute => with_cast!(gen_elif_attribute, ast::ElifAttribute, node),
        SyntaxKind::ElseAttribute => with_cast!(gen_else_attribute, ast::ElseAttribute, node),
        SyntaxKind::EarlyDepthTestAttribute => {
            gen_attr_standard_with_args(
                &node
                    .as_node()
                    .expect("We just matched on the SyntaxKind")
                    .clone(),
                SyntaxKind::EarlyDepthTest,
                dprint_core_macros::sc!("early_depth_test")
            )
        }
        SyntaxKind::AttributeList => with_cast!(gen_attribute_list, ast::AttributeList, node),
        SyntaxKind::SizeAttribute => with_cast!(gen_size_attribute, ast::SizeAttribute, node),
        SyntaxKind::WorkgroupSizeAttribute => with_cast!(gen_workgroup_size_attribute, ast::WorkgroupSizeAttribute, node),
        SyntaxKind::VertexAttribute => with_cast!(gen_vertex_attribute, ast::VertexAttribute, node),
        SyntaxKind::FragmentAttribute => with_cast!(gen_fragment_attribute, ast::FragmentAttribute, node),
        SyntaxKind::ComputeAttribute => with_cast!(gen_compute_attribute, ast::ComputeAttribute, node),
        SyntaxKind::BuiltinValueName => with_cast!(gen_builtin_value_name, ast::BuiltinValueName, node),
        SyntaxKind::BreakStatement => with_cast!(gen_break_statement, ast::BreakStatement, node),
        SyntaxKind::ContinueStatement => with_cast!(gen_continue_statement, ast::ContinueStatement, node),
        SyntaxKind::DiscardStatement => with_cast!(gen_discard_statement, ast::DiscardStatement, node),
        SyntaxKind::EarlyDepthTestMode => with_node!(gen_early_depth_test_mode, node),
        SyntaxKind::ForInitializer => with_node!(gen_for_statement_initializer, node),
        SyntaxKind::ForCondition => with_node!(gen_for_statement_condition, node),
        SyntaxKind::ForContinuingPart => with_node!(gen_for_statement_continuing_part, node),

        SyntaxKind::EmptyStatement => {
            // We remove lonely semicolons
            Ok(PrintItemBuffer::default())
        },

        SyntaxKind::Alias => with_sc!("alias"),
        SyntaxKind::Break => with_sc!("break"),
        SyntaxKind::Case => with_sc!("case"),
        SyntaxKind::Const => with_sc!("const"),
        SyntaxKind::ConstantAssert => with_sc!("assert"),
        SyntaxKind::Continue => with_sc!("continue"),
        SyntaxKind::Continuing => with_sc!("continuing"),
        SyntaxKind::Default => with_sc!("default"),
        SyntaxKind::Diagnostic => with_sc!("diagnostic"),
        SyntaxKind::Discard => with_sc!("discard"),
        SyntaxKind::Align => with_sc!("align"),
        SyntaxKind::Builtin => with_sc!("builtin"),
        SyntaxKind::Binding => with_sc!("binding"),
        SyntaxKind::BlendSrc => with_sc!("blend_src"),
        SyntaxKind::Group => with_sc!("group"),
        SyntaxKind::Id => with_sc!("id"),
        SyntaxKind::Interpolate => with_sc!("interpolate"),
        SyntaxKind::Invariant => with_sc!("invariant"),
        SyntaxKind::Location => with_sc!("location"),
        SyntaxKind::MustUse => with_sc!("must_use"),
        SyntaxKind::Size => with_sc!("size"),
        SyntaxKind::WorkgroupSize => with_sc!("workgroup_size"),
        SyntaxKind::Vertex => with_sc!("vertex"),
        SyntaxKind::Fragment => with_sc!("fragment"),
        SyntaxKind::Compute => with_sc!("compute"),
        SyntaxKind::Perspective => with_sc!("perspective"),
        SyntaxKind::EarlyDepthTest => with_sc!("early_depth_test"),
        SyntaxKind::LessEqual => with_sc!("less_equal"),
        SyntaxKind::GreaterEqual => with_sc!("greater_equal"),
        SyntaxKind::Force => with_sc!("force"),
        SyntaxKind::Unchanged => with_sc!("unchanged"),
        SyntaxKind::Linear => with_sc!("linear"),
        SyntaxKind::Flat => with_sc!("flat"),
        SyntaxKind::Center => with_sc!("center"),
        SyntaxKind::Centroid => with_sc!("centroid"),
        SyntaxKind::Sample => with_sc!("sample"),
        SyntaxKind::First => with_sc!("first"),
        SyntaxKind::Either => with_sc!("either"),
        SyntaxKind::Else => with_sc!("else"),
        SyntaxKind::Enable => with_sc!("enable"),
        SyntaxKind::False => with_sc!("false"),
        SyntaxKind::Fn => with_sc!("fn"),
        SyntaxKind::For => with_sc!("for"),
        SyntaxKind::If => with_sc!("if"),
        SyntaxKind::Let => with_sc!("let"),
        SyntaxKind::Loop => with_sc!("loop"),
        SyntaxKind::Override => with_sc!("override"),
        SyntaxKind::Requires => with_sc!("requires"),
        SyntaxKind::Return => with_sc!("return"),
        SyntaxKind::Struct => with_sc!("struct"),
        SyntaxKind::Switch => with_sc!("switch"),
        SyntaxKind::True => with_sc!("true"),
        SyntaxKind::Var => with_sc!("var"),
        SyntaxKind::While => with_sc!("while"),
        SyntaxKind::And => with_sc!("&"),
        SyntaxKind::AndAnd => with_sc!("&&"),
        SyntaxKind::Arrow => with_sc!("=>"),
        SyntaxKind::AttributeOperator => with_sc!("@"),
        SyntaxKind::ForwardSlash => with_sc!("/"),
        SyntaxKind::Bang => with_sc!("!"),
        SyntaxKind::BracketLeft => with_sc!("["),
        SyntaxKind::BracketRight => with_sc!("]"),
        SyntaxKind::BraceLeft => with_sc!("{"),
        SyntaxKind::BraceRight => with_sc!("}"),
        SyntaxKind::Colon => with_sc!(":"),
        SyntaxKind::ColonColon => with_sc!("::"),
        SyntaxKind::Comma => with_sc!(","),
        SyntaxKind::Equal => with_sc!("="),
        SyntaxKind::EqualEqual => with_sc!("=="),
        SyntaxKind::NotEqual => with_sc!("!="),
        SyntaxKind::GreaterThan |
        SyntaxKind::TemplateEnd => with_sc!(">"),
        SyntaxKind::GreaterThanEqual => with_sc!(">="),
        SyntaxKind::LessThanEqual => with_sc!("<="),
        SyntaxKind::LessThan |
        SyntaxKind::TemplateStart => with_sc!("<"),
        SyntaxKind::Modulo => with_sc!("%"),
        SyntaxKind::Minus => with_sc!("-"),
        SyntaxKind::MinusMinus => with_sc!("--"),
        SyntaxKind::Period => with_sc!("."),
        SyntaxKind::Plus => with_sc!("+"),
        SyntaxKind::PlusPlus => with_sc!("++"),
        SyntaxKind::Or => with_sc!("|"),
        SyntaxKind::OrOr => with_sc!("||"),
        SyntaxKind::ParenthesisLeft => with_sc!("("),
        SyntaxKind::ParenthesisRight => with_sc!(")"),
        SyntaxKind::Semicolon => with_sc!(";"),
        SyntaxKind::Star => with_sc!("*"),
        SyntaxKind::Tilde => with_sc!("~"),
        SyntaxKind::Underscore => with_sc!("_"),
        SyntaxKind::Xor => with_sc!("^"),
        SyntaxKind::Import => with_sc!("import"),
        SyntaxKind::Package => with_sc!("package"),
        SyntaxKind::Super => with_sc!("super"),
        SyntaxKind::As => with_sc!("as"),
        SyntaxKind::Elif => with_sc!("elif"),
        SyntaxKind::PlusEqual => with_sc!("+="),
        SyntaxKind::MinusEqual => with_sc!("-="),
        SyntaxKind::TimesEqual => with_sc!("*="),
        SyntaxKind::DivisionEqual => with_sc!("/="),
        SyntaxKind::ModuloEqual => with_sc!("%="),
        SyntaxKind::AndEqual => with_sc!("&="),
        SyntaxKind::OrEqual => with_sc!("|="),
        SyntaxKind::XorEqual => with_sc!("^="),
        SyntaxKind::ShiftRightEqual => with_sc!(">>="),
        SyntaxKind::ShiftLeftEqual => with_sc!("<<="),
        SyntaxKind::ShiftLeft => with_sc!("<<"),
        SyntaxKind::ShiftRight => with_sc!(">>"),

        SyntaxKind::LineEndingComment |
        SyntaxKind::BlockComment => {
            let comment = read_comment(node).expect("We just matched on the SyntaxKind");
            Ok(gen_comment(&comment))
        },
        SyntaxKind::Blankspace => {
            let blankspace = read_blankspace(node).expect("We just matched on the SyntaxKind");
            gen_line_spacing(&blankspace)
        },
        SyntaxKind::Identifier
        | SyntaxKind::FloatLiteral
        | SyntaxKind::IntLiteral
        | SyntaxKind::StringLiteral => {
            let mut formatted = PrintItemBuffer::default();
            formatted.push_string(
                node
                    .as_token()
                    .ok_or_else(|| FormatDocumentError::UnexpectedNodeOrToken { received: Some(node.clone()) })?
                    .text()
                    .to_owned()
            );
            Ok(formatted)
        },


        SyntaxKind::Error |
        SyntaxKind::Reserved => {
            // We don't won't format a source that contains errors, or reserved keywords
            Err(FormatDocumentError::UnsupportedNodeOrToken  { received: node.clone() })
        },

        _ => {
            Err(FormatDocumentError::UnsupportedNodeOrToken  { received: node.clone() })
        },

    }
}

pub fn gen_node_preceding_trivia(node: &NodeWithTrivia) -> FormatDocumentResult<PrintItemBuffer> {
    if node.format {
        gen_node_trivia(&node.preceding_trivia)
    } else {
        gen_node_trivia_verbatim(&node.preceding_trivia)
    }
}

pub fn gen_node_succeeding_trivia(node: &NodeWithTrivia) -> FormatDocumentResult<PrintItemBuffer> {
    // We intentionally ignore `node.format` on succeeding trivia
    // Yes this is not a beautiful solution, but it turns a lot of
    // head-scratcher problems into nonproblems.
    // Users don't usually expect succeeding_trivia to be "part" of the item that they ignored -
    // while preserving trailing double spaces after an ignored item might be correct, its unexpected.
    // So... this solution is fine for now.
    gen_node_trivia(&node.succeeding_trivia)
}

pub fn gen_node_content(node: &NodeWithTrivia) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();

    if let NodeWithTriviaContent::Content(content) = &node.node {
        if node.format {
            formatted.extend(gen_node(node, content)?);
        } else {
            formatted.extend(gen_node_syntax_verbatim(content)?);
        }
    }

    Ok(formatted)
}

pub fn gen_node_trivia_verbatim(
    trivia: &[NodeTriviaItem]
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();
    for trivia in trivia {
        match trivia {
            NodeTriviaItem::LineSpacing(content) => {
                formatted.extend(gen_node_syntax_verbatim(&content.syntax())?);
            },
            NodeTriviaItem::Comment(content) | NodeTriviaItem::NewlinedComment(content) => {
                formatted.extend(gen_node_syntax_verbatim(&content.syntax())?);
            },
            NodeTriviaItem::AttributeList(content) => {
                formatted.extend(gen_node_syntax_verbatim(&NodeOrToken::Node(
                    content.syntax().clone(),
                ))?);
            },
            NodeTriviaItem::Discarded(content) => {
                formatted.extend(gen_node_syntax_verbatim(&content)?);
            },
        }
    }

    Ok(formatted)
}

pub fn gen_node_trivia(trivia: &[NodeTriviaItem]) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();
    for trivia in trivia {
        match trivia {
            NodeTriviaItem::LineSpacing(content) => {
                formatted.extend(gen_line_spacing(content)?);
            },
            NodeTriviaItem::Comment(content) => {
                formatted.extend(gen_comment(content));
            },
            NodeTriviaItem::NewlinedComment(content) => {
                formatted.extend(gen_comment(content));
                formatted.request(Request::expect(RequestItem::LineBreak));
            },
            NodeTriviaItem::AttributeList(content) => {
                formatted.extend(gen_attribute_list(content)?);
            },
            NodeTriviaItem::Discarded(_) => {},
        }
    }

    Ok(formatted)
}

pub fn gen_node_with_trivia(node: &NodeWithTrivia) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_node_preceding_trivia(node)?);
    formatted.extend(gen_node_content(node)?);
    formatted.extend(gen_node_succeeding_trivia(node)?);

    Ok(formatted)
}

pub fn gen_node_with_trivia_no_newlines(
    trivia: &NodeWithTrivia
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();
    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.extend(gen_node_with_trivia(trivia)?);
    formatted.request(Request::discourage(RequestItem::LineBreak));
    Ok(formatted)
}
