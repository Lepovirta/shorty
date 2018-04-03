/*
 * Defines common functions for implementing a Repository.
 * find: return stored String for given ID
 * store: store given String and return an ID for it
 */
pub trait Repository {
    fn find(&self, id: String) -> Option<String>;
    fn store(&mut self, url: &String) -> String;
}

/*
 * Wrap Repository inside a Box making it an trait object. This way it is
 * possible to pass it in rocket library's request handlers. Request handlers
 * require concrete types and this can be achieved with a trait object
 * providing dynamic dispatch.
 * TODO: is this the 'best' way?
 */
pub struct BRepository {
    pub data : Box<Repository + Sync + Send>,
}
