use super::super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use rocket::form::{self, FromFormField, ValueField};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();
        match title {
            Some(p) => {
                if !p.trim().is_empty() {
                    Self(Some(p))
                } else {
                    Self(None)
                }
            }
            // None => Err(ClipError::InvalidPassword("Password xetasi".to_string()))
            None => Self(None),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}


#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title {
    fn from_value(field:ValueField<'r>) -> form::Result<'r,Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}