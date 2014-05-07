// See LICENSE file for copyright and license details.

// http://bitsquid.blogspot.ru/2012/01/sensible-error-handling-part-1.html

#![feature(macro_rules)]

use std::cast::transmute;
use std::local_data;

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

local_data_key!(CONTEXTS: Vec<ErrorContextInternal>)

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
        local_data::get_mut(CONTEXTS, |contexts| {
            contexts.unwrap().push(ErrorContextInternal {
                description: description,
                data: unsafe {
                    transmute::<&'a str, &'static str>(data)
                },
            });
        });
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
            local_data::get_mut(CONTEXTS, |contexts| {
                contexts.unwrap().pop();
            });
        }
    }
}

fn init() {
    local_data::get_mut(CONTEXTS, |contexts| {
        assert!(contexts.is_none());
    });
    local_data::set(CONTEXTS, Vec::new());
}

#[inline]
fn is_initialized() -> bool {
    local_data::get(CONTEXTS, |contexts| {
        contexts.is_some()
    })
}

fn print_contexts() {
    local_data::get(CONTEXTS, |contexts| {
        if contexts.is_some() {
            for context in contexts.unwrap().iter() {
                println!("When {}: {}", context.description, context.data);
            }
        }
    });
}

fn on_task_fail() {
    print_contexts();
    local_data::get_mut(CONTEXTS, |contexts| {
        if contexts.is_some() {
            contexts.unwrap().clear();
        }
    });
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab:
