use std::sync::{Arc, Mutex};

use liquid::{self, Renderable, Block, LiquidOptions, Template};
use liquid::lexer::{Token, Element};
use liquid::parser::parse;

use redis::Connection;

use cache::RedisCache;
use tags::CacheT;

pub struct RedisCacheBlock;

impl RedisCacheBlock {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Box<Block> {
        let initialize = move |_tag_name: &str,
                               arguments: &[Token],
                               tokens: Vec<Element>,
                               options: &LiquidOptions|
                               -> Result<Box<Renderable>, liquid::Error> {
            let mut args = arguments.iter();
            let inner = try!(parse(&tokens, options));

            let cache_key = match args.next() {
                Some(&Token::Identifier(ref x)) => x.clone(),
                x => {
                    return Err(liquid::Error::Parser(format!("Expected an identifier, found {:?}",
                                                             x)))
                }
            };

            let cachet = CacheT {
                engine: Box::new(RedisCache::new(connection.clone())),
                cache_key: cache_key,
                inner: Template::new(inner),
            };
            Ok(Box::new(cachet))
        };
        return Box::new(initialize);
    }
}

