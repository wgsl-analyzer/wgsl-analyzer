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
    /// Converts the RequestItem to its index in the Bitmaps that store expected, discouraged, and forced requests.
    /// If multiple request items are requested at a stage (e.g expect space & line break), the request item with
    /// the highest index is used.
    #[must_use]
    pub const fn to_index(self) -> u8 {
        match self {
            RequestItem::Space => 0,
            RequestItem::LineBreak => 1,
            RequestItem::EmptyLine => 2,
        }
    }

    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(RequestItem::Space),
            1 => Some(RequestItem::LineBreak),
            2 => Some(RequestItem::EmptyLine),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct RequestItemMap(u8);

impl RequestItemMap {
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
        RequestItem::from_index(log as u8)
    }
    #[must_use]
    pub const fn extended_by(
        self,
        other: RequestItem,
    ) -> Self {
        self.union(&Self::from(other))
    }
}

// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// pub enum RequestPolicy {
//     Forced,
//     Discouraged,
//     Expected,
// }

// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// pub struct RequestSubject {
//     item: RequestItem,
//     policy: RequestPolicy,
// }

#[derive(Clone)]
pub enum Request {
    Unconditional {
        // TODO use bitmaps instead of sets here
        expected: RequestItemMap,
        discouraged: RequestItemMap,
        forced: RequestItemMap,

        // TODO think the exact theory of this through
        suggest_linebreak: bool,
    },
    Conditional {
        condition: ConditionResolver,
        on_true: Box<RequestFolder>,
        on_false: Box<RequestFolder>,
    },
}

impl Request {
    pub fn empty() -> Self {
        Self::Unconditional {
            expected: RequestItemMap::empty(),
            discouraged: RequestItemMap::empty(),
            forced: RequestItemMap::empty(),
            suggest_linebreak: false,
        }
    }

    pub fn expect(item: RequestItem) -> Self {
        Self::Unconditional {
            expected: RequestItemMap::from(item),
            discouraged: RequestItemMap::empty(),
            forced: RequestItemMap::empty(),
            suggest_linebreak: false,
        }
    }

    pub fn discourage(item: RequestItem) -> Self {
        Self::Unconditional {
            expected: RequestItemMap::empty(),
            discouraged: RequestItemMap::from(item),
            forced: RequestItemMap::empty(),
            suggest_linebreak: false,
        }
    }

    pub fn force(item: RequestItem) -> Self {
        Self::Unconditional {
            expected: RequestItemMap::empty(),
            discouraged: RequestItemMap::empty(),
            forced: RequestItemMap::from(item),
            suggest_linebreak: false,
        }
    }

    pub fn or_newline(self) -> Self {
        //TODO Redesign requests once again
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
            Self::Conditional { .. } => todo!(),
        }
    }

    // ==== Request Logic ====
    pub fn combine(
        left: Self,
        right: Self,
    ) -> Self {
        match (left, right) {
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

            (
                request_left,
                Self::Conditional {
                    condition,
                    mut on_true,
                    mut on_false,
                },
            ) => {
                on_true.push_left(request_left.clone());
                on_false.push_left(request_left);

                Self::Conditional {
                    condition,
                    on_true,
                    on_false,
                }
            },
            (
                Self::Conditional {
                    condition,
                    mut on_true,
                    mut on_false,
                },
                request_right,
            ) => {
                on_true.push_left(request_right.clone());
                on_false.push_left(request_right);

                Self::Conditional {
                    condition,
                    on_true,
                    on_false,
                }
            },
        }
    }
}

//TODO This whole api is still a little bit clunky
//TODO Make sure push_left is really needed, because so far RequestFolder merging is commutative, lets see if its still commutative after im done
/// A structure to fold multiple requests into a single one.
#[derive(Default, Clone)]
pub struct RequestFolder {
    pub folded_request: Option<Request>,
}

impl RequestFolder {
    pub fn push_left(
        &mut self,
        request: Request,
    ) {
        if let Some(old_request) = self.folded_request.take() {
            self.folded_request = Some(Request::combine(old_request, request));
        } else {
            self.folded_request = Some(request);
        }
    }

    // pub fn push_right(
    //     &mut self,
    //     request: Request,
    // ) {
    //     if let Some(old_request) = self.folded_request.take() {
    //         self.folded_request = Some(Request::combine(request, old_request));
    //     } else {
    //         self.folded_request = Some(request);
    //     }
    // }

    pub fn append(
        &mut self,
        mut other: Self,
    ) {
        if let Some(new_request) = other.folded_request.take() {
            if let Some(old_request) = self.folded_request.take() {
                self.folded_request = Some(Request::combine(old_request, new_request));
            } else {
                self.folded_request = Some(new_request);
            }
        }
    }

    pub fn resolve(
        &mut self,
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

        if let Some(request) = self.folded_request.take() {
            match request {
                Request::Unconditional {
                    expected,
                    discouraged,
                    forced,
                    suggest_linebreak,
                } => {
                    let candidates = expected.difference(&discouraged);
                    let candidates = candidates.union(&forced);

                    //TODO if newlines are discouraged, clear suggest_linebreak

                    if let Some(chosen) = candidates.highest_index() {
                        apply_item(chosen, target, suggest_linebreak);
                    } else if suggest_linebreak {
                        target.push_signal(Signal::PossibleNewLine);
                    }
                },
                Request::Conditional {
                    condition,
                    mut on_true,
                    mut on_false,
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
}

impl From<Request> for RequestFolder {
    fn from(value: Request) -> Self {
        Self {
            folded_request: Some(value),
        }
    }
}
