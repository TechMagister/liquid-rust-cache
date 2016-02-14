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

pub struct RawCacheBlock;

impl RawCacheBlock {
    pub fn new(path: &str) -> Box<Block> {
        let cache_path = path.to_owned();

        let initialize = move |_tag_name: &str,
                               arguments: &[Token],
                               tokens: Vec<Element>,
                               options: &LiquidOptions|
                               -> Result<Box<Renderable>, liquid::Error> {

            let mut args = arguments.iter();
            let inner = try!(parse(&tokens, &options));

            let cache_key = match args.next() {
                Some(&Token::Identifier(ref x)) => x.clone(),
                x => {
                    return Err(liquid::Error::Parser(format!("Expected an identifier, found {:?}",
                                                             x)))
                }
            };

            let cachet = CacheT {
                engine: Box::new(RawCache::new(cache_path.clone())),
                cache_key: cache_key,
                inner: Template::new(inner),
            };
            Ok(Box::new(cachet))
        };
        return Box::new(initialize);
    }
}

#[test]
fn test_cache() {
    let block = RawCacheBlock::new("./tests/tmp");
    let options: LiquidOptions = Default::default();
    let cache = block("cache",
                      &vec![Token::Identifier("testkeycache".to_string())],
                      vec![Expression(vec![Token::StringLiteral("world".to_string())],
                                      "{{'world'}}".to_string())],
                      &options);
    let mut context = Context::new();
    context.set_val("testkeycache",
                    Value::Str("raw_cache::test_cache".to_string()));
    assert_eq!(cache.unwrap().render(&mut context).unwrap(),
               Some("world".to_string()));
}
