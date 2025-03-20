mod key;
mod value;

pub use key::Key;
pub use value::Value;

pub trait DatabaseClient {
    fn put(&mut self, key: Key, value: Value);
}
