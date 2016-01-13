# liquid-cache
Cache block for the liquid templating engine ( https://github.com/FerarDuanSednan/liquid-rust )

**Still basic and unstable, this work is currently under development**

TODO
-------
- [ ] Improve caching keys ( md5 of the content, etc )
- [ ] Allow a 2nd arg to the filter to set the expiration time

Exemples :
---------

**Cache into a folder**
```rust
let vec = vec![Value::Str("GOOOO".to_string()); 100000];

let text = r#"
{%cache makey %}
  {% for i in vec %}
    My stuff {{ i }}
  {% endfor %}
{% endcache %}
"#;

let mut options : LiquidOptions = Default::default();
options.blocks.insert("cache".to_string(), Box::new(RawCacheBlock::new("./tests/tmp")) as Box<Block>); 

let template = parse(&text, &mut options).unwrap();

let mut data = Context::new();
data.set_val("vec", Value::Array(vec));
data.set_val("makey", Value::Str("cachekeytest".to_string())); // tmp file : ./tests/tmp/cachekeytest

let _ = template.render(&mut data);
```

**Use redis to cache data**
```rust
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
```

