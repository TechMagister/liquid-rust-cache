extern crate liquid;
extern crate redis;
extern crate liquid_cache as lrc;

use std::sync::{Arc, Mutex};

use liquid::{Renderable, Block, LiquidOptions, parse, Context, Value};

use lrc::{RedisCacheBlock};

#[test]
fn redis_cache() {

    // redis
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let con = Arc::new(Mutex::new(client.get_connection().unwrap()));
    // End

    let vec = vec![Value::Str("GOOOO".to_string()); 100000];

    let text = r#"
    {%cache makey %}
{% for i in vec %}
  My stuff {{ i }}
{% endfor %}
{% endcache %}
    "#;

    let mut options : LiquidOptions = Default::default();
    options.blocks.insert("cache".to_string(), Box::new(RedisCacheBlock::new(con.clone())) as Box<Block>); 

    let template = parse(&text, &mut options).unwrap();

    let mut data = Context::new();
    data.set_val("vec", Value::Array(vec));
    data.set_val("makey", Value::Str("cachekeytest".to_string()));

    let _ = template.render(&mut data);

}
