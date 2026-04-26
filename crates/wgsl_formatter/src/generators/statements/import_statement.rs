use dprint_core_macros::sc;
use itertools::{Itertools as _, Position, put_back};
use parser::SyntaxKind;
use rowan::SyntaxToken;
use syntax::{
    AstNode as _,
    ast::{self, ImportCollection, ImportItem, ImportPath, ImportTree},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_node_optional, parse_token, parse_token_optional},
    generators::{
        comments::{
            Comment, gen_comment, gen_comments, parse_comment_optional,
            parse_many_comments_and_blankspace,
        },
        name::gen_name,
    },
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

// TODO(MonaMayrhofer) Collapse imports
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
    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_token(&mut syntax, ast::SyntaxKind::Package)?;
    parse_token(&mut syntax, ast::SyntaxKind::ColonColon)?;
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
    let mut syntax = put_back(node.syntax().children_with_tokens());

    enum SuperRelativeItem {
        Super,
        Comment(Comment),
    }

    let mut items = Vec::new();

    #[expect(clippy::redundant_pattern_matching, reason = "Looks neater")]
    loop {
        if let Some(_) = parse_token_optional(&mut syntax, SyntaxKind::Super) {
            items.push(SuperRelativeItem::Super);
        } else if let Some(_) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // We ignore blankspace
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            items.push(SuperRelativeItem::Comment(comment));
        } else {
            break;
        }
        parse_token_optional(&mut syntax, ast::SyntaxKind::ColonColon);
    }

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    for item in items {
        match item {
            SuperRelativeItem::Super => {
                formatted.push_sc(sc!("super"));
                formatted.push_sc(sc!("::"));
            },
            SuperRelativeItem::Comment(comment) => {
                formatted.extend(gen_comment(&comment));
            },
        }
    }
    Ok(formatted)
}
pub fn gen_import_item(node: &ast::ImportItem) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(node.syntax().children_with_tokens());
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_alias = if parse_token_optional(&mut syntax, SyntaxKind::As).is_some() {
        let item_comments_after_as = parse_many_comments_and_blankspace(&mut syntax)?;
        let item_name = parse_node::<ast::Name>(&mut syntax)?;
        Some((item_comments_after_as, item_name))
    } else {
        None
    };
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_name(&item_name)?);
    formatted.extend(gen_comments(&item_comments_after_name));
    if let Some((item_comments_after_as, item_alias)) = item_alias {
        formatted.expect(RequestItem::Space);
        formatted.push_sc(sc!("as"));
        formatted.expect(RequestItem::Space);
        formatted.extend(gen_comments(&item_comments_after_as));
        formatted.extend(gen_name(&item_alias)?);
    }
    Ok(formatted)
}
pub fn gen_import_path(node: &ast::ImportPath) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(node.syntax().children_with_tokens());
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ColonColon)?;
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_path_rest = parse_node_optional::<ImportPath>(&mut syntax);
    let item_collection_rest = parse_node_optional::<ImportCollection>(&mut syntax);
    let item_item = parse_node_optional::<ImportItem>(&mut syntax);
    let item_comments_after_rest = parse_many_comments_and_blankspace(&mut syntax)?;

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_name(&item_name)?);
    formatted.extend(gen_comments(&item_comments_after_name));
    formatted.start_indent();
    formatted.start_new_line_group();
    formatted.request(Request::empty().or_newline());
    formatted.push_sc(sc!("::"));
    formatted.finish_new_line_group();
    formatted.finish_indent();

    formatted.extend(gen_comments(&item_comments_after_colon));

    if let Some(path) = item_path_rest {
        formatted.extend(gen_import_path(&path)?);
    }
    if let Some(collection) = item_collection_rest {
        formatted.extend(gen_import_collection(&collection)?);
    }
    if let Some(item) = item_item {
        formatted.extend(gen_import_item(&item)?);
    }

    formatted.extend(gen_comments(&item_comments_after_rest));

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
    let mut syntax = put_back(node.syntax().children_with_tokens());

    let mut items = Vec::new();

    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;

    loop {
        let before = parse_many_comments_and_blankspace(&mut syntax)?;
        // This also allows ImportCollection directly inside ImportCollection, but
        // its no problems to be more general. It does make the code simpler.
        if let Some(item) = parse_node_optional::<ImportTree>(&mut syntax) {
            let after = parse_many_comments_and_blankspace(&mut syntax)?;
            items.push((before, Some(item), after));
        }

        if parse_token_optional(&mut syntax, SyntaxKind::Comma).is_none() {
            break;
        }
    }

    parse_token(&mut syntax, SyntaxKind::BraceRight)?;

    parse_end(&mut syntax)?;

    items.sort_by(|(_, tree_a, _), (_, tree_b, _)| {
        let tree_a = tree_a.as_ref().map(CmpImportTree);
        let tree_b = tree_b.as_ref().map(CmpImportTree);
        tree_a.cmp(&tree_b)
    });

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut group = MultilineGroup::new(&mut formatted);
    group.push_sc(sc!("{"));

    group.start_indent();

    for (position, (before, item, after)) in items.iter().with_position() {
        group.extend(gen_comments(before));
        if let Some(item) = item {
            match item {
                ImportTree::ImportPath(path) => group.extend(gen_import_path(path)?),
                ImportTree::ImportItem(item) => group.extend(gen_import_item(item)?),
                // This case will never happen but it makes the code simpler to just use ImportTree here
                ImportTree::ImportCollection(collection) => {
                    group.extend(gen_import_collection(collection)?);
                },
            }
        }
        group.extend(gen_comments(after));
        group.discourage(RequestItem::Space);

        if position != Position::Last && position != Position::Only {
            group.push_sc(sc!(","));
            group.request(Request::expect(RequestItem::Space).or_newline());
        }
    }

    group.finish_indent();
    group.grouped_possible_newline();

    group.push_sc(sc!("}"));

    group.end();

    Ok(formatted)
}

pub fn gen_import_statement(
    node: &ast::ImportStatement,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_token(&mut syntax, ast::SyntaxKind::Import)?;
    let item_comments_after_import = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_package_relative = parse_node_optional::<ast::ImportPackageRelative>(&mut syntax);
    let item_super_relative = parse_node_optional::<ast::ImportSuperRelative>(&mut syntax);
    let item_path = parse_node_optional::<ast::ImportPath>(&mut syntax);
    let item_collection = parse_node_optional::<ast::ImportCollection>(&mut syntax);
    let item_item = parse_node_optional::<ast::ImportItem>(&mut syntax);

    let item_comments_after_importee = parse_many_comments_and_blankspace(&mut syntax)?;

    parse_token(&mut syntax, ast::SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("import"));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comments_after_import));

    if let Some(package_relative) = item_package_relative {
        formatted.extend(gen_import_package_relative(&package_relative)?);
    }
    if let Some(super_relative) = item_super_relative {
        formatted.extend(gen_import_super_relative(&super_relative)?);
    }
    if let Some(path) = item_path {
        formatted.extend(gen_import_path(&path)?);
    }
    if let Some(collection) = item_collection {
        formatted.extend(gen_import_collection(&collection)?);
    }
    if let Some(item) = item_item {
        formatted.extend(gen_import_item(&item)?);
    }

    formatted.extend(gen_comments(&item_comments_after_importee));

    if include_semicolon {
        formatted.discourage(RequestItem::Space);
        formatted.push_sc(sc!(";"));
    }

    Ok(formatted)
}
