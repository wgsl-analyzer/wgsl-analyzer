use std::{
    collections::BTreeSet,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use dprint_core::formatting::{
    ConditionReevaluation, ConditionResolver, LineNumber, LineNumberAnchor, PrintItems, Signal,
    conditions,
};

use crate::format::{
    helpers::create_is_multiple_lines_resolver,
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestFolder},
    },
};

use super::print_item_buffer::request_folder::RequestItem;

pub struct MultilineGroup<'buffer> {
    buffer: &'buffer mut PrintItemBuffer,
    pub(crate) is_multiple_lines: ConditionResolver,
    end_ln: LineNumber,
    start_reeval: Option<ConditionReevaluation>,
}

impl<'buffer> MultilineGroup<'buffer> {
    pub fn new(formatted: &'buffer mut PrintItemBuffer) -> Self {
        let start_ln = LineNumber::new("start");
        let end_ln = LineNumber::new("end");
        let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

        formatted.push_info(start_ln);
        formatted.push_anchor(LineNumberAnchor::new(end_ln));

        Self {
            buffer: formatted,
            is_multiple_lines,
            end_ln,
            start_reeval: None,
        }
    }

    pub fn start_indent(&mut self) {
        let mut start_nl_condition = conditions::if_true_or(
            "paramMultilineStartIndent",
            Rc::clone(&self.is_multiple_lines),
            {
                let mut pi = PrintItems::default();
                pi.push_signal(Signal::NewLine);
                pi.push_signal(Signal::StartIndent);
                pi
            },
            {
                let mut pi = PrintItems::default();
                pi.push_signal(Signal::PossibleNewLine);
                pi
            },
        );
        self.start_reeval = Some(start_nl_condition.create_reevaluation());
        self.buffer.push_condition(start_nl_condition);
        self.buffer.start_new_line_group();

        // TODO This is a bit of a shortcoming of the PBI api, we would want to write this after the "(", but can't because of the conditions between
        // TODO This does not belong into multilinegroup
        self.buffer.request(Request::Unconditional {
            expected: BTreeSet::new(),
            discouraged: BTreeSet::from([
                RequestItem::Space,
                RequestItem::LineBreak,
                RequestItem::EmptyLine,
            ]),
            forced: BTreeSet::new(),
            suggest_linebreak: false,
        });
    }

    pub fn grouped_newline_or_space(&mut self) {
        self.buffer.request(Request::Conditional {
            condition: Rc::clone(&self.is_multiple_lines),
            on_true: Box::new(RequestFolder::from(Request::Unconditional {
                expected: BTreeSet::from([RequestItem::LineBreak]),
                discouraged: BTreeSet::new(),
                forced: BTreeSet::new(),
                suggest_linebreak: false,
            })),
            on_false: Box::new(RequestFolder::from(Request::Unconditional {
                expected: BTreeSet::from([RequestItem::Space]),
                discouraged: BTreeSet::new(),
                forced: BTreeSet::new(),
                suggest_linebreak: true,
            })),
        });
    }

    pub fn grouped_possible_newline(&mut self) {
        self.buffer.request(Request::Conditional {
            condition: Rc::clone(&self.is_multiple_lines),
            on_true: Box::new(RequestFolder::from(Request::Unconditional {
                expected: BTreeSet::from([RequestItem::LineBreak]),
                discouraged: BTreeSet::new(),
                forced: BTreeSet::new(),
                suggest_linebreak: false,
            })),
            on_false: Box::new(RequestFolder::from(Request::Unconditional {
                expected: BTreeSet::new(),
                discouraged: BTreeSet::new(),
                forced: BTreeSet::new(),
                suggest_linebreak: true,
            })),
        });
    }

    pub fn extend_if_multi_line(
        &mut self,
        items: PrintItems,
    ) {
        self.buffer.push_condition(conditions::if_true(
            "paramTrailingComma",
            Rc::clone(&self.is_multiple_lines),
            items,
        ));
    }

    pub fn finish_indent(&mut self) {
        // No trailing spaces
        self.buffer.discourage(RequestItem::Space);

        self.buffer.push_condition(conditions::if_true(
            "paramMultilineEndIndent",
            Rc::clone(&self.is_multiple_lines),
            {
                let mut pi = PrintItems::default();
                pi.push_signal(Signal::FinishIndent);
                pi
            },
        ));
    }

    pub fn end(&mut self) {
        self.buffer.finish_new_line_group();
        self.buffer.push_info(self.end_ln);
        if let Some(start_reeval) = self.start_reeval {
            self.buffer.push_reevaluation(start_reeval);
        }
    }
}

impl Deref for MultilineGroup<'_> {
    type Target = PrintItemBuffer;

    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl DerefMut for MultilineGroup<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer
    }
}
