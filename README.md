[cratesio-image]: https://img.shields.io/crates/v/luno.svg
[cratesio]: https://crates.io/crates/luno
[docsrs-image]: https://docs.rs/luno/badge.svg
[docsrs]: https://docs.rs/luno

# [Luno][docsrs] [![Chrono on crates.io][cratesio-image]][cratesio] [![Chrono on docs.rs][docsrs-image]][docsrs] [![Build Status](https://travis-ci.org/samfatoks/luno-rs.svg?branch=master)](https://travis-ci.org/samatoks/luno-rs)

This Rust library provides a wrapper for the [Luno API](https://www.luno.com/api).

## Authentication

Please visit the [Settings](https://www.luno.com/wallet/settings/api_keys) page
to generate an API key.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
luno = "0.1"
```

### Example usage

A full working example of this library in action.

```rust
#[macro_use]
extern crate log;

use luno_rs::LunoClient;
use std::env;

#[async_std::main]
async fn main() {
    env_logger::builder().format_timestamp_millis().init();

    let api_id = env::var("LUNO_API_ID").unwrap();
    let api_secret = env::var("LUNO_API_SECRET").unwrap();

    let client = LunoClient::new(api_id, api_secret).unwrap();
    let balances = client.get_balances().await.unwrap();
    for balance in balances {
        info!("{} -> Balance: {}, Reserved: {}", balance.asset, balance.balance, balance.reserved);
    }
}
```

Remember to substitute `<id>` and `<secret>` for your own Id and Secret.

We recommend using environment variables rather than including your credentials in plaintext. In Bash you do so as follows:

```
$ export LUNO_API_ID="<id>"
$ export LUNO_API_SECRET="<secret>"
```

## License

[MIT](https://github.com/samfatoks/luno-rs/blob/master/LICENSE.md)