# See LICENSE file for copyright and license details.

all: src/error_context.rs
	rustc --crate-type=lib src/error_context.rs
	rustc src/bin/example.rs -L .

# vim: set tabstop=4 shiftwidth=4 softtabstop=4:
