use liquid::{self, Renderable, Block, LiquidOptions, Context, Template};
use liquid::lexer::{Token, Element};
use liquid::parser::parse;

#[cfg(test)]
use std::default::Default;
#[cfg(test)]
use liquid::lexer::Element::Expression;

use cache::{self, Cache};

pub struct RawCacheBlock {
    cache_path: String
}

impl RawCacheBlock {
    pub fn new(path: &str) -> RawCacheBlock{
        RawCacheBlock {cache_path : path.to_string() }
    }
}

struct CacheT<'a> {
    cacher : cache::RawCache,
    cache_key: String,
    inner: Template<'a>
}


impl<'a> Renderable for CacheT<'a>{
    fn render(&self, context: &mut Context) -> Result<Option<String>, liquid::Error> {
        let cached =  self.cacher.get(&self.cache_key);

        if cached.is_some() {
            return Ok(cached);
        } else {
            match self.inner.render(context) {
                Ok(Some(s)) => {
                    self.cacher.set(&self.cache_key, s.clone());
                    Ok(Some(s))
                },
                Ok(None) => Ok(None),
                Err(e) => Err(e)
            }
        }

    }
}

impl Block for RawCacheBlock{
    fn initialize<'a>(&'a self,
                      _tag_name: &str,
                      arguments: &[Token],
                      tokens: Vec<Element>,
                      options: &'a LiquidOptions)
        -> Result<Box<Renderable +'a >, liquid::Error> {

            let mut args = arguments.iter();
            let inner = try!(parse(&tokens, options));

            let cache_key = match args.next() {
                Some(&Token::Identifier(ref x)) => x.clone(),
                x => return Err(liquid::Error::Parser(format!("Expected an identifier, found {:?}", x))),
            };

            let cachet = CacheT {
                cacher: cache::RawCache::new(self.cache_path.clone()),
                cache_key: cache_key,
                inner : Template::new(inner)
            };
            Ok(Box::new(cachet) as Box<Renderable>)
        }
}

#[test]
fn test_cache() {
    let block = RawCacheBlock::new("./tests/tmp");
    let options: LiquidOptions = Default::default();
    let cache = block.initialize("cache",
                                 &vec![Token::Identifier("testkeycache".to_string())],
                                 vec![Expression(vec![Token::StringLiteral("world".to_string())],
                                 "{{'world'}}".to_string())],
                                 &options);
    assert_eq!(cache.unwrap().render(&mut Default::default()).unwrap(), Some("world".to_string()));
}

