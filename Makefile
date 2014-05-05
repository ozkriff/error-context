# See LICENSE file for copyright and license details.

all: error_context.rs
	rustc --crate-type=lib error_context.rs
	rustc example.rs -L .

# vim: set tabstop=4 shiftwidth=4 softtabstop=4:
