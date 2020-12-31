[cratesio-image]: https://img.shields.io/crates/v/luno.svg
[cratesio]: https://crates.io/crates/luno
[docsrs-image]: https://docs.rs/luno/badge.svg
[docsrs]: https://docs.rs/luno

# [Luno][docsrs] [![Chrono on crates.io][cratesio-image]][cratesio] [![Chrono on docs.rs][docsrs-image]][docsrs] [![Build Status](https://travis-ci.com/samfatoks/luno-rs.svg?branch=main)](https://travis-ci.com/samfatoks/luno-rs)

Rust wrapper for [Luno API](https://www.luno.com/api).

## Authentication

Please visit the [Settings](https://www.luno.com/wallet/settings/api_keys) page
to generate an API key.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
luno-rs = "0.1"
```

### Example usage

A full working example of this library in action.

```rust
use luno_rs::LunoClient;
use std::env;

#[async_std::main]
async fn main() {
    env_logger::builder().format_timestamp_millis().init();

    let key_id = env::var("LUNO_KEY_ID").unwrap();
    let key_secret = env::var("LUNO_KEY_SECRET").unwrap();

    let client = LunoClient::new(key_id, key_secret).unwrap();
    let balances = client.get_balances().await.unwrap();
    for balance in balances {
        println!("{} -> Balance: {}, Reserved: {}", balance.asset, balance.balance, balance.reserved);
    }
}
```

We recommend using environment variables rather than including your credentials in plaintext. Run the following in Bash to export Key ID and Secret:

```bash
export LUNO_KEY_ID="<id>"
export LUNO_KEY_SECRET="<secret>"
```

Remember to substitute `<id>` and `<secret>` with your own Key Id and Secret.

## License

[MIT](https://github.com/samfatoks/luno-rs/blob/master/LICENSE.md)
