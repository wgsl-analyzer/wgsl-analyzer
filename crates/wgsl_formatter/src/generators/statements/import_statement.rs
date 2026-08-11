use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::SyntaxKind;
use rowan::SyntaxToken;
use syntax::{
    AstNode as _,
    ast::{self, ImportTree},
};

use crate::{
    ast_parse::{
        Chain, IgnoreBlankspace, IgnoreComma, NoTrivia, UntilSucceedingNewline, parse_end,
        parse_node_with, syntax_iter,
    },
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::NodeWithTriviaContent,
};

// TODO(MonaMayrhofer,post-1.0) Collapse imports
// import bevy:a;
// import bevy:b;
// import bevy:{a,b};
//
// This is best done using a ParsedImports struct with a parse_imports() function (similar to how comments are handled)
// then a ParsedImports::simplify that collapses them.
// Vec<Comment> are either attached to after import items (or a the end of an import collection {a, b, /*hi*/})

pub fn gen_import_package_relative(
    node: &ast::ImportPackageRelative
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(node.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(ast::SyntaxKind::Package)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(ast::SyntaxKind::ColonColon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("package"));
    formatted.push_sc(sc!("::"));
    Ok(formatted)
}
pub fn gen_import_super_relative(
    node: &ast::ImportSuperRelative
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(node.syntax());

    let mut items = Vec::new();

    loop {
        let item = parse_node_with(&mut syntax, IgnoreBlankspace)
            .expect_kind_optional(SyntaxKind::Super)?;

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(item);
        }
        if is_end {
            break;
        }

        let item =
            parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::ColonColon)?;

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    for item in items {
        formatted.extend(gen_node_with_trivia(&item)?);
    }
    Ok(formatted)
}
pub fn gen_import_item(node: &ast::ImportItem) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(node.syntax());
    let item_name = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?;
    let item_as = parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::As, &mut syntax);
    let item_alias = if item_as.is_some() {
        Some(parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?)
    } else {
        None
    };
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_name)?);
    if let Some(item_alias) = item_alias {
        formatted.request(Request::expect(RequestItem::Space));
        formatted.push_sc(sc!("as"));
        formatted.request(Request::expect(RequestItem::Space));
        formatted.extend(gen_node_with_trivia(&item_alias)?);
    }
    Ok(formatted)
}
pub fn gen_import_path(node: &ast::ImportPath) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(node.syntax());
    let item_name = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ColonColon)?;
    let item_path_rest = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_kind(SyntaxKind::ImportPath, &mut syntax);
    let item_collection_rest = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_kind(SyntaxKind::ImportCollection, &mut syntax);
    let item_item = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_kind(SyntaxKind::ImportItem, &mut syntax);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_node_with_trivia(&item_name)?);
    formatted.start_indent_before_requests();
    formatted.start_new_line_group();
    formatted.request(Request::empty().or_newline());
    formatted.push_sc(sc!("::"));
    formatted.finish_new_line_group();
    formatted.finish_indent_before_requests();

    if let Some(path) = item_path_rest {
        formatted.extend(gen_node_with_trivia(&path)?);
    }
    if let Some(collection) = item_collection_rest {
        formatted.extend(gen_node_with_trivia(&collection)?);
    }
    if let Some(item) = item_item {
        formatted.extend(gen_node_with_trivia(&item)?);
    }

    Ok(formatted)
}

pub struct CmpImportTree<'tree>(pub &'tree ImportTree);
impl PartialEq for CmpImportTree<'_> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for CmpImportTree<'_> {}
impl PartialOrd for CmpImportTree<'_> {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CmpImportTree<'_> {
    #[expect(clippy::min_ident_chars, reason = "Readable enough, keep it short")]
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        let a = self;
        let b = other;

        match (a.0, b.0) {
            (ImportTree::ImportItem(a), ImportTree::ImportItem(b)) => {
                let a = a.name().and_then(|name| name.ident_token());
                let b = b.name().and_then(|name| name.ident_token());
                let a = a.as_ref().map(SyntaxToken::text);
                let b = b.as_ref().map(SyntaxToken::text);
                a.cmp(&b)
            },
            (ImportTree::ImportPath(a_path), ImportTree::ImportPath(b_path)) => {
                let a = a_path.name().and_then(|name| name.ident_token());
                let b = b_path.name().and_then(|name| name.ident_token());
                let a = a.as_ref().map(SyntaxToken::text);
                let b = b.as_ref().map(SyntaxToken::text);
                match a.cmp(&b) {
                    std::cmp::Ordering::Equal => {
                        let a = a_path.item();
                        let a = a.as_ref().map(CmpImportTree);
                        let b = b_path.item();
                        let b = b.as_ref().map(CmpImportTree);
                        a.cmp(&b)
                    },
                    order @ (std::cmp::Ordering::Less | std::cmp::Ordering::Greater) => order,
                }
            },

            (ImportTree::ImportCollection(_), ImportTree::ImportCollection(_)) => {
                todo!()
            },

            (ImportTree::ImportItem(_), _) => std::cmp::Ordering::Less,
            #[expect(clippy::match_same_arms, reason = "Order of matches is important")]
            (_, ImportTree::ImportItem(_)) => std::cmp::Ordering::Greater,
            (ImportTree::ImportCollection(_), _) => std::cmp::Ordering::Greater,
            (_, ImportTree::ImportCollection(_)) => std::cmp::Ordering::Less,
        }
    }
}

pub fn gen_import_collection(
    node: &ast::ImportCollection
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(node.syntax());

    let mut items = Vec::new();

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceLeft)?;

    loop {
        let mut item = parse_node_with(
            &mut syntax,
            Chain(UntilSucceedingNewline, Chain(IgnoreBlankspace, IgnoreComma)),
        );

        // TODO This should be absorbed into the parse_node
        if item
            .kind()
            .is_some_and(|item| item == SyntaxKind::BraceRight)
        {
            let old_node = std::mem::replace(&mut item.node, NodeWithTriviaContent::End);
            syntax.put_back(old_node.into_option().unwrap()); //TODO
        }

        let is_end = item.is_end();
        if !item.is_whitespace() {
            let import_tree = item
                .content()
                .and_then(|node_or_token| match node_or_token {
                    rowan::NodeOrToken::Node(node) => Some(node),
                    rowan::NodeOrToken::Token(_) => None,
                })
                .and_then(ImportTree::cast);
            items.push((item, import_tree));
        }
        if is_end {
            break;
        }
    }

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceRight)?;

    parse_end(&mut syntax)?;

    items.sort_by(|(_, tree_a), (_, tree_b)| {
        let tree_a = tree_a.as_ref().map(CmpImportTree);
        let tree_b = tree_b.as_ref().map(CmpImportTree);
        tree_a.cmp(&tree_b)
    });

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut group = MultilineGroup::new(&mut formatted);
    group.push_sc(sc!("{"));

    group.start_indent();

    for (position, (item, _)) in items.iter().with_position() {
        group.extend(gen_node_with_trivia(item)?);
        if item.has_content() {
            group.request(Request::discourage(RequestItem::Space));
            if position != Position::Last && position != Position::Only {
                group.push_sc(sc!(","));
                group.request(Request::expect(RequestItem::Space).or_newline());
            }
        }
    }

    group.finish_indent();
    group.request(Request::discourage(RequestItem::Space));
    group.grouped_possible_newline();

    group.push_sc(sc!("}"));

    group.end();

    Ok(formatted)
}

pub fn gen_import_statement(node: &ast::ImportStatement) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(node.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(ast::SyntaxKind::Import)?;

    let item_package_relative = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_kind(SyntaxKind::ImportPackageRelative, &mut syntax);
    let item_super_relative = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_kind(SyntaxKind::ImportSuperRelative, &mut syntax);
    let item_entity =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<ast::ImportTree>()?;

    parse_node_with(&mut syntax, NoTrivia).expect_kind(ast::SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("import"));
    formatted.request(Request::expect(RequestItem::Space));

    if let Some(package_relative) = item_package_relative {
        formatted.extend(gen_node_with_trivia(&package_relative)?);
    }
    if let Some(super_relative) = item_super_relative {
        formatted.extend(gen_node_with_trivia(&super_relative)?);
    }
    formatted.extend(gen_node_with_trivia(&item_entity)?);

    if statement_needs_semicolon_policy(node.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }

    Ok(formatted)
}
