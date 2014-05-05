// See LICENSE file for copyright and license details.

#![feature(phase)]

#[phase(syntax, link)]
extern crate error_context;

fn main() {
    error_context::ErrorContext::init();
    set_context!("loading level", "level1");
    set_context!("loading chunk", "chunk_02008");
    {
        set_context!("loading model", "tank.model");
        set_context!("loading texture", "tank.png");
        {
            // ...
            context_fail!("Can`t parse integer: {}", 1);
        }
    }
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab:
