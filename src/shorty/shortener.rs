use harsh::{Harsh, HarshBuilder};


/*
 * Defines common functions for implementing a Shortener.
 * new: provide initialization
 * next: return next short id
 */
pub trait Shortener {
    fn new(len: usize) -> Self;
    fn next(&mut self) -> String;
}


/*
 * Example implementation of a Shortener using Harsh library.
 */
pub struct HarshShortener {
    id: u64,
    generator: Harsh,
}

impl Shortener for HarshShortener {
    fn new(len: usize) -> HarshShortener {
        HarshShortener {
            id: 0,
            generator: HarshBuilder::new().length(len).init().unwrap(),
        }
    }
    fn next(&mut self) -> String {
        let hash = self.generator.encode(&[self.id]).unwrap();
        self.id += 1;
        hash
    }
}
