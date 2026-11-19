//! Helper for the generator functions to generate items that should break together.
use std::ops::{Deref, DerefMut};

use dprint_core::formatting::{
    Arena, ConditionReevaluation, ConditionResolver, LineNumber, LineNumberAnchor, PrintItems,
    condition_helpers, conditions,
};

use super::print_item_buffer::spacing_request::RequestItem;
use crate::print_item_buffer::{PrintItemBuffer, spacing_request::Request};

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
/// The [`MultilineGroup::end_before_requests`] method needs to be called before it is dropped.
///
pub(crate) struct MultilineGroup<'group, 'buffer> {
    buffer: &'group mut PrintItemBuffer<'buffer>,
    pub(crate) is_multiple_lines: ConditionResolver<'buffer>,
    end_ln: LineNumber,
    start_reeval: Option<ConditionReevaluation>,

    #[cfg(debug_assertions)]
    state: MultilineGroupState,
}

#[must_use]
pub(crate) fn create_is_multiple_lines_resolver<'buffer>(
    arena: &'buffer Arena,
    start_ln: LineNumber,
    end_ln: LineNumber,
) -> ConditionResolver<'buffer> {
    arena.alloc_resolver(move |condition_context| {
        condition_helpers::is_multiple_lines(condition_context, start_ln, end_ln)
    })
}

impl<'group, 'buffer> MultilineGroup<'group, 'buffer> {
    pub(crate) fn new_before_requests(
        arena: &'buffer Arena,
        formatted: &'group mut PrintItemBuffer<'buffer>,
    ) -> Self {
        let start_ln = LineNumber::new("start");
        let end_ln = LineNumber::new("end");
        let is_multiple_lines = create_is_multiple_lines_resolver(arena, start_ln, end_ln);

        formatted.start_new_line_group_before_requests(arena);
        formatted.push_info_before_requests(arena, start_ln);
        formatted.push_anchor_before_requests(arena, LineNumberAnchor::new(end_ln));

        let mut start_nl_condition = conditions::if_true_or(
            "paramMultilineStartIndent",
            is_multiple_lines,
            PrintItems::new(arena),
            PrintItems::new(arena),
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

    pub(crate) fn grouped_newline_or_space(&mut self) {
        self.buffer.request(Request::Conditional {
            condition: self.is_multiple_lines,
            on_true: Box::new(Request::expect(RequestItem::LineBreak)),
            on_false: Box::new(Request::expect(RequestItem::Space).or_newline()),
        });
    }

    pub(crate) fn grouped_request(
        &mut self,
        request_on_multiline: Request<'buffer>,
        request_on_single_line: Request<'buffer>,
    ) {
        self.buffer.request(Request::Conditional {
            condition: self.is_multiple_lines,
            on_true: Box::new(request_on_multiline),
            on_false: Box::new(request_on_single_line),
        });
    }

    pub(crate) fn grouped_possible_newline(&mut self) {
        self.grouped_request(
            Request::expect(RequestItem::LineBreak),
            Request::empty().or_newline(),
        );
    }

    pub(crate) fn extend_if_multi_line(
        &mut self,
        arena: &'buffer Arena,
        items: PrintItems<'buffer>,
    ) {
        self.buffer.apply_end_request(arena);
        self.buffer
            .push_condition_before_requests(conditions::if_true(
                "paramTrailingComma",
                self.is_multiple_lines,
                items,
            ));
    }

    #[cfg_attr(
        not(debug_assertions),
        expect(unused_mut, reason = "state not modified")
    )]
    pub(crate) fn end_before_requests(
        mut self,
        arena: &'buffer Arena,
    ) {
        #[cfg(debug_assertions)]
        {
            core::assert_matches!(
                self.state,
                MultilineGroupState::New,
                "MultilineGroup was in wrong state"
            );
            self.state = MultilineGroupState::Ended;
        }

        self.buffer.push_info_before_requests(arena, self.end_ln);

        // It is legal to call end without calling start_ident or finish_indent
        if let Some(start_reeval) = self.start_reeval {
            self.buffer
                .push_reevaluation_before_requests(arena, start_reeval);
        }
        self.buffer.finish_new_line_group_before_requests(arena);
    }
}

impl<'group, 'buffer> Deref for MultilineGroup<'group, 'buffer> {
    type Target = PrintItemBuffer<'buffer>;

    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl DerefMut for MultilineGroup<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer
    }
}

#[cfg(debug_assertions)]
impl Drop for MultilineGroup<'_, '_> {
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
