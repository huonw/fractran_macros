#![no_std]

extern crate core;

/// The interface provided by a compiled `Fractran` program.
///
/// Use the `fractran!` macro to create a constructor.
pub trait Fractran: core::iter::Iterator<()> {
    fn state<'a>(&'a self) -> &'a [u32];

    /// Run the program to completion, returning the internal state.
    fn run<'a>(&'a mut self) -> &'a [u32] {
        for _ in *self {}

        self.state()
    }
}
