# 短网址核心功能

用于将原始网址转换成6个字符长度的短网址。

## 引入

```toml
[dependencies]
shorturl-core = { git = "https://github.com/r7n/shorturl-core.git" }
```

## 直接使用

```rust
use shorturl_core::short_url;

let url:String = "r7n.cc".to_string();
let shorted_url:String = short_url(url); # 3TrFk7
println!("{}", shorted_url);
```

## 带 seed

```rust
use shorturl_core::short_url_with_seed;

let url:String = "r7n.cc".to_string();
let shorted_url:String = short_url_with_seed(url, 1u32); # 3KC4r5
println!("{}", shorted_url);
```

