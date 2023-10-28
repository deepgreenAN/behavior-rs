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
    async fn get<'a, T: DeserializeOwned>(url: &'a str) -> T;
    /// sync post user request
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

    /// fake get function
    pub async fn get<T: DeserializeOwned>(_url: &str) -> T {
        let user = User {
            name: "John".to_string(),
            age: 20,
        };
        serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap()
    }

    /// fake post_user function
    pub fn post_user(_url: &str, _user: &User) -> u16 {
        200
    }
}

struct FakeModuleMyBehavior;

#[async_trait]
impl MyBehavior for FakeModuleMyBehavior {
    async fn get<'a, T: DeserializeOwned>(url: &'a str) -> T {
        fake::get::<T>(url).await
    }
    fn post_user<'a, 'b>(url: &'a str, user: &'b User) -> u16 {
        fake::post_user(url, user)
    }
}

struct OriginalModuleMyBehavior;

#[async_trait]
impl MyBehavior for OriginalModuleMyBehavior {
    async fn get<'a, T: DeserializeOwned>(url: &'a str) -> T {
        original::get::<T>(url).await
    }
    fn post_user<'a, 'b>(url: &'a str, user: &'b User) -> u16 {
        original::post_user(url, user)
    }
}

fn main() {}
