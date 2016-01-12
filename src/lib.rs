extern crate liquid;
extern crate redis;

mod cache;
mod tags;

pub use tags::raw_cache::RawCacheBlock;
pub use tags::redis_cache::RedisCacheBlock;
