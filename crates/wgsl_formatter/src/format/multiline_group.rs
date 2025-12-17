use std::rc::Rc;

use dprint_core::formatting::{LineNumber, LineNumberAnchor, PrintItems, Signal, conditions};
use dprint_core_macros::sc;

use crate::format::{
    helpers::create_is_multiple_lines_resolver,
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
};

pub fn gen_multiline_group<I: IntoIterator<Item = PrintItemBuffer>>(lines: I) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();

    let start_ln = LineNumber::new("start");
    let end_ln = LineNumber::new("end");
    let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

    formatted.push_info(start_ln);
    formatted.push_anchor(LineNumberAnchor::new(end_ln));

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
        // formatted.request_line_break(SeparationPolicy::Expected);
        formatted.request(SeparationRequest {
            line_break: SeparationPolicy::ExpectedIf {
                on_branch: true,
                of_resolver: Rc::clone(&is_multiple_lines),
            },
            space: SeparationPolicy::ExpectedIf {
                on_branch: false,
                of_resolver: Rc::clone(&is_multiple_lines),
            },
            ..Default::default()
        });
    }

    // No trailing spaces
    formatted.request(SeparationRequest {
        space: SeparationPolicy::Discouraged,
        ..Default::default()
    });

    formatted.push_condition(conditions::if_true(
        "paramMultilineEndIndent",
        is_multiple_lines,
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::FinishIndent);
            pi
        },
    ));

    formatted.push_signal(Signal::FinishNewLineGroup);
    formatted.push_info(end_ln);
    formatted.push_reevaluation(start_reeval);

    formatted
}
