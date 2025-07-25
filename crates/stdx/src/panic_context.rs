//! A micro-crate to enhance panic messages with context info.

use std::{cell::RefCell, panic, sync::Once};

/// Dummy for leveraging RAII cleanup to pop frames.
#[must_use]
pub struct PanicContext {
    // prevent arbitrary construction
    _priv: (),
}

impl Drop for PanicContext {
    fn drop(&mut self) {
        with_ctx(|context| assert!(context.pop().is_some()));
    }
}

pub fn enter(frame: String) -> PanicContext {
    #[expect(clippy::print_stderr, reason = "not relevant here")]
    fn set_hook() {
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            with_ctx(|context| {
                if !context.is_empty() {
                    eprintln!("Panic context:");
                    for frame in context.iter() {
                        eprintln!("> {frame}\n");
                    }
                }
            });
            default_hook(panic_info);
        }));
    }

    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(set_hook);

    with_ctx(|context| context.push(frame));
    PanicContext { _priv: () }
}

fn with_ctx(function: impl FnOnce(&mut Vec<String>)) {
    thread_local! {
        static CTX: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    }
    CTX.with(|context| function(&mut context.borrow_mut()));
}
