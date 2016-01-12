use std::sync::{Arc, Mutex};

use liquid::{self, Renderable, Block, LiquidOptions, Template};
use liquid::lexer::{Token, Element};
use liquid::parser::parse;

use redis::Connection;

use cache::RedisCache;
use tags::CacheT;

pub struct RedisCacheBlock {
    conn: Arc<Mutex<Connection>>,
}

impl RedisCacheBlock {
    pub fn new(connection: Arc<Mutex<Connection>>) -> RedisCacheBlock {
        RedisCacheBlock { conn: connection }
    }
}

impl Block for RedisCacheBlock {
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
            engine: Box::new(RedisCache::new(self.conn.clone())),
            cache_key: cache_key,
            inner: Template::new(inner),
        };
        Ok(Box::new(cachet) as Box<Renderable>)
    }
}

