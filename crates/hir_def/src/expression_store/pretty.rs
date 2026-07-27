//! A pretty-printer for HIR.

use std::{
    fmt::{self, Write},
    mem,
};

use base_db::Lookup;
use syntax::{Edition, ast::HasName as _};

use super::{ast, ExpressionStore, ExpressionId, Expression, TypeSpecifierId};
use crate::{
    HasSource, InFile,
    body::{Binding, BindingId, Body, BodySourceMap},
    database::{DefDatabase, DefinitionWithBodyId, FunctionId, ModuleDefinitionId, StructId},
    expression::{Literal, Statement},
    expression_store::path::Path,
    mod_path::PathKind,
    signature::{FunctionSignature, StructSignature},
    type_specifier::IdentExpression,
};
use itertools::Itertools as _;

macro_rules! write {
    ($destination:expr, $($argument:tt)*) => {
        {
            #[expect(clippy::let_underscore_untyped, reason = "not a concern here")]
            let _ = std::write!($destination, $($argument)*).unwrap();
        }
    };
}

macro_rules! write_line {
    ($destination:expr) => {
        { $destination.newline(); }
    };
    ($destination:expr, $($argument:tt)*) => {
        {
            #[expect(clippy::let_underscore_untyped, reason = "not a concern here")]
            let _ = write!($destination, $($argument)*); $destination.newline();
        }
    };
}

macro_rules! write_name {
    ($self:ident, $item_id:ident) => {{
        write!(self, "{}", item_name(self.database, $item_id, "<missing>"));
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineFormat {
    Oneline,
    Newline,
    Indentation,
}

fn item_name<Id, Location>(
    database: &dyn DefDatabase,
    id: Id,
    default: &str,
) -> String
where
    Id: Lookup<Data = Location, Database = (dyn DefDatabase + 'static)>,
    Location: HasSource,
    Location::Value: ast::HasName,
{
    let location = id.lookup(database);
    let source = location.source(database);
    let name = source.value.name();
    name_to_string(name.as_ref(), default)
}

fn name_to_string(
    name: Option<&ast::Name>,
    default: &str,
) -> String {
    name.and_then(syntax::ast::Name::ident_token).map_or_else(|| default.to_owned(), |token| token.to_string())
}

pub fn print_signature(
    db: &dyn DefDatabase,
    owner: ModuleDefinitionId,
    edition: Edition,
) -> String {
    match owner {
        ModuleDefinitionId::Struct(id) => {
            let signature = db.struct_data(id).0;
            print_struct(db, id, &signature, edition)
        },
        ModuleDefinitionId::Function(id) => {
            let signature = db.function_data(id).0;
            print_function(db, id, &signature, edition)
        },
        ModuleDefinitionId::GlobalConstant(id) => format!("unimplemented {id:?}"),
        ModuleDefinitionId::GlobalAssertStatement(id) => format!("unimplemented {id:?}"),
        ModuleDefinitionId::GlobalVariable(id) => format!("unimplemented {id:?}"),
        ModuleDefinitionId::Override(id) => format!("unimplemented {id:?}"),
        ModuleDefinitionId::TypeAlias(id) => format!("unimplemented {id:?}"),
        ModuleDefinitionId::Module(id) => format!("unimplemented {id:?}"),
    }
}

pub fn print_path(
    db: &dyn DefDatabase,
    store: &ExpressionStore,
    path: &Path,
    edition: Edition,
) -> String {
    let mut printer = Printer {
        database: db,
        store,
        buffer: String::new(),
        indentation_level: 0,
        line_format: LineFormat::Newline,
        edition,
    };
    printer.print_path(path);
    printer.buffer
}

pub fn print_struct(
    db: &dyn DefDatabase,
    id: StructId,
    StructSignature {
        name,
        store,
        fields,
    }: &StructSignature,
    edition: Edition,
) -> String {
    let mut printer = Printer {
        database: db,
        store,
        buffer: String::new(),
        indentation_level: 0,
        line_format: LineFormat::Newline,
        edition,
    };
    write!(printer, "struct ");
    write!(printer, "{}", name.as_str());
    write_line!(printer, " {{...}}");
    printer.buffer
}

pub fn print_function(
    db: &dyn DefDatabase,
    id: FunctionId,
    signature @ FunctionSignature {
        name,
        store,
        parameters,
        return_type,
    }: &FunctionSignature,
    edition: Edition,
) -> String {
    let mut printer = Printer {
        database: db,
        store,
        buffer: String::new(),
        indentation_level: 0,
        line_format: LineFormat::Newline,
        edition,
    };
    write!(printer, "fn ");
    write!(printer, "{}", name.as_str());
    write!(printer, "(");
    for (i, (parameter, parameter_data)) in parameters.iter().enumerate() {
        if i != 0 {
            write!(printer, ", ");
        }
        printer.print_type_ref(parameter_data.r#type);
    }
    write!(printer, ")");
    if let Some(ret_type) = return_type {
        write!(printer, " -> ");
        printer.print_type_ref(*ret_type);
    }
    write_line!(printer, " {{...}}");

    printer.buffer
}

pub fn print_expr_hir(
    db: &dyn DefDatabase,
    store: &ExpressionStore,
    expression: ExpressionId,
    edition: Edition,
) -> String {
    let mut printer = Printer {
        database: db,
        store,
        buffer: String::new(),
        indentation_level: 0,
        line_format: LineFormat::Newline,
        edition,
    };
    printer.print_expression(expression);
    printer.buffer
}

struct Printer<'a> {
    database: &'a dyn DefDatabase,
    store: &'a ExpressionStore,
    buffer: String,
    indentation_level: usize,
    line_format: LineFormat,
    edition: Edition,
}

impl Write for Printer<'_> {
    fn write_str(
        &mut self,
        s: &str,
    ) -> fmt::Result {
        for line in s.split_inclusive('\n') {
            if matches!(self.line_format, LineFormat::Indentation) {
                match self.buffer.chars().rev().find(|ch| *ch != ' ') {
                    Some('\n') | None => {},
                    _ => self.buffer.push('\n'),
                }
                self.buffer.push_str(&"    ".repeat(self.indentation_level));
            }

            self.buffer.push_str(line);

            if matches!(
                self.line_format,
                LineFormat::Newline | LineFormat::Indentation
            ) {
                self.line_format = if line.ends_with('\n') {
                    LineFormat::Indentation
                } else {
                    LineFormat::Newline
                };
            }
        }

        Ok(())
    }
}

impl Printer<'_> {
    fn indented(
        &mut self,
        f: impl FnOnce(&mut Self),
    ) {
        self.indentation_level += 1;
        write_line!(self);
        f(self);
        self.indentation_level -= 1;
        self.buffer = self.buffer.trim_end_matches('\n').to_owned();
    }

    fn whitespace(&mut self) {
        match self.buffer.chars().next_back() {
            None | Some('\n' | ' ') => {},
            _ => self.buffer.push(' '),
        }
    }

    // Add a newline if the current line is not empty.
    // If the current line is empty, add a space instead.
    //
    // Do not use [`writeln!()`] or [`wln!()`] here, which will result in
    // infinite recursive calls to this function.
    fn newline(&mut self) {
        if matches!(self.line_format, LineFormat::Oneline) {
            match self.buffer.chars().last() {
                Some(' ') | None => {},
                Some(_) => {
                    write!(self, " ");
                },
            }
        } else {
            match self.buffer.chars().rev().find_position(|ch| *ch != ' ') {
                Some((_, '\n')) | None => {},
                Some((idx, _)) => {
                    if idx != 0 {
                        self.buffer.drain(self.buffer.len() - idx..);
                    }
                    write!(self, "\n");
                },
            }
        }
    }

    fn print_identifier_expression(
        &mut self,
        identifier_expression: &IdentExpression,
    ) {
        self.print_path(&identifier_expression.path);
        self.print_template(&identifier_expression.template_parameters);
    }

    fn print_expression(
        &mut self,
        expression: ExpressionId,
    ) {
        let expression = &self.store[expression];

        match expression {
            Expression::Missing => write!(self, "\u{fffd}"),
            Expression::IdentExpression(ident) => self.print_path(&ident.path),
            Expression::Call {
                ident_expression,
                arguments,
            } => {
                self.print_identifier_expression(ident_expression);
                write!(self, "(");
                if !arguments.is_empty() {
                    self.indented(|printer| {
                        for argument in &**arguments {
                            printer.print_expression(*argument);
                            write_line!(printer, ",");
                        }
                    });
                }
                write!(self, ")");
            },
            Expression::Field { expression, name } => {
                self.print_expression(*expression);
                write!(self, ".{}", name.as_str());
            },
            Expression::UnaryOperator {
                expression,
                operator,
            } => {
                write!(self, "{}", operator.symbol());
                self.print_expression(*expression);
            },
            Expression::BinaryOperation {
                left_side,
                right_side,
                operation,
            } => {
                self.print_expression(*left_side);
                self.whitespace();
                write!(self, "{}", operation.symbol());
                self.whitespace();
                self.print_expression(*right_side);
            },
            Expression::Index { left_side, index } => {
                self.print_expression(*left_side);
                write!(self, "[");
                self.print_expression(*index);
                write!(self, "]");
            },
            Expression::Literal(literal) => self.print_literal(*literal),
        }
    }

    fn print_body(
        &mut self,
        body: &Body,
    ) {
        self.whitespace();
        if let Some(main_binding) = &body.main_binding {
            self.print_binding(body, *main_binding);
        }
        write!(self, "{{");
        if !body.statements.is_empty() {
            self.indented(|printer| {
                for (_, statement) in body.statements.iter() {
                    printer.print_statement(body, statement);
                }
                printer.newline();
            });
        }
        write!(self, "}}");
    }

    fn print_statement(
        &mut self,
        body: &Body,
        stmt: &Statement,
    ) {
        match stmt {
            Statement::Assert { expression } => (),
            Statement::Assignment {
                left_side,
                right_side,
            } => (),
            Statement::Break => {
                write!(self, "break;");
            },
            Statement::BreakIf { condition } => (),
            Statement::Compound { statements } => (),
            Statement::CompoundAssignment {
                left_side,
                right_side,
                operator,
            } => (),
            Statement::Const {
                binding_id,
                type_ref,
                initializer,
            } => (),
            Statement::Continue => {
                write!(self, "continue;");
            },
            Statement::Continuing { block } => {
                write!(self, "continuing");
                self.whitespace();
                self.print_statement(body, &body.statements[*block]);
            },
            Statement::Discard => (),
            Statement::Expression { expression } => {
                self.print_expression(*expression);
                write!(self, ";");
                write_line!(self);
            },
            Statement::For {
                initializer,
                condition,
                continuing_part,
                block,
            } => (),
            Statement::If {
                condition,
                block,
                else_if_blocks,
                else_block,
            } => (),
            Statement::IncrDecr {
                expression,
                operator,
            } => (),
            Statement::Let {
                type_ref,
                initializer,
                binding_id,
            } => {
                write!(self, "let ");
                self.print_binding(body, *binding_id);
                if let Some(type_ref) = type_ref {
                    write!(self, ": ");
                    self.print_type_ref(*type_ref);
                }
                if let Some(initializer) = initializer {
                    write!(self, " = ");
                    self.print_expression(*initializer);
                }
                write_line!(self, ";");
            },
            Statement::Loop { body } => {
                // loop keyword
                // print body
            },
            Statement::Missing => {
                // print semicolon
            },
            Statement::PhonyAssignment { right_side } => {
                // print underscore
                // print equals
                // print right side
                // print semicolon
            },
            Statement::Return { expression } => {
                // print return keyword
                // print expression
                // print semicolon
            },
            Statement::Switch {
                expression,
                case_blocks,
            } => {
                // print switch keyword
                // print expression
                // print opening brace
                // for each case_block in case_blocks
                //   print case keyword
                //   for each case_selector in case_blocks.case_selector
                //     print case_selector
                //   print colon
                //   print statement
                // print closing brace
            },
            Statement::Variable {
                binding_id,
                type_ref,
                initializer,
                template_arguments,
            } => {
                write!(self, "var");
                self.print_template(template_arguments);
                self.whitespace();
                self.print_binding(body, *binding_id);
                if let Some(type_ref) = type_ref {
                    write!(self, ": ");
                    self.print_type_ref(*type_ref);
                }
                if let Some(initializer) = initializer {
                    write!(self, " = ");
                    self.print_expression(*initializer);
                }
                write_line!(self, ";");
            },
            Statement::While { condition, block } => (),
        }
    }

    fn print_literal(
        &mut self,
        literal: Literal,
    ) {
        match literal {
            Literal::Bool(boolean) => write!(self, "{}", boolean),
            Literal::Int(integer, kind) => {
                write!(self, "{}", integer);
                write!(self, "{}", kind.suffix());
            },
            Literal::Float(float, kind) => {
                write!(self, "{}", float);
                write!(self, "{}", kind.suffix());
            },
        }
    }

    fn print_binding(
        &mut self,
        body: &Body,
        id: BindingId,
    ) {
        write!(self, "{}", body.bindings[id].name.as_str());
    }

    fn print_path(
        &mut self,
        path: &Path,
    ) {
        match path.kind() {
            PathKind::Plain => {},
            PathKind::SELF => write!(self, "self"),
            PathKind::Super(supers) => {
                for super_level in 0..supers {
                    if super_level == 0 {
                        write!(self, "super");
                    } else {
                        write!(self, "::super");
                    }
                }
            },
            PathKind::Package => write!(self, "package"),
        }

        for (index, segment) in path.segments().iter().enumerate() {
            if index != 0 || !matches!(path.kind(), PathKind::Plain) {
                write!(self, "::");
            }

            write!(self, "{}", segment.as_str());
        }
    }

    pub(crate) fn print_template(
        &mut self,
        template_arguments: &Vec<ExpressionId>,
    ) {
        write!(self, "<");
        let mut first = true;
        for argument in template_arguments {
            if !first {
                write!(self, ", ");
            }
            first = false;
            self.print_expression(*argument);
        }
        write!(self, ">");
    }

    pub(crate) fn print_type_ref(
        &mut self,
        type_ref: TypeSpecifierId,
    ) {
        // FIXME: deduplicate with `HirDisplay` impl
        let type_specifier = &self.store.types[type_ref];
        self.print_path(&type_specifier.path);
        self.print_template(&type_specifier.template_parameters);
    }
}
