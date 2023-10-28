use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    age: u8,
}

#[async_trait]
trait MyBehavior {
    /// async get request
    async fn get<T: DeserializeOwned>(url: String) -> T;
    /// sync post user request
    fn post_user<'a>(url: String, user: &'a User) -> u16;
}

mod original {
    use crate::User;
    use serde::de::DeserializeOwned;

    pub async fn get<T: DeserializeOwned>(url: String) -> T {
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
    use serde::de::DeserializeOwned;

    /// fake get function
    pub async fn get<T: DeserializeOwned>(_url: String) -> T {
        let user = User {
            name: "John".to_string(),
            age: 20,
        };
        serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap()
    }

    /// fake post_user function
    pub fn post_user(_url: String, _user: &User) -> u16 {
        200
    }
}

struct FakeModuleMyBehavior;

#[async_trait]
impl MyBehavior for FakeModuleMyBehavior {
    async fn get<T: DeserializeOwned>(url: String) -> T {
        fake::get(url).await
    }
    fn post_user<'a>(url: String, user: &'a User) -> u16 {
        fake::post_user(url, user)
    }
}

struct OriginalModuleMyBehavior;

#[async_trait]
impl MyBehavior for OriginalModuleMyBehavior {
    async fn get<T: DeserializeOwned>(url: String) -> T {
        original::get(url).await
    }
    fn post_user<'a>(url: String, user: &'a User) -> u16 {
        original::post_user(url, user)
    }
}

fn main() {}
