pub extern crate r2d2;
pub extern crate memcache;
pub mod connection_info;
pub mod error;
pub mod connection_manager;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
