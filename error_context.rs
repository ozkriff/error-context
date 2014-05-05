// See LICENSE file for copyright and license details.

// http://bitsquid.blogspot.ru/2012/01/sensible-error-handling-part-1.html

#![feature(macro_rules)]

use std::cast::transmute;
use std::local_data;

#[macro_export]
macro_rules! context_fail (
    ($($e: expr),+) => ({
        error_context::ErrorContext::print_contexts();
        fail!($($e),+);
    })
)

#[macro_export]
macro_rules! set_context (
    ($name: expr, $data: expr) => (
        let _ec = error_context::ErrorContext::new($name, $data);
    )
)

fn store_str(s: &&str) -> uint {
    unsafe {
        transmute::<&&str, uint>(s)
    }
}

fn get_str(n: uint) -> &str {
    unsafe {
        *transmute::<uint, &&str>(n)
    }
}

struct ContextInfo {
    name: uint,
    data: uint,
}

local_data_key!(CONTEXTS: Vec<ContextInfo>)

pub struct ErrorContext;

impl ErrorContext {
    pub fn init() {
        local_data::get_mut(CONTEXTS, |contexts| {
            assert!(contexts.is_none());
        });
        local_data::set(CONTEXTS, Vec::new());
    }

    pub fn new(name: &str, data: &str) -> ErrorContext {
        local_data::get_mut(CONTEXTS, |contexts| {
            if contexts.is_none() {
                fail!("Contexts not initialized");
            }
            contexts.unwrap().push(ContextInfo {
                name: store_str(&name),
                data: store_str(&data),
            });
        });
        ErrorContext
    }

    pub fn print_contexts() {
        local_data::get(CONTEXTS, |contexts| {
            if contexts.is_none() {
                fail!("Contexts not initialized");
            }
            for context in contexts.unwrap().iter() {
                let name = get_str(context.name);
                let data = get_str(context.data);
                println!("When {}: {}", name, data);
            }
        });
    }
}

impl Drop for ErrorContext {
    fn drop(&mut self) {
        local_data::get_mut(CONTEXTS, |contexts| {
            contexts.unwrap().pop();
        });
    }
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab:
