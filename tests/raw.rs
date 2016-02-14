extern crate liquid;
extern crate liquid_cache as lrc;

use liquid::{Renderable, LiquidOptions, parse, Context, Value};

use lrc::{RawCacheBlock};

#[test]
fn raw_cache() {

    let vec = vec![Value::Str("GOOOO".to_string()); 100000];

    let text = r#"
    {%cache makey %}
{% for i in vec %}
  My stuff {{ i }}
{% endfor %}
{% endcache %}
    "#;

    let mut options : LiquidOptions = Default::default();
    options.blocks.insert("cache".to_string(), RawCacheBlock::new("./tests/tmp"));

    let template = parse(&text, options).unwrap();

    let mut data = Context::new();
    data.set_val("vec", Value::Array(vec));
    data.set_val("makey", Value::Str("cachekeytest".to_string()));

    let _ = template.render(&mut data);

}
