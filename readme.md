# Behavior

[![crates.io](https://img.shields.io/crates/v/behavior.svg)](https://crates.io/crates/behavior)
[![docs.rs](https://docs.rs/behavior/badge.svg)](https://docs.rs/behavior)

A macro checks like "behavior" in elixir language.  

## Example

The "behavior" is defined as a trait whose methods are all static methods. And `behavior::behavior` macro checks if all given modules implement behavior trait.  

```rust
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    age: u8,
}

#[behavior::behavior(modules(original, fake))] // check behaviors
#[async_trait::async_trait]
trait MyBehavior {
    async fn get<T: DeserializeOwned>(url: String) -> T;
    fn post_user<'a>(url: String, user: &'a User) -> u16;
}

mod original {
    use crate::User;

    pub async fn get<T: serde::de::DeserializeOwned>(url: String) -> T {
        let res = reqwest::Client::new().get(&url).send().await.unwrap();

        res.json().await.unwrap()
    }
    pub fn post_user(url: String, user: &User) -> u16 {
        reqwest::blocking::Client::new()
            .post(&url)
            .json(user)
            .send()
            .unwrap()
            .status()
            .as_u16()
    }
}

mod fake {
    use crate::User;

    pub async fn get<T: serde::de::DeserializeOwned>(_url: String) -> T {
        serde_json::from_str("{}").unwrap()
    }

    pub fn post_user(_url: String, _user: &User) -> u16 {
        200
    }
}

```

It is useful to guarantee the APIs when using feature flag.

```rust
#[cfg(feature = "fake")]
pub use fake::*;
#[cfg(not(feature = "fake"))]
pub use original::*;
```

Because the macro generates a type which implement behavior trait for each module (in the example, `FakeModuleMyBehavior` and `OriginalModuleMyBehavior`), you can use behavior as normal trait. But it is not recommended.  
