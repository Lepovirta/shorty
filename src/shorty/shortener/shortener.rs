/*
 * Defines common functions for implementing a Shortener.
 * new: provide initialization
 * next: return next short id
 */
pub trait Shortener {
    fn new(len: usize) -> Self;
    fn next(&mut self) -> String;
}
