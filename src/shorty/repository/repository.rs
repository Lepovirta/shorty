/*
 * Defines common functions for implementing a Repository.
 * find: return stored String for given ID
 * store: store given String and return an ID for it
 */
pub trait Repository: Sync + Send {
    fn find(&self, id: &str) -> Option<String>;
    fn store(&self, id: &str, url: &str);
}
