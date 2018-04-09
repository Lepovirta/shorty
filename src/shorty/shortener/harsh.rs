use harsh::{Harsh, HarshBuilder};

use shortener::Shortener;

/*
 * Example implementation of a Shortener using Harsh library.
 */
pub struct HarshShortener {
    id: u64,
    generator: Harsh,
}

impl HarshShortener {
    pub fn new(len: usize) -> HarshShortener {
        HarshShortener {
            id: 0,
            generator: HarshBuilder::new().length(len).init().unwrap(),
        }
    }
}

impl Shortener for HarshShortener {
    fn next(&mut self) -> String {
        let hash = self.generator.encode(&[self.id]).unwrap();
        self.id += 1;
        hash
    }
}
