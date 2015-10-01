#![feature(fnbox)]

#[macro_use]
extern crate log;

use std::boxed::FnBox;
use std::fmt::Display;

/// Just an FnOnce closure that logs the start, end and the error (if it occurs).
/// The closure is of form FnOnce(I) -> Result<O, E>
///
/// Returns a Box&lt;FnBox&gt;
///
/// * Box so that it's Sized
/// * FnOnce so that it can be called in a box
///
/// This makes it only usable on nightly. Maybe I'll provide a named_mut() that uses FnMut
///
/// S: Type of the name to log
/// F: Type of the closure to run
/// I, O, E: Type of Input, Output and Error of the Closure
pub fn named<'a, S, F, I, O, E>(name: S, f: F) -> Box<FnBox(I) -> Result<O, E> + 'a>
    where S: Display + Sized + 'a,
        F: 'a + FnOnce(I) -> Result<O, E>,
        E: Display {
        
    Box::new(move | input: I | {
        info!("Running task '{}'", &name);
        let result = f(input);
        match result {
            Ok(_) => info!("Done: '{}'", name),
            Err(ref e) => info!("FAILED task '{}' with error: '{}'", &name, e)
        }
        result
    })
}
