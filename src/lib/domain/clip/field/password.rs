use super::super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(p) => {
                if !p.trim().is_empty(){
                    Ok(Self(Some(p)))
                }else {
                    Ok(Self(None))
                }
            }
            // None => Err(ClipError::InvalidPassword("Password xetasi".to_string()))
            None=>Ok(Self(None))
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }

    
}


impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}


impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }

}