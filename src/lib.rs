#![feature(conservative_impl_trait)]

#[macro_use]
extern crate combine;

mod parser;
#[cfg(test)]
mod tests;
mod types;

pub use types::*;
