use liquid::{self, Renderable, Block, LiquidOptions, Template};
use liquid::lexer::{Token, Element};
use liquid::parser::parse;

#[cfg(test)]
use std::default::Default;
#[cfg(test)]
use liquid::lexer::Element::Expression;
#[cfg(test)]
use liquid::{Context, Value};

use cache::RawCache;
use tags::CacheT;

pub struct RawCacheBlock {
    cache_path: String,
}

impl RawCacheBlock {
    pub fn new(path: &str) -> RawCacheBlock {
        RawCacheBlock { cache_path: path.to_string() }
    }
}

impl Block for RawCacheBlock {
    fn initialize<'a>(&'a self,
                      _tag_name: &str,
                      arguments: &[Token],
                      tokens: Vec<Element>,
                      options: &'a LiquidOptions)
                      -> Result<Box<Renderable + 'a>, liquid::Error> {

        let mut args = arguments.iter();
        let inner = try!(parse(&tokens, options));

        let cache_key = match args.next() {
            Some(&Token::Identifier(ref x)) => x.clone(),
            x => {
                return Err(liquid::Error::Parser(format!("Expected an identifier, found {:?}", x)))
            }
        };

        let cachet = CacheT {
            engine: Box::new(RawCache::new(self.cache_path.clone())),
            cache_key: cache_key,
            inner: Template::new(inner),
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
    let mut context = Context::new();
    context.set_val("testkeycache", Value::Str("raw_cache::test_cache".to_string()));
    assert_eq!(cache.unwrap().render(&mut context).unwrap(),
               Some("world".to_string()));
}
