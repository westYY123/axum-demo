use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct UserDto {
    name: String,
    password: String,
    gender: Option<Gender>,
    register_time: Instant,
    birth: Instant,
}

#[derive(Debug, Deserialize)]
pub struct User {
    name: String,
    password: String,
    gender: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

impl From<User> for UserDto {
    fn from(value: User) -> Self {
        Self {
            name: value.name,
            password: value.password,
            gender: match value.gender {
                Some(s) => {
                    if s.to_lowercase().contains("male") {
                        Some(Gender::Male)
                    } else if s.to_lowercase().contains("female") {
                        Some(Gender::Female)
                    } else {
                        None
                    }
                }
                None => None,
            },
            register_time: Instant::now(),
            birth: Instant::now(),
        }
    }
}
