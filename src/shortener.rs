use harsh::{Harsh, HarshBuilder};


pub trait Shortener {
    fn new() -> Self;
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
    fn new() -> HarshShortener {
        HarshShortener {
            id: 0,
            generator: HarshBuilder::new().init().unwrap(),
        }
    }
    fn next(&mut self) -> String {
        let hash = self.generator.encode(&[self.id]).unwrap();
        self.id += 1;
        hash
    }
}
