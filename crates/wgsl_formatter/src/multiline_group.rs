use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use dprint_core::formatting::{
    ConditionReevaluation, ConditionResolver, LineNumber, LineNumberAnchor, PrintItems, Signal,
    conditions,
};

use crate::{
    helpers::create_is_multiple_lines_resolver,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItemSet},
    },
};

use super::print_item_buffer::spacing_request::RequestItem;

#[cfg(debug_assertions)]
#[derive(Debug)]
enum MultilineGroupState {
    New,
    StartedIndent,
    FinishedIndent,
    Ended,
}

/// Helper to generate a number of items that are either within a single line all on separate lines.
///
/// To use this helper (and to keep the api small), a few rules do need to be manually followed.
///
/// The [`MultilineGroup::end`] method needs to be called before it is dropped.
///
pub struct MultilineGroup<'buffer> {
    buffer: &'buffer mut PrintItemBuffer,
    pub(crate) is_multiple_lines: ConditionResolver,
    end_ln: LineNumber,
    start_reeval: Option<ConditionReevaluation>,

    #[cfg(debug_assertions)]
    state: MultilineGroupState,
}

impl<'buffer> MultilineGroup<'buffer> {
    #[deprecated]
    pub fn new(formatted: &'buffer mut PrintItemBuffer) -> Self {
        formatted.apply_end_request();
        Self::new_before_requests(formatted)
    }

    pub fn new_before_requests(formatted: &'buffer mut PrintItemBuffer) -> Self {
        let start_ln = LineNumber::new("start");
        let end_ln = LineNumber::new("end");
        let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

        formatted.start_new_line_group_before_requests();
        formatted.push_info_before_requests(start_ln);
        formatted.push_anchor_before_requests(LineNumberAnchor::new(end_ln));

        Self {
            buffer: formatted,
            is_multiple_lines,
            end_ln,
            start_reeval: None,

            #[cfg(debug_assertions)]
            state: MultilineGroupState::New,
        }
    }

    #[deprecated]
    pub fn start_indent(&mut self) {
        self.apply_end_request();
        self.start_indent_before_requests();
    }

    pub fn start_indent_before_requests(&mut self) {
        #[cfg(debug_assertions)]
        {
            core::assert_matches!(
                self.state,
                MultilineGroupState::New,
                "MultilineGroup was in wrong state"
            );
            self.state = MultilineGroupState::StartedIndent;
        }

        // TODO I wonder if we can get rid of this thing inserting newlines here...
        let mut start_nl_condition = conditions::if_true_or(
            "paramMultilineStartIndent",
            Rc::clone(&self.is_multiple_lines),
            {
                let mut pi = PrintItems::default();
                pi.push_signal(Signal::NewLine);
                pi
            },
            {
                let mut pi = PrintItems::default();
                pi.push_signal(Signal::PossibleNewLine);
                pi
            },
        );
        self.start_reeval = Some(start_nl_condition.create_reevaluation());
        self.buffer
            .push_condition_before_requests(start_nl_condition);
        self.buffer.start_indent_before_requests();

        // This is a bit of a shortcoming of the PBI api, this does not really belong into multilinegroup,
        // and would better be located directly after a "(" token (or whatever was used to open the multilinegroup)
        // However because pushing the start_nl_condition and the indent will reset any request - it best fits here for now,
        // until we can move the start_nl_condition to also use the RequestFolder api
        self.buffer.request(Request::Unconditional {
            expected: RequestItemSet::empty(),
            discouraged: RequestItemSet::empty()
                .extended_by(RequestItem::Space)
                .extended_by(RequestItem::LineBreak)
                .extended_by(RequestItem::EmptyLine),
            forced: RequestItemSet::empty(),
            suggest_linebreak: false,
        });
    }

    pub fn grouped_newline_or_space(&mut self) {
        self.buffer.request(Request::Conditional {
            condition: Rc::clone(&self.is_multiple_lines),
            on_true: Box::new(Request::expect(RequestItem::LineBreak)),
            on_false: Box::new(Request::expect(RequestItem::Space).or_newline()),
        });
    }

    pub fn grouped_request(
        &mut self,
        request_on_multiline: Request,
        request_on_single_line: Request,
    ) {
        self.buffer.request(Request::Conditional {
            condition: Rc::clone(&self.is_multiple_lines),
            on_true: Box::new(request_on_multiline),
            on_false: Box::new(request_on_single_line),
        });
    }

    pub fn grouped_possible_newline(&mut self) {
        self.buffer.request(Request::Conditional {
            condition: Rc::clone(&self.is_multiple_lines),
            on_true: Box::new(Request::expect(RequestItem::LineBreak)),
            on_false: Box::new(Request::empty().or_newline()),
        });
    }

    pub fn extend_if_multi_line(
        &mut self,
        items: PrintItems,
    ) {
        self.buffer.apply_end_request();
        self.buffer
            .push_condition_before_requests(conditions::if_true(
                "paramTrailingComma",
                Rc::clone(&self.is_multiple_lines),
                items,
            ));
    }

    pub fn finish_indent(&mut self) {
        #[cfg(debug_assertions)]
        {
            core::assert_matches!(
                self.state,
                MultilineGroupState::StartedIndent,
                "MultilineGroup was in wrong state"
            );
            self.state = MultilineGroupState::FinishedIndent;
        }

        self.buffer.finish_indent_before_requests();
    }

    #[deprecated]
    pub fn end(&mut self) {
        self.apply_end_request();
        self.end_before_requests();
    }

    pub fn end_before_requests(&mut self) {
        #[cfg(debug_assertions)]
        {
            core::assert_matches!(
                self.state,
                MultilineGroupState::FinishedIndent | MultilineGroupState::New,
                "MultilineGroup was in wrong state"
            );
            self.state = MultilineGroupState::Ended;
        }

        self.buffer.push_info_before_requests(self.end_ln);

        // It is legal to call end without calling start_ident or finish_indent
        if let Some(start_reeval) = self.start_reeval {
            self.buffer.push_reevaluation_before_requests(start_reeval);
        }
        self.buffer.finish_new_line_group_before_requests();
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
