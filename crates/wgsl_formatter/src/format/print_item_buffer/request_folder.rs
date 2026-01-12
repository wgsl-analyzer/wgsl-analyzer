use std::collections::BTreeSet;

use dprint_core::formatting::{
    Condition, ConditionProperties, ConditionResolver, PrintItems, Signal,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum RequestItem {
    Space = 1,
    SpaceOrNewline = 2,
    LineBreak = 3,
    EmptyLine = 4,
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
        expected: BTreeSet<RequestItem>,
        discouraged: BTreeSet<RequestItem>,
        forced: BTreeSet<RequestItem>,
    },
    Conditional {
        condition: ConditionResolver,
        on_true: Box<RequestFolder>,
        on_false: Box<RequestFolder>,
    },
}

impl Request {
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
                },
                Self::Unconditional {
                    expected: exp_right,
                    discouraged: disc_right,
                    forced: forced_right,
                },
            ) => {
                let mut combined_exp = exp_left;
                combined_exp.extend(exp_right.iter());

                let mut combined_disc = disc_left;
                combined_disc.extend(disc_right.iter());

                let mut combined_forced = forced_left;
                combined_forced.extend(forced_right.iter());

                Self::Unconditional {
                    expected: combined_exp,
                    discouraged: combined_disc,
                    forced: combined_forced,
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
        ) {
            match item {
                RequestItem::Space => {
                    target.push_space();
                },
                RequestItem::SpaceOrNewline => {
                    target.push_signal(Signal::SpaceOrNewLine);
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
                } => {
                    let candidates = expected
                        .difference(&discouraged)
                        .copied()
                        .collect::<BTreeSet<_>>();
                    let candidates = candidates.union(&forced).collect::<BTreeSet<_>>();

                    if let Some(chosen) = candidates.last() {
                        apply_item(**chosen, target);
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
