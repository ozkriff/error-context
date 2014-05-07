// See LICENSE file for copyright and license details.

#![feature(phase)]

#[phase(syntax, link)]
extern crate error_context;

fn main() {
    set_error_context!("loading level", "level1");
    {
        set_error_context!("loading model", "tank.model");
        {
            set_error_context!("loading texture", "tank.png");
            {
                // ...
                fail!("Can`t parse integer: {}", 1);
            }
        }
    }
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab:
