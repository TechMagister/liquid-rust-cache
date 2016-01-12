pub mod raw_cache;
pub mod redis_cache;

use liquid::{self, Renderable, Template, Context, Value};
use cache;

pub struct CacheT<'a> {
    engine: Box<cache::Cache>,
    cache_key: String,
    inner: Template<'a>
}

impl<'a> Renderable for CacheT<'a> {
    fn render(&self, context: &mut Context) -> Result<Option<String>, liquid::Error> {

        let cache_key = match context.get_val(&self.cache_key) {
            Some(&Value::Str(ref s)) => s.clone(),
            x => return Err(liquid::Error::from(
                    format!("Wrong cache key, expected Str, found : {:?}", x))),
        };

        let cached = self.engine.get(&cache_key);

        if cached.is_some() {
            return Ok(cached);
        } else {
            match self.inner.render(context) {
                Ok(Some(ref s)) => {
                    self.engine.set(&cache_key, s);
                    Ok(Some(s.clone()))
                }
                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        }

    }
}
