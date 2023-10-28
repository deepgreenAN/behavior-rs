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

// #[cfg(feature = "fake")]
// pub use fake::*;
// #[cfg(not(feature = "fake"))]
// pub use original::*;

#[allow(unused_must_use)]
fn main() {
    // The following is not recommended.
    async {
        let user: User = <FakeModuleMyBehavior as MyBehavior>::get("some_url".to_string()).await;
        let _ = <OriginalModuleMyBehavior as MyBehavior>::post_user("some_url".to_string(), &user);
    };

    {};
}
