use dprint_core::formatting::{
    Anchor, Condition, ConditionResolver, Info, PrintItem, PrintItems, Signal, conditions,
};
use dprint_core_macros::sc;

#[derive(Default)]
pub enum SeparationPolicy {
    /// The separation will be present - even if it's discouraged
    Forced,
    /// The separation will be present - unless it's discouraged
    Expected,

    // TODO Allowed is only used for newlines, split SeparationPolicy for newlines and other types
    /// If the layout engine wants to have a separation here in order to fulfill its constraints, it will be allowed - unless it gets discouraged elsewhere
    ///
    /// Mainly used to signal possible newlines
    Allowed,
    /// No separation should appear heare - unless it gets forced
    Discouraged,
    /// No statement about wether or not there should be a separation here
    #[default]
    Ignored,

    /// Condition acts as expected if the condition is true, otherwise its ignored
    // TODO That this is a special case on expected, really shows that this is poorly designed
    // However I will leave it like that for now, until we have actual need (and therefore insight)
    // for a more sophisticated solution
    // Note: It really only works for expected, because expected is the only one that isn't requried to
    // be known at the time of combining policies.
    ExpectedIf {
        on_branch: bool,
        of_resolver: ConditionResolver,
    },
}

impl SeparationPolicy {
    pub fn combine_with(
        self,
        other: Self,
    ) -> Self {
        use SeparationPolicy::Allowed;
        use SeparationPolicy::Discouraged;
        use SeparationPolicy::Expected;
        use SeparationPolicy::ExpectedIf;
        use SeparationPolicy::Forced;
        use SeparationPolicy::Ignored;

        match (self, other) {
            //If one side makes no statement, take the other
            (Ignored, other) | (other, Ignored) => other,

            //If one side is forced, it stays forced
            (Forced, _) | (_, Forced) => Forced,

            // If it is not forced, but discouraged, it stays discouraged
            (_, Discouraged) | (Discouraged, _) => Discouraged,

            (Allowed, Allowed) => Allowed,

            // If one side says it is expected and the other says it might be expected => it is expected
            (Expected | ExpectedIf { .. }, Expected)
            | (Expected, ExpectedIf { .. })
            // Expected ranks higher than allowed
            // TODO Currently hen there is an ExpectedIf (whose condition is not met) and an allowed.
            // it will resolve to not being there, instead of being allowed there
            | (Expected | ExpectedIf { .. }, Allowed)
            | (Allowed, Expected | ExpectedIf { .. }) => Expected,
            (ExpectedIf { .. }, ExpectedIf { .. }) => {
                //These would be combined using disjunctions if they are on the true path, and conjunctions on the false path
                todo!("Implement support for disjuctions and conjunctions")
            },
        }
    }
}

/// Request separation between items
///
/// When applying the different kinds of separation are checked in order
/// If a separation with higher precedence is selected, lower precedence separations are ignored (except on conditionals, see below)
/// 1. Empty Lines
/// 2. Line breaks
/// 3. Spaces
///
/// Because dprint's conditionals cannot support this seperation precedence, if one of the separations
/// has an `ExpectIf`, the lower precedence items are processed nevertheless.
/// They are expected to have matching opposite conditionals.
///
//TODO Find a better solution for handling conditionals
#[derive(Default)]
pub struct SeparationRequest {
    pub empty_line: SeparationPolicy,
    pub line_break: SeparationPolicy,
    pub space: SeparationPolicy,
}

impl SeparationRequest {
    pub const fn discouraged() -> Self {
        Self {
            empty_line: SeparationPolicy::Discouraged,
            line_break: SeparationPolicy::Discouraged,
            space: SeparationPolicy::Discouraged,
        }
    }

    pub fn merge(
        self,
        other: Self,
    ) -> Self {
        Self {
            empty_line: self.empty_line.combine_with(other.empty_line),
            line_break: self.line_break.combine_with(other.line_break),
            space: self.space.combine_with(other.space),
        }
    }

    // TODO I really don't like how unclean this function is
    pub fn apply(
        self,
        to: &mut PrintItems,
    ) {
        #[inline]
        fn get_conditional(
            on_branch: bool,
            name: &'static str,
            resolver: ConditionResolver,
            items: PrintItems,
        ) -> Condition {
            if (on_branch) {
                conditions::if_true(name, resolver, items)
            } else {
                conditions::if_false(name, resolver, items)
            }
        }

        fn apply_empty_line(to: &mut PrintItems) {
            to.push_signal(Signal::NewLine);
            to.push_signal(Signal::NewLine);
        }
        fn apply_line_break(to: &mut PrintItems) {
            to.push_signal(Signal::NewLine);
        }
        fn apply_space(to: &mut PrintItems) {
            to.push_space();
        }

        match self.empty_line {
            SeparationPolicy::Forced | SeparationPolicy::Expected => {
                apply_empty_line(to);
                return;
            },
            SeparationPolicy::ExpectedIf {
                on_branch,
                of_resolver,
            } => {
                to.push_condition(get_conditional(
                    on_branch,
                    "empty_line_expected_if",
                    of_resolver,
                    {
                        let mut pi = PrintItems::new();
                        apply_empty_line(&mut pi);
                        pi
                    },
                ));
            },
            SeparationPolicy::Discouraged | SeparationPolicy::Ignored => {},
            SeparationPolicy::Allowed => {
                todo!("Allowed is only valid for line_breaks, separate the cases");
            },
        }
        match self.line_break {
            SeparationPolicy::Forced | SeparationPolicy::Expected => {
                apply_line_break(to);
                return;
            },
            SeparationPolicy::ExpectedIf {
                on_branch,
                of_resolver,
            } => {
                to.push_condition(get_conditional(
                    on_branch,
                    "empty_line_expected_if",
                    of_resolver,
                    {
                        let mut pi = PrintItems::new();
                        apply_line_break(&mut pi);
                        pi
                    },
                ));
            },
            SeparationPolicy::Allowed => {
                to.push_signal(Signal::PossibleNewLine);
            },
            SeparationPolicy::Discouraged | SeparationPolicy::Ignored => {},
        }
        match self.space {
            SeparationPolicy::Forced | SeparationPolicy::Expected => {
                apply_space(to);
                //return; not needed because this is the last item
            },
            SeparationPolicy::ExpectedIf {
                on_branch,
                of_resolver,
            } => {
                to.push_condition(get_conditional(
                    on_branch,
                    "empty_line_expected_if",
                    of_resolver,
                    {
                        let mut pi = PrintItems::new();
                        apply_space(&mut pi);
                        pi
                    },
                ));
            },
            SeparationPolicy::Discouraged | SeparationPolicy::Ignored => {},
            SeparationPolicy::Allowed => {
                todo!("Allowed is only valid for line_breaks, separate the cases");
            },
        }
    }
}

// The motivating example for this is, that there is no obvious way to encode the following rules cleanly into "vanilla" PrintItems
// 1. There should not be a space between the name of a function and the opening parenthesis "fn main("
// 2. A block comment (/* aaa */) should be preceded and followed by a space
// 3. There should not be a space after the opening parenthesis of a function, even if the next token is a block comment
// 4. There should not be a space before the closing parenthesis of a function, even if the preceding token is a block comment
//
// Example formattings: fn main /*aaa*/ (/*bbb*/ param: u32, param2: u32 /*ccc*/)
//
// Considered alternatives:
// * Track if the last pushed item is a space, and branch on that everytime you would add a space
//   * Cons: Very verbose, imperative and brittle ("forget to update the last pushed item"), cannot deal with rule 4 properly.
// * "Cleverly" structure code and where to put spaces, so that these cases are implicitly dealt with
//   * Cons: "Clever" code that doesn't explicitly state intent, and thus is brittle, new requirements might require big restructurings
// * Re-parse the AST into a formatting-ast which tracks comments etc.
//   * We already do that. The formatting-ast is built and immediately destroyed by the parse->format structure that
//     the gen_*_ functions are built with. While it makes the code cleaner, it does not solve the problem.
//
// Chosen solution:
// * Feels like it can most clearly encode the intent behind statements like
//   "add a comma, unless its followed by ')'" or "there should be a single space after 'fn' and before the name"
// * In the formatting code we don't actually care about "what exactly the next or previous token is", instead
//   we wan't to communicate that we may want separation to adjacent text.
/// A wrapper for `PrintItem`s which adds the ability to do "item-requests"
///
/// In a lot of places the intent is to have code of a particular shape, depending on its surroundings.
/// "Add a space, if the previous item was something that we need to separation from"
///
/// All formatting should go through this struct, which keeps track of `PrintItemRequest`s.
/// Example:
/// * Snippet A requests that there should be a space after it `AAA|_|`
/// * Snippet B requests that there should be a space after and in front of it `|_|BBB|_|`
/// * Snippet C requests that there may never be a space in front of it `|X|CCC`
///
/// The `PrintItemBuffer` automatically tracks and resolves these requests, so that the outcome will be
/// `AAA BBBCCC`, where the two spaces between A and B were collapsed and the space after B was overwritten
///
/// Known downsides to this soution:
/// * Does not integrate at all with dprint's conditionals
/// * Another layer ontop of dprint's IR, which doesn't feel like it should be necessary
///
#[derive(Default)]
pub struct PrintItemBuffer {
    pub start_request: Option<SeparationRequest>,
    pub items: PrintItems,
    pub end_request: Option<SeparationRequest>,
}

impl PrintItemBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request(
        &mut self,
        incoming_request: SeparationRequest,
    ) {
        let request_tracker = if self.items.is_empty() {
            &mut self.start_request
        } else {
            &mut self.end_request
        };
        *request_tracker = match request_tracker.take() {
            Some(old_request) => Some(old_request.merge(incoming_request)),
            None => Some(incoming_request),
        };
    }

    pub fn finish(self) -> PrintItems {
        let mut pi = PrintItems::default();
        if let Some(start_request) = self.start_request {
            start_request.apply(&mut pi);
        }
        pi.extend(self.items);
        if let Some(end_request) = self.end_request {
            end_request.apply(&mut pi);
        }
        pi
    }

    fn apply_end_request(&mut self) {
        if let Some(end_request) = self.end_request.take() {
            end_request.apply(&mut self.items);
        }
    }

    // ==== Helper Methods ====

    pub fn extend(
        &mut self,
        other: Self,
    ) {
        // Merge the incoming start_request with our end_request
        if let Some(start_request) = other.start_request {
            self.request(start_request);
        }

        // If there are incoming items, apply the current end request and add the items
        if !other.items.is_empty() {
            self.apply_end_request();
            self.items.extend(other.items);
        }

        // Merge the incoming end_request
        if let Some(end_request) = other.end_request {
            self.request(end_request);
        }
    }

    pub fn request_space(
        &mut self,
        policy: SeparationPolicy,
    ) {
        self.request(SeparationRequest {
            space: policy,
            ..Default::default()
        });
    }

    pub fn request_line_break(
        &mut self,
        policy: SeparationPolicy,
    ) {
        self.request(SeparationRequest {
            line_break: policy,
            ..Default::default()
        });
    }

    pub fn request_empty_line(
        &mut self,
        policy: SeparationPolicy,
    ) {
        self.request(SeparationRequest {
            empty_line: policy,
            ..Default::default()
        });
    }

    pub fn expect_line_break(&mut self) {
        self.request_line_break(SeparationPolicy::Expected);
    }

    pub fn expect_single_space(&mut self) {
        self.request_space(SeparationPolicy::Expected);
    }

    pub fn push_string(
        &mut self,
        string: String,
    ) {
        self.apply_end_request();
        self.items.push_string(string);
    }

    #[deprecated = "Most signals would interact with requests. Use requests instead"]
    pub fn push_signal(
        &mut self,
        signal: Signal,
    ) {
        self.apply_end_request();
        self.items.push_signal(signal);
    }

    pub fn push_sc(
        &mut self,
        sc: &'static dprint_core::formatting::StringContainer,
    ) {
        self.apply_end_request();
        self.items.push_sc(sc);
    }

    pub fn push_info<T: Into<Info>>(
        &mut self,
        info: T,
    ) {
        self.apply_end_request();
        self.items.push_info(info);
    }

    pub fn push_anchor<T: Into<Anchor>>(
        &mut self,
        anchor: T,
    ) {
        self.apply_end_request();
        self.items.push_anchor(anchor);
    }

    pub fn push_condition(
        &mut self,
        condition: dprint_core::formatting::Condition,
    ) {
        self.apply_end_request();
        self.items.push_condition(condition);
    }

    pub fn push_reevaluation(
        &mut self,
        reeval: dprint_core::formatting::ConditionReevaluation,
    ) {
        self.apply_end_request();
        self.items.push_reevaluation(reeval);
    }
}
