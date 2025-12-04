pub mod request_folder;

use std::collections::BTreeSet;

use dprint_core::formatting::{
    Anchor, Condition, ConditionResolver, Info, PrintItem, PrintItems, Signal, conditions,
};
use dprint_core_macros::sc;

use crate::format::print_item_buffer::request_folder::{Request, RequestFolder, RequestItem};

#[derive(Default)]
#[deprecated(note = "There should be a better api to directly request request_fold's requestitems")]
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

// TODO Remove this API and replace it with a better one, once i know that the RequestFolder api works and its worth it to refactor all invocations of SeparationRequests
#[derive(Default)]
#[deprecated(note = "There should be a better api to directly request request_fold's requestitems")]
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
/// Known downsides to this solution:
/// * Exponential blowup when using with dprint's conditionals (not a big problem most of the time as not many dprint conditionals are used consecutively)
/// * Another layer on top of dprint's IR, which doesn't feel like it should be necessary
///
#[derive(Default)]
pub struct PrintItemBuffer {
    pub start_request: RequestFolder,
    pub items: PrintItems,
    pub end_request: RequestFolder,
}

impl PrintItemBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request_folder(
        &mut self,
        incoming_request: RequestFolder,
    ) {
        let request_tracker = if self.items.is_empty() {
            &mut self.start_request
        } else {
            &mut self.end_request
        };
        request_tracker.append(incoming_request);
    }

    pub fn request_request(
        &mut self,
        incoming_request: Request,
    ) {
        self.request_folder(RequestFolder {
            folded_request: Some(incoming_request),
        });
    }

    #[deprecated]
    pub fn request(
        &mut self,
        incoming_request: SeparationRequest,
    ) {
        fn conditional(
            of_resolver: ConditionResolver,
            expect_on_true: Option<RequestItem>,
            expect_on_false: Option<RequestItem>,
        ) -> Request {
            Request::Conditional {
                condition: of_resolver,
                on_true: Box::new(RequestFolder {
                    folded_request: Some(Request::Unconditional {
                        expected: BTreeSet::from_iter(expect_on_true.into_iter()),
                        discouraged: BTreeSet::new(),
                        forced: BTreeSet::new(),
                    }),
                }),
                on_false: Box::new(RequestFolder {
                    folded_request: Some(Request::Unconditional {
                        expected: BTreeSet::from_iter(expect_on_false.into_iter()),
                        discouraged: BTreeSet::new(),
                        forced: BTreeSet::new(),
                    }),
                }),
            }
        }

        match incoming_request.empty_line {
            SeparationPolicy::Forced => {
                self.request_request(Request::Unconditional {
                    forced: BTreeSet::from([RequestItem::EmptyLine]),
                    discouraged: BTreeSet::new(),
                    expected: BTreeSet::new(),
                });
            },
            SeparationPolicy::Expected => {
                self.request_request(Request::Unconditional {
                    expected: BTreeSet::from([RequestItem::EmptyLine]),
                    discouraged: BTreeSet::new(),
                    forced: BTreeSet::new(),
                });
            },
            SeparationPolicy::Allowed => todo!(),
            SeparationPolicy::Discouraged => {
                self.request_request(Request::Unconditional {
                    expected: BTreeSet::new(),
                    discouraged: BTreeSet::from([RequestItem::EmptyLine]),
                    forced: BTreeSet::new(),
                });
            },
            SeparationPolicy::Ignored => {},
            SeparationPolicy::ExpectedIf {
                on_branch,
                of_resolver,
            } => {
                if on_branch {
                    self.request_request(conditional(
                        of_resolver,
                        Some(RequestItem::EmptyLine),
                        None,
                    ));
                } else {
                    self.request_request(conditional(
                        of_resolver,
                        None,
                        Some(RequestItem::EmptyLine),
                    ));
                }
            },
        }

        match incoming_request.space {
            SeparationPolicy::Forced => {
                self.request_request(Request::Unconditional {
                    forced: BTreeSet::from([RequestItem::Space]),
                    discouraged: BTreeSet::new(),
                    expected: BTreeSet::new(),
                });
            },
            SeparationPolicy::Expected => {
                self.request_request(Request::Unconditional {
                    expected: BTreeSet::from([RequestItem::Space]),
                    discouraged: BTreeSet::new(),
                    forced: BTreeSet::new(),
                });
            },
            SeparationPolicy::Allowed => todo!(),
            SeparationPolicy::Discouraged => {
                self.request_request(Request::Unconditional {
                    expected: BTreeSet::new(),
                    discouraged: BTreeSet::from([RequestItem::Space]),
                    forced: BTreeSet::new(),
                });
            },
            SeparationPolicy::Ignored => {},
            SeparationPolicy::ExpectedIf {
                on_branch,
                of_resolver,
            } => {
                if on_branch {
                    self.request_request(conditional(of_resolver, Some(RequestItem::Space), None));
                } else {
                    self.request_request(conditional(of_resolver, None, Some(RequestItem::Space)));
                }
            },
        }

        match incoming_request.line_break {
            SeparationPolicy::Forced => {
                self.request_request(Request::Unconditional {
                    expected: BTreeSet::new(),
                    discouraged: BTreeSet::new(),
                    forced: BTreeSet::from([RequestItem::LineBreak]),
                });
            },
            SeparationPolicy::Expected => {
                self.request_request(Request::Unconditional {
                    expected: BTreeSet::from([RequestItem::LineBreak]),
                    discouraged: BTreeSet::new(),
                    forced: BTreeSet::new(),
                });
            },
            SeparationPolicy::Allowed => {
                self.request_request(Request::Unconditional {
                    expected: BTreeSet::from([RequestItem::SpaceOrNewline]),
                    discouraged: BTreeSet::new(),
                    forced: BTreeSet::new(),
                });
            },
            SeparationPolicy::Discouraged => {
                self.request_request(Request::Unconditional {
                    expected: BTreeSet::new(),
                    discouraged: BTreeSet::from([RequestItem::LineBreak]),
                    forced: BTreeSet::new(),
                });
            },
            SeparationPolicy::Ignored => {},
            SeparationPolicy::ExpectedIf {
                on_branch,
                of_resolver,
            } => {
                if on_branch {
                    self.request_request(conditional(
                        of_resolver,
                        Some(RequestItem::LineBreak),
                        None,
                    ));
                } else {
                    self.request_request(conditional(
                        of_resolver,
                        None,
                        Some(RequestItem::LineBreak),
                    ));
                }
            },
        }
    }

    pub fn finish(mut self) -> PrintItems {
        let mut pi = PrintItems::default();
        self.start_request.resolve(&mut pi);
        pi.extend(self.items);
        self.end_request.resolve(&mut pi);
        pi
    }

    fn apply_end_request(&mut self) {
        self.end_request.resolve(&mut self.items);
    }

    // ==== Helper Methods ====

    pub fn extend(
        &mut self,
        other: Self,
    ) {
        // Merge the incoming start_request
        self.request_folder(other.start_request);

        // If there are incoming items, apply the current end request and add the items
        if !other.items.is_empty() {
            self.apply_end_request();
            self.items.extend(other.items);
        }

        // Merge the incoming end_request
        self.request_folder(other.end_request);
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
        #[cfg(feature = "prefer-immediate-crash")]
        {
            if string.contains("\n") {
                panic!("Cannot push string with newlines to PrintItemBuffer {string:?}");
            }
        }
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
