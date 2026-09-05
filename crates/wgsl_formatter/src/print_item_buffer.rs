//! The working structure that the generator functions emit formatted text into.
pub mod spacing_request;

use dprint_core::formatting::{Anchor, Info, PrintItem, PrintItems, Signal};

use crate::print_item_buffer::spacing_request::Request;

// The motivating example for this is, that there is no obvious way to encode the following rules cleanly into "vanilla" PrintItems
// 1. There should not be a space between the name of a function and the opening parenthesis "fn main("
// 2. A block comment (/* aaa */) should be preceded and followed by a space
// 3. There should not be a space after the opening parenthesis of a function, even if the next token is a block comment
// 4. There should not be a space before the closing parenthesis of a function, even if the preceding token is a block comment
//
// Example formatting: fn main /*aaa*/ (/*bbb*/ param: u32, param2: u32 /*ccc*/)
//
// Considered alternatives:
// * Track if the last pushed item is a space, and branch on that everytime you would add a space
//   * Cons: Very verbose, imperative and brittle ("forget to update the last pushed item"), cannot deal with rule 4 properly.
// * "Cleverly" structure code and where to put spaces, so that these cases are implicitly dealt with
//   * Cons: "Clever" code that doesn't explicitly state intent, and thus is brittle, new requirements might require big restructurings
// * Excessive use of dprint's conditionals to check "was the last item a space"
//   * This would probably work, but dprint's conditionals are a lot less lightweight than a Request (which basically is 3 u8's that get and-ed)
//   * dprint's conditionals have a shortcoming in that they can't represent the intent of "no matter what comes next, i don't want a
//     space here". With conditionals we would have to check everywhere we push spaces if a space is allowed here.
// * Re-parse the AST into a formatting-ast which tracks comments etc.
//   * We already do that. The formatting-ast is built and immediately destroyed by the parse->format structure that
//     the gen_*_ functions are built with. While it makes the code cleaner, it does not solve the problem.
//
// Chosen solution:
// * Feels like it can most clearly encode the intent behind statements like
//   "add a comma, unless its followed by ')'" or "there should be a single space after 'fn' and before the name"
// * In the formatting code we don't actually care about "what exactly the next or previous token is", instead
//   we wan't to communicate that we may want separation to adjacent text.
/// A wrapper for `PrintItem`s which adds the ability to do "item-requests".
///
/// # Motivation
///
/// In many places code gets more terse when we can express that some sort of separation should or should
/// not be inserted at a point.
/// Instead of checking "if the previous print item was something we have to be separated from or not" we can
/// just request that "in the default case insert a space here". If the previous item is something that
/// does not want separation after it (like the opening parenthesis of a function) we can request
/// that there should not be a space after it, directly at the point where the parentheses are inserted.
///
/// Additionally this request system also allows to solve cases very elegantly, where otherwise we would
/// need a "look-ahead" in dprint, to determine if a space should be inserted or not.
/// (At the time of writing this, look-aheads don't seem to be supported by dprint.)
/// An example of such a case is when we remove needless parentheses - for example around the condition in a `break if`.
/// Typically we would want spaces around any parenthesis statement that we eliminated, however in the case of a `break if`
/// we do not want a space after the condition, because a semicolon goes there, and we don't need to be separated from that.
/// While that could be solved by checking the context of the parenthesis statement, in the case of nested parenthesis statements
/// this can get needlessly complicated quickly.
///
/// Another use case for the request system is that through requests we can guarantee that at no point will there be two spaces
/// following each other - as consecutive space requests are "combined" into one.
/// We can also know that a space after a newline is "combined" into just a newline - similarly with empty lines.
///
/// # Usage
///
/// All formatting should go through this struct, which keeps track of said requests, that can be issued
/// using [`Self::request`].
/// Requests are kept in a state where they can be merged with incoming requests until either [`Self::apply_end_request`] is called,
/// or a non-mergeable item (like a String or [`dprint_core_macros::sc!`]) is pushed, at which point the request is resolved and
/// its result is pushed to the output.
///
/// Take a look at the [`Request`] documentation for more details on how requests are resolved.
///
/// # Known downsides to this solution:
/// * Exponential blowup when using with dprint's conditionals (not a big problem most of the time as not many dprint conditionals are used consecutively)
/// * Another layer on top of dprint's IR, which doesn't feel like it should be necessary
///
#[derive(Default)]
pub struct PrintItemBuffer {
    pub items_before_start_request: PrintItems,
    pub start_request: Request,
    pub items: PrintItems,
    pub end_request: Request,
    pub items_after_end_request: PrintItems,
}

impl PrintItemBuffer {
    /// Pushes a [`Request`] onto the buffer.
    ///
    /// Requests that are added consecutively are combined together.
    ///
    /// Generally you don't need to worry about when Requests are combined, as that
    /// procedure is commutative - so conceptually Requests only get combined once
    /// all the Buffers get combined together (with [`Self::extend`]).
    pub fn request(
        &mut self,
        incoming_request: Request,
    ) {
        let request_tracker = if self.items.is_empty() {
            // PERFORMANCE: With the current implementation of the gen_ functions this path is a lot less likely - usually spacing requests
            // are issued between printitems.
            // A simple benchmark on large_file.rs yielded at 0.3ms speedup (2%) on my machine.
            std::hint::cold_path();

            &mut self.start_request
        } else {
            &mut self.end_request
        };

        *request_tracker = Request::combine(std::mem::take(request_tracker), incoming_request);
    }

    #[must_use]
    pub fn finish(self) -> PrintItems {
        let mut pi = PrintItems::default();
        pi.extend(self.items_before_start_request);
        self.start_request.resolve(&mut pi);
        pi.extend(self.items);
        self.end_request.resolve(&mut pi);
        pi
    }

    /// Forces the trailing requests in this buffer to be applied immediately, preventing further Requests from merging with them.
    ///
    /// Normally consecutive Requests get combined (so 2 spaces would get collapsed into one space for example). However
    /// if [`Self::apply_end_request`] is called in between two requests, they will not get combined.
    ///
    /// ```rust
    /// # use crate::wgsl_formatter::print_item_buffer::spacing_request::{Request, RequestItem};
    /// # use crate::wgsl_formatter::print_item_buffer::PrintItemBuffer;
    /// let formatted = dprint_core::formatting::format(
    ///     || {
    ///         let mut formatted = PrintItemBuffer::default();
    ///         formatted.push_sc(dprint_core_macros::sc!("|"));
    ///         formatted.request(Request::expect(RequestItem::Space));
    ///         formatted.request(Request::expect(RequestItem::Space));
    ///         formatted.push_sc(dprint_core_macros::sc!("|"));
    ///         formatted.request(Request::expect(RequestItem::Space));
    ///         formatted.apply_end_request();
    ///         formatted.request(Request::expect(RequestItem::Space));
    ///         formatted.push_sc(dprint_core_macros::sc!("|"));
    ///         formatted.finish()
    ///     },
    ///     dprint_core::formatting::PrintOptions {
    ///         max_width: 80,
    ///         indent_width: 4,
    ///         use_tabs: false,
    ///         new_line_text: "\n",
    ///     },
    /// );
    /// assert_eq!(formatted, "| |  |")
    /// ```
    pub fn apply_end_request(&mut self) {
        std::mem::take(&mut self.end_request).resolve(&mut self.items);
        let items_after_end_requests = std::mem::take(&mut self.items_after_end_request);
        self.items.extend(items_after_end_requests);
    }

    /// Appends another [`PrintItemBuffer`] onto this one.
    pub fn extend(
        &mut self,
        other: Self,
    ) {
        // PrintItemBuffer's should behave associatively,
        // extending a PrintItemBuffer A with another one B, should be equivalent
        // to doing all the actions that have been performed on B, on A.

        self.push_items_before_requests(other.items_before_start_request);
        // Merge the incoming start_request
        self.request(other.start_request);

        // If there are incoming items, apply the current end request and add the items
        if !other.items.is_empty() {
            self.apply_end_request();
            self.items.extend(other.items);
        }

        // Merge the incoming end_request
        self.request(other.end_request);

        self.push_items_after_requests(other.items_after_end_request);
    }

    fn push_items_before_requests(
        &mut self,
        items: PrintItems,
    ) {
        if self.items.is_empty() {
            self.items_before_start_request.extend(items);
        } else {
            self.items.extend(items);
        }
    }

    fn push_item_before_requests(
        &mut self,
        item: PrintItem,
    ) {
        let mut pi = PrintItems::default();
        pi.push_item(item);
        self.push_items_before_requests(pi);
    }

    fn push_items_after_requests(
        &mut self,
        items: PrintItems,
    ) {
        self.items_after_end_request.extend(items);
    }

    fn push_item_after_requests(
        &mut self,
        item: PrintItem,
    ) {
        let mut pi = PrintItems::default();
        pi.push_item(item);
        self.push_items_after_requests(pi);
    }

    /// Applies trailing requests and pushes a string to the buffer whose content is not yet known at compile time.
    ///
    /// The string may not contain newlines or tabs - those need to be pushed separately via [`Self::push_tab`] or [`Self::request`].
    /// Prefer using [`Self::push_sc`] whenever possible.
    ///
    /// # Panics
    /// If compiled with prefer-immediate-crash this function will panic if the string contains newlines or tabs.
    pub fn push_string(
        &mut self,
        string: String,
    ) {
        #[cfg(feature = "prefer-immediate-crash")]
        {
            assert!(
                !string.contains('\n'),
                "Cannot push string with newlines to PrintItemBuffer {string:?}"
            );
            assert!(
                !string.contains('\t'),
                "Cannot push string with tabs to PrintItemBuffer {string:?}"
            );
        }
        self.apply_end_request();
        self.items.push_string(string);
    }

    /// Applies trailing requests and pushes a literal tab character to the buffer.
    ///
    /// Do not use this for indentation, use [`Self::start_indent_before_request`] instead.
    pub fn push_tab(&mut self) {
        self.apply_end_request();
        self.items.push_signal(Signal::Tab);
    }

    /// Applies trailing requests and pushes a string to the buffer whose content is known at compile time.
    ///
    /// To obtain a [`dprint_core::formatting::StringContainer`] use [`dprint_core_macros::sc!`].
    /// If you need to push a string whose content is not known at compile time, use [`Self::push_string`].
    pub fn push_sc(
        &mut self,
        sc: &'static dprint_core::formatting::StringContainer,
    ) {
        self.apply_end_request();
        self.items.push_sc(sc);
    }

    /// Inserts a dprint-info into the buffer, before all trailing requests.
    ///
    /// This does not apply any trailing request, but instead inserts the info before them.
    /// If you need to add the info *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn push_info_before_requests<T>(
        &mut self,
        info: T,
    ) where
        T: Into<Info>,
    {
        self.push_item_before_requests(PrintItem::Info(info.into()));
    }

    /// Inserts a dprint-anchor into the buffer, before all trailing requests.
    ///
    /// This does not apply any trailing request, but instead inserts the anchor before them.
    /// If you need to add the anchor *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn push_anchor_before_requests<T>(
        &mut self,
        anchor: T,
    ) where
        T: Into<Anchor>,
    {
        self.push_item_before_requests(PrintItem::Anchor(anchor.into()));
    }

    /// Inserts a dprint-condition into the buffer, before all trailing requests.
    ///
    /// This does not apply any trailing request, but instead inserts the condition before them.
    /// If you need to add the condition *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn push_condition_before_requests(
        &mut self,
        condition: dprint_core::formatting::Condition,
    ) {
        if self.items.is_empty() {
            self.items_before_start_request.push_condition(condition);
        } else {
            self.items.push_condition(condition);
        }
    }

    /// Inserts a dprint-reevaluation into the buffer, before all trailing requests.
    ///
    /// This does not apply any trailing request, but instead inserts the reevaluation before them.
    /// If you need to add the reevaluation *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn push_reevaluation_before_requests(
        &mut self,
        reeval: dprint_core::formatting::ConditionReevaluation,
    ) {
        self.push_item_before_requests(PrintItem::ConditionReevaluation(reeval));
    }

    /// Starts a new indent level at the point before all trailing requests.
    ///
    /// This does not apply any trailing request, but instead starts the indentation before them.
    /// If you need to start the indentation *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn start_indent_before_requests(&mut self) {
        self.push_item_before_requests(PrintItem::Signal(Signal::StartIndent));
    }

    /// Finishes a new indent level at the point before all trailing requests.
    ///
    /// This does not apply any trailing request, but instead finishes the indentation before them.
    /// If you need to finish the indentation *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn finish_indent_before_requests(&mut self) {
        self.push_item_before_requests(PrintItem::Signal(Signal::FinishIndent));
    }

    /// Starts ignoring indentation at the point before all trailing requests.
    ///
    /// Any lines following this will not be indented, but instead start at column 0.
    ///
    /// This does not apply any trailing request, but instead starts ignoring indentation before them.
    /// If you need to do so *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn start_ignoring_indent_before_requests(&mut self) {
        self.push_item_before_requests(PrintItem::Signal(Signal::StartIgnoringIndent));
    }

    /// Stops ignoring indentation at the point before all trailing requests.
    ///
    /// Any lines following this will be indented again.
    ///
    /// This does not apply any trailing request, but instead stops ignoring indentation before them.
    /// If you need to do so *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn finish_ignoring_indent_before_requests(&mut self) {
        self.push_item_before_requests(PrintItem::Signal(Signal::FinishIgnoringIndent));
    }

    /// Decreases the precedence of following items getting broken into multiple lines.
    ///
    /// Linebreaks can happen at points where a [`Request::or_newline()`] was inserted.
    ///
    /// This does not apply any trailing request, but instead starts the newline group before them.
    /// If you need to do so *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn start_new_line_group_before_requests(&mut self) {
        //self.push_item_before_requests(PrintItem::String(dprint_core_macros::sc!("[")));
        self.push_item_before_requests(PrintItem::Signal(Signal::StartNewLineGroup));
    }

    /// Decreases the precedence of following items getting broken into multiple lines.
    ///
    /// Linebreaks can happen at points where a [`Request::or_newline()`] was inserted.
    ///
    /// This does not apply any trailing request, but instead queues the newline group to
    /// be started as soon as trailing requests are applied (either by pushing a concrete item
    /// or by calling [`Self::apply_end_request`]).
    pub fn start_new_line_group_after_requests(&mut self) {
        //self.push_item_after_requests(PrintItem::String(dprint_core_macros::sc!("[")));
        self.push_item_after_requests(PrintItem::Signal(Signal::StartNewLineGroup));
    }

    /// Increases the precedence of following items getting broken into multiple lines.
    ///
    /// Linebreaks can happen at points where a [`Request::or_newline()`] was inserted.
    ///
    /// This does not apply any trailing request, but instead starts the newline group before them.
    /// If you need to do so *after* trailing requests, manually call [`Self::apply_end_request`].
    pub fn finish_new_line_group_before_requests(&mut self) {
        //self.push_item_before_requests(PrintItem::String(dprint_core_macros::sc!("]")));
        self.push_item_before_requests(PrintItem::Signal(Signal::FinishNewLineGroup));
    }

    /// Increases the precedence of following items getting broken into multiple lines.
    ///
    /// Linebreaks can happen at points where a [`Request::or_newline()`] was inserted.
    ///
    /// This does not apply any trailing request, but instead queues the newline group to
    /// be started as soon as trailing requests are applied (either by pushing a concrete item
    /// or by calling [`Self::apply_end_request`]).
    pub fn finish_new_line_group_after_requests(&mut self) {
        //self.push_item_after_requests(PrintItem::String(dprint_core_macros::sc!("]")));
        self.push_item_after_requests(PrintItem::Signal(Signal::FinishNewLineGroup));
    }
}
