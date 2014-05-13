// See LICENSE file for copyright and license details.

// http://bitsquid.blogspot.ru/2012/01/sensible-error-handling-part-1.html

#![feature(macro_rules)]

use std::mem::transmute;
use std::cell;

#[macro_export]
macro_rules! set_error_context (
    ($name: expr, $data: expr) => (
        let _ec = error_context::ErrorContext::new($name, $data);
    )
)

// ErrorContext version without destructor for storing in global array
struct ErrorContextInternal {
    description: &'static str,
    data: &'static str,
}

local_data_key!(CONTEXTS: cell::RefCell<Vec<ErrorContextInternal>>)

pub struct ErrorContext<'a> {
    description: &'static str,
    data: &'a str,
}

impl<'a> ErrorContext<'a> {
    #[inline]
    pub fn new<'a>(description: &'static str, data: &'a str) -> ErrorContext<'a> {
        if !is_initialized() {
            init();
        }
        match CONTEXTS.get() {
            Some(contexts) => {
                contexts.borrow_mut().push(ErrorContextInternal {
                    description: description,
                    data: unsafe {
                        transmute::<&'a str, &'static str>(data)
                    },
                });
            }
            None => fail!("Contexts can not be None"),
        }
        ErrorContext {
            description: description,
            data: data,
        }
    }
}

#[unsafe_destructor]
impl<'a> Drop for ErrorContext<'a> {
    fn drop(&mut self) {
        if std::task::failing() {
            on_task_fail();
        } else {
            match CONTEXTS.get() {
                Some(contexts) => {
                    contexts.borrow_mut().pop();
                },
                None => {},
            }
        }
    }
}

fn init() {
    assert!(!is_initialized());
    CONTEXTS.replace(Some(cell::RefCell::new(Vec::new())));
}

#[inline]
fn is_initialized() -> bool {
    CONTEXTS.get().is_some()
}

fn print_contexts() {
    match CONTEXTS.get() {
        Some(contexts) => {
            for context in contexts.borrow().iter() {
                println!("When {}: {}", context.description, context.data);
            }
        },
        None => fail!("Contexts can not be None"),
    }
}

fn on_task_fail() {
    print_contexts();
    match CONTEXTS.get() {
        Some(contexts) => contexts.borrow_mut().clear(),
        None => fail!("Contexts can not be None"),
    }
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab:
