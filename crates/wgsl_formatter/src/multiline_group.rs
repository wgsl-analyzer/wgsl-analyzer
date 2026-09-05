//! Helper for the generator functions to generate items that should break together.
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use dprint_core::formatting::{
    ConditionReevaluation, ConditionResolver, ConditionResolverContext, LineNumber,
    LineNumberAnchor, PrintItems, condition_helpers, conditions,
};

use crate::print_item_buffer::{PrintItemBuffer, spacing_request::Request};

use super::print_item_buffer::spacing_request::RequestItem;

#[cfg(debug_assertions)]
#[derive(Debug)]
enum MultilineGroupState {
    New,
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

#[must_use]
pub fn create_is_multiple_lines_resolver(
    start_ln: LineNumber,
    end_ln: LineNumber,
) -> ConditionResolver {
    Rc::new(
        move |condition_context: &mut ConditionResolverContext<'_, '_>| {
            condition_helpers::is_multiple_lines(condition_context, start_ln, end_ln)
        },
    )
}

impl<'buffer> MultilineGroup<'buffer> {
    pub fn new_before_requests(formatted: &'buffer mut PrintItemBuffer) -> Self {
        let start_ln = LineNumber::new("start");
        let end_ln = LineNumber::new("end");
        let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

        formatted.start_new_line_group_before_requests();
        formatted.push_info_before_requests(start_ln);
        formatted.push_anchor_before_requests(LineNumberAnchor::new(end_ln));

        let mut start_nl_condition = conditions::if_true_or(
            "paramMultilineStartIndent",
            Rc::clone(&is_multiple_lines),
            PrintItems::default(),
            PrintItems::default(),
        );

        let start_reeval = Some(start_nl_condition.create_reevaluation());
        formatted.push_condition_before_requests(start_nl_condition);

        Self {
            buffer: formatted,
            is_multiple_lines,
            end_ln,
            start_reeval,

            #[cfg(debug_assertions)]
            state: MultilineGroupState::New,
        }
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
        self.grouped_request(
            Request::expect(RequestItem::LineBreak),
            Request::empty().or_newline(),
        );
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

    pub fn end_before_requests(mut self) {
        #[cfg(debug_assertions)]
        {
            core::assert_matches!(
                self.state,
                MultilineGroupState::New,
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

#[cfg(debug_assertions)]
impl Drop for MultilineGroup<'_> {
    fn drop(&mut self) {
        // Come on we need linear types, please...
        if !::std::thread::panicking() {
            core::assert_matches!(
                self.state,
                MultilineGroupState::Ended,
                "MultilineGroup was dropped without end_before_requests having been called"
            );
        }
    }
}
