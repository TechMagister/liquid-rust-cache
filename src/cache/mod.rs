
pub use cache::raw::RawCache;
pub use cache::redis::RedisCache;

mod raw;
mod redis;


pub trait Cache {
    fn get(&self, _k: &String) -> Option<String>;
    fn set(&self, _k: &String, _v: &String);
}
