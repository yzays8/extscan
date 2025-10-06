#![deny(unsafe_code)]

mod app;
mod detector;
mod error;
mod ffi;
mod magic;
mod scanner;

pub use app::{App, Config};
