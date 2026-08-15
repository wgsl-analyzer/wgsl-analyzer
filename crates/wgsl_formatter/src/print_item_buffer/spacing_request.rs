use std::fmt::Debug;

use dprint_core::formatting::{
    Condition, ConditionProperties, ConditionResolver, PrintItems, Signal,
};

/// A possible kind of whitespace that can be requested and, through [`RequestFolder`], be merged together if multiple requests are issued.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RequestItem {
    Space,
    LineBreak,
    EmptyLine,
}

impl RequestItem {
    /// Converts the [`RequestItem`] to its index in the [`RequestItemSet`]s that store expected, discouraged, and forced requests.
    /// If multiple request items are requested at a stage (e.g expect space & line break), the request item with
    /// the highest index is used.
    #[must_use]
    pub const fn to_index(self) -> u8 {
        match self {
            Self::Space => 0,
            Self::LineBreak => 1,
            Self::EmptyLine => 2,
        }
    }

    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Space),
            1 => Some(Self::LineBreak),
            2 => Some(Self::EmptyLine),
            _ => None,
        }
    }
}

/// A Set holding [`RequestItems`], implemented via a bitmap.
#[derive(Clone)]
pub struct RequestItemSet(u8);

impl Debug for RequestItemSet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut set = f.debug_set();
        for index in 0..7 {
            let bit = 1_u8 << index;
            if self.0 & bit != 0 {
                set.entry(&RequestItem::from_index(index));
            }
        }
        set.finish()?;
        Ok(())
    }
}

impl RequestItemSet {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }
    #[must_use]
    pub const fn from(item: RequestItem) -> Self {
        Self(1 << item.to_index())
    }
    #[must_use]
    pub const fn union(
        &self,
        other: &Self,
    ) -> Self {
        Self(self.0 | other.0)
    }
    #[must_use]
    pub const fn difference(
        &self,
        other: &Self,
    ) -> Self {
        Self(self.0 & !(other.0))
    }
    #[must_use]
    pub const fn highest_index(&self) -> Option<RequestItem> {
        if self.0 == 0 {
            return None;
        }
        let log = self.0.ilog2();

        #[expect(
            clippy::cast_possible_truncation,
            reason = "ilog2 of u8 can never exceed u8"
        )]
        #[expect(clippy::as_conversions, reason = "keep it const")]
        RequestItem::from_index(log as u8)
    }

    #[must_use]
    pub const fn contains(
        self,
        item: RequestItem,
    ) -> bool {
        self.0 & (1 << item.to_index()) != 0
    }
}

/// A structure that marks whether a particular [`RequestItem`] is expected, discouraged or forced - unconditionally or under some condition.
///
/// An "expected" [`RequestItem`] will be put into the output, unless it is "discouraged".
/// A "forced" [`RequestItem`] will be put into the output, regardless of if it is "discouraged".
/// If multiple [`RequestItem`] would be eligible to be put into the output,
/// only the "biggest" one (the one that subsumes all the other ones) is actually put into the output.
///
/// Optionally a request can also "suggest" a newline, which means that if a space or nothing at all would be put into the output,
/// and a newline is not "discouraged" at this point, then either a [`SpaceOrNewline`] or a [`PossibleNewline`] is output.
#[derive(Clone)]
pub enum Request {
    Unconditional {
        expected: RequestItemSet,
        discouraged: RequestItemSet,
        forced: RequestItemSet,

        // Biggest Expected Item:
        //  - Nothing: Ok
        //  - Space: Ok
        //  - Newline: Clear suggest_linebreak
        //  - EmptyLine: Clear suggest_linebreak
        //
        // Biggest Discouraged Item:
        //  - Nothing: Ok
        //  - Space: Ok
        //  - Newline: Clear suggest_linebreak
        //  - EmptyLine: Clear suggest_linebreak
        //
        // Biggest Forced Item:
        //  - Nothing: Ok
        //  - Space: Ok
        //  - Newline: Clear suggest_linebreak
        //  - EmptyLine: Clear suggest_linebreak
        //
        // If suggest_linebreak persists through expected, discouraged, forced
        // Then it is turned into either a SpaceOrNewline or PossibleNewLine
        suggest_linebreak: bool,
    },
    Conditional {
        condition: ConditionResolver,
        on_true: Box<Self>,
        on_false: Box<Self>,
    },
}

impl Default for Request {
    fn default() -> Self {
        Self::empty()
    }
}

impl Request {
    #[must_use]
    pub const fn empty() -> Self {
        Self::Unconditional {
            expected: RequestItemSet::empty(),
            discouraged: RequestItemSet::empty(),
            forced: RequestItemSet::empty(),
            suggest_linebreak: false,
        }
    }

    #[must_use]
    pub const fn expect(item: RequestItem) -> Self {
        Self::Unconditional {
            expected: RequestItemSet::from(item),
            discouraged: RequestItemSet::empty(),
            forced: RequestItemSet::empty(),
            suggest_linebreak: false,
        }
    }

    #[must_use]
    pub const fn discourage(item: RequestItem) -> Self {
        Self::Unconditional {
            expected: RequestItemSet::empty(),
            discouraged: RequestItemSet::from(item),
            forced: RequestItemSet::empty(),
            suggest_linebreak: false,
        }
    }

    #[must_use]
    pub const fn force(item: RequestItem) -> Self {
        Self::Unconditional {
            expected: RequestItemSet::empty(),
            discouraged: RequestItemSet::empty(),
            forced: RequestItemSet::from(item),
            suggest_linebreak: false,
        }
    }

    #[must_use]
    pub fn or_newline(self) -> Self {
        match self {
            Self::Unconditional {
                expected,
                discouraged,
                forced,
                suggest_linebreak: _,
            } => Self::Unconditional {
                expected,
                discouraged,
                forced,
                suggest_linebreak: true,
            },
            Self::Conditional {
                condition,
                on_false,
                on_true,
            } => Self::Conditional {
                condition,
                on_true: Box::new(on_true.or_newline()),
                on_false: Box::new(on_false.or_newline()),
            },
        }
    }

    // ==== Request Logic ====
    /// Create a request that is the combination of left and right.
    ///
    /// This is commutative with respect to the outcome, so order of left and right does not matter.
    /// However - when using Conditional Requests, order of left and right will determine how the conditions are combined
    /// (but in practice that should not have any implications).
    #[must_use]
    pub fn combine(
        left: Self,
        right: Self,
    ) -> Self {
        #[expect(
            clippy::match_same_arms,
            reason = "We want to explicitly enumerate the important cases to make commutativty apparent"
        )]
        match (left, right) {
            // COMMUTATIVITY: union is commutative, || is commutative
            (
                Self::Unconditional {
                    expected: exp_left,
                    discouraged: disc_left,
                    forced: forced_left,
                    suggest_linebreak: left_potential_newline,
                },
                Self::Unconditional {
                    expected: exp_right,
                    discouraged: disc_right,
                    forced: forced_right,
                    suggest_linebreak: right_potential_newline,
                },
            ) => {
                let combined_exp = exp_left.union(&exp_right);
                let combined_disc = disc_left.union(&disc_right);
                let combined_forced = forced_left.union(&forced_right);

                Self::Unconditional {
                    expected: combined_exp,
                    discouraged: combined_disc,
                    forced: combined_forced,
                    suggest_linebreak: left_potential_newline || right_potential_newline,
                }
            },

            // COMMUTATIVITY: This is not 100% commutative, however it does not matter as the result is the same
            // The structure looks like this - and here it does not matter if you reverse the ordering of the ifs
            // If right_cond {
            //    if left_cond { combine(left_true, right_true) } else { combine(left_false, right_true) }
            // } else {
            //    if left_cond { combine(left_true, right_false) } else { combine(left_false, right_false) }
            // }
            (
                request_left @ Self::Conditional { .. },
                Self::Conditional {
                    condition,
                    on_true,
                    on_false,
                },
            ) => Self::Conditional {
                condition,
                on_true: Box::new(Self::combine(request_left.clone(), *on_true)),
                on_false: Box::new(Self::combine(request_left, *on_false)),
            },

            // COMMUTATIVITY: The body of this Conditional is the same as with the arguments flipped.
            // The only difference is the order of arguments to Request::combine, but Request::combine is commutative.
            (
                request_left,
                Self::Conditional {
                    condition,
                    on_true,
                    on_false,
                },
            ) => Self::Conditional {
                condition,
                on_true: Box::new(Self::combine(request_left.clone(), *on_true)),
                on_false: Box::new(Self::combine(request_left, *on_false)),
            },
            // COMMUTATIVITY: The body of this Conditional is the same as with the arguments flipped.
            // The only difference is the order of arguments to Request::combine, but Request::combine is commutative.
            (
                Self::Conditional {
                    condition,
                    on_true,
                    on_false,
                },
                request_right,
            ) => Self::Conditional {
                condition,
                on_true: Box::new(Self::combine(*on_true, request_right.clone())),
                on_false: Box::new(Self::combine(*on_false, request_right)),
            },
        }
    }

    /// Evaluate this [`Request`] and append its output to the given [`PrintItems`].
    pub fn resolve(
        self,
        target: &mut PrintItems,
    ) {
        fn apply_item(
            item: RequestItem,
            target: &mut PrintItems,
            suggest_newline: bool,
        ) {
            match item {
                RequestItem::Space => {
                    if suggest_newline {
                        target.push_signal(Signal::SpaceOrNewLine);
                    } else {
                        target.push_signal(Signal::SpaceIfNotTrailing);
                    }
                },
                RequestItem::LineBreak => {
                    target.push_signal(Signal::NewLine);
                },
                RequestItem::EmptyLine => {
                    target.push_signal(Signal::NewLine);
                    target.push_signal(Signal::NewLine);
                },
            }
        }

        match self {
            Self::Unconditional {
                expected,
                discouraged,
                forced,
                suggest_linebreak,
            } => {
                let candidates = expected.difference(&discouraged);
                let candidates = candidates.union(&forced);

                // if newlines are discouraged, clear suggest_linebreak
                let suggest_linebreak =
                    suggest_linebreak && !discouraged.contains(RequestItem::LineBreak);

                if let Some(chosen) = candidates.highest_index() {
                    apply_item(chosen, target, suggest_linebreak);
                } else if suggest_linebreak {
                    target.push_signal(Signal::PossibleNewLine);
                }
            },
            Self::Conditional {
                condition,
                on_true,
                on_false,
            } => {
                target.push_condition(Condition::new(
                    "request_conditional",
                    ConditionProperties {
                        condition,
                        true_path: {
                            let mut pi = PrintItems::new();
                            on_true.resolve(&mut pi);
                            Some(pi)
                        },
                        false_path: {
                            let mut pi = PrintItems::new();
                            on_false.resolve(&mut pi);
                            Some(pi)
                        },
                    },
                ));
            },
        }
    }
}
