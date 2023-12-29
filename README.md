## librime rust bindings

https://github.com/rime/librime

commit: de12d6ae52ea0439ac689a9469b7041653a19fd5

## features

1. logging: building librime with *-DENABLE_LOGGING=on*

2. link-cxx: link librime with *-lc++*
   
3. link-stdcxx: link librime with *-lstdc++*

## usage

```toml
[dependencies.librime-api]
version = "0.1.0"
features = ["link-cxx"]
```

```rust
use librime_api::api as librime;
```
