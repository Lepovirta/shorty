/*
 * Defines common functions for implementing a Shortener.
 * new: provide initialization
 * next: return next short id
 */
pub trait Shortener: Sync + Send {
    fn next(&mut self) -> String;
}
