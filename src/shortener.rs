use harsh::{Harsh, HarshBuilder};


/*
 * Shortener keeps track of the current ID and provides a short url generator.
 */
pub struct Shortener<T> {
    id: u64,
    generator: T,
}

/*
 * Shortable is the interface when interacting  with Shorteners.
 */
pub trait Shortable<T> {
    fn next(&mut self) -> String;
}


/*
 * Example implementation of a Shortener using Harsh library.
 */
pub type HarshShortener = Shortener<Harsh>;

impl HarshShortener {
    pub fn new() -> HarshShortener {
        Shortener {
            id: 0,
            generator: HarshBuilder::new().init().unwrap(),
        }
    }
}

impl Shortable<Harsh> for HarshShortener {
    fn next(&mut self) -> String {
        let hash = self.generator.encode(&[self.id]).unwrap();
        self.id += 1;
        hash
    }
}
