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
    async fn get<'a, T: DeserializeOwned>(url: &'a str) -> T;
    fn post_user<'a, 'b>(url: &'a str, user: &'b User) -> u16;
}

mod original {
    use super::User;
    use serde::de::DeserializeOwned;

    pub async fn get<T: DeserializeOwned>(url: &str) -> T {
        let res = reqwest::Client::new().get(url).send().await.unwrap();

        res.json().await.unwrap()
    }
    pub fn post_user<'a, 'b>(url: &'a str, user: &'b User) -> u16 {
        reqwest::blocking::Client::new()
            .post(url)
            .json(user)
            .send()
            .unwrap()
            .status()
            .as_u16()
    }
}

mod fake {
    use super::User;
    use serde::de::DeserializeOwned;

    pub async fn get<T: DeserializeOwned>(_url: &str) -> T {
        let user = User {
            name: "John".to_string(),
            age: 20,
        };
        serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap()
    }

    pub fn post_user(_url: &str, _user: &User) -> u16 {
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
        let user: User = <FakeModuleMyBehavior as MyBehavior>::get("some_url").await;
        let _ = <OriginalModuleMyBehavior as MyBehavior>::post_user("some_url", &user);
    };

    {};
}
