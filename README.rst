Error Context
#############


Overview
========

A library for providing context information for error messages, written in Rust.

This is attempt to write ErrorContext from awesome `bitsquid blog post`_ in Rust.


Usage
=====

See example.rs ::

    #![feature(phase)]

    #[phase(syntax, link)]
    extern crate error_context;

    fn main() {
        error_context::ErrorContext::init();
        set_context!("loading level", "level1");
        {
            set_context!("loading model", "tank.model");
            {
                set_context!("loading texture", "tank.png");
                {
                    // ...
                    context_fail!("Can`t parse integer: {}", 1);
                }
            }
        }
    }

Output ::

    When loading level: level1
    When loading model: tank.model
    When loading texture: tank.png
    task '<main>' failed at 'Can`t parse integer: 1', example.rs:17


Contribute
==========

Feel free to report bugs and patches using GitHub's pull requests
system on `ozkriff/error-context`_.  Any feedback would be much appreciated!


License
=======

error-context is licensed under the MIT license (see the "LICENSE" file).

.. _`ozkriff/error-context`: https://github.com/ozkriff/error-context
.. _`bitsquid blog post`: http://bitsquid.blogspot.ru/2012/01/sensible-error-handling-part-1.html
