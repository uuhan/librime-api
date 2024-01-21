## About

This is a Rust crate which export & extend the [librime](https://github.com/rime/librime) project

Aims to have user friendly api and to help writing librime modules in Rust.

## Usage

```toml
[dependencies.librime-api]
git = "https://github.com/uuhan/librime-api"
branch = "master"
```

```rust,no_run
use librime_api::*;
let mut rime = RimeBuilder::new()
    .shared_data_dir("./rime-ice")
    .user_data_dir("./rime-user")
    .distribution_name("RIME")
    .distribution_code_name("RIME")
    .distribution_version("RIME")
    .app_name("rime-query")
    .log_dir("./rime-log")
    .build()
    .unwrap();
```

