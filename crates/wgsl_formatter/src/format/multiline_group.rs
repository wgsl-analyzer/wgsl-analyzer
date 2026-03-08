use std::{collections::BTreeSet, rc::Rc};

use dprint_core::formatting::{
    LineNumber, LineNumberAnchor, PrintItem, PrintItems, Signal, conditions,
};

use crate::format::{
    format,
    helpers::create_is_multiple_lines_resolver,
    print_item_buffer::{
        PrintItemBuffer, SeparationPolicy, SeparationRequest,
        request_folder::{Request, RequestFolder},
    },
};

use super::print_item_buffer::request_folder::RequestItem;

pub fn gen_multiline_group<I: IntoIterator<Item = PrintItemBuffer>>(lines: I) -> PrintItemBuffer {
    gen_surrounded_group(None, lines, None)
}

pub fn gen_surrounded_group<I: IntoIterator<Item = PrintItemBuffer>>(
    opener: Option<PrintItemBuffer>,
    lines: I,
    closer: Option<PrintItemBuffer>,
) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();

    let start_ln = LineNumber::new("start");
    let end_ln = LineNumber::new("end");
    let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

    formatted.push_info(start_ln);
    formatted.push_anchor(LineNumberAnchor::new(end_ln));
    if let Some(opener) = opener {
        formatted.extend(opener);
    }

    let mut start_nl_condition = conditions::if_true_or(
        "paramMultilineStartIndent",
        Rc::clone(&is_multiple_lines),
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
    let start_reeval = start_nl_condition.create_reevaluation();
    formatted.push_condition(start_nl_condition);
    formatted.push_signal(Signal::StartNewLineGroup);

    // TODO This is a bit of a shortcoming of the PBI api, we would want to write this after the "(", but can't because of the conditions between
    formatted.request(SeparationRequest::discouraged());

    for line in lines {
        formatted.extend(line);
        formatted.request_request(Request::Conditional {
            condition: Rc::clone(&is_multiple_lines),
            on_true: Box::new(RequestFolder::from(Request::Unconditional {
                expected: BTreeSet::from([RequestItem::LineBreak]),
                discouraged: BTreeSet::new(),
                forced: BTreeSet::new(),
            })),
            on_false: Box::new(RequestFolder::from(Request::Unconditional {
                expected: BTreeSet::from([RequestItem::Space]),
                discouraged: BTreeSet::new(),
                forced: BTreeSet::new(),
            })),
        });
    }

    // No trailing spaces
    formatted.discourage(RequestItem::Space);

    formatted.push_condition(conditions::if_true(
        "paramMultilineEndIndent",
        is_multiple_lines,
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::FinishIndent);
            pi
        },
    ));

    if let Some(closer) = closer {
        formatted.extend(closer);
    }

    formatted.push_signal(Signal::FinishNewLineGroup);
    formatted.push_info(end_ln);
    formatted.push_reevaluation(start_reeval);

    formatted
}
