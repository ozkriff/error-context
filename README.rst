Error Context
#############


Overview
========

This is attampt to write ErrorContext from `blog post`_ in Rust.


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


Contribute
==========

Feel free to report bugs and patches using GitHub's pull requests
system on `ozkriff/error-context`_.  Any feedback would be much appreciated!


License
=======

error-context is licensed under the MIT license (see the "LICENSE" file).

.. _`ozkriff/error-context`: https://github.com/ozkriff/error-context
.. _`blog post`: http://bitsquid.blogspot.ru/2012/01/sensible-error-handling-part-1.html
