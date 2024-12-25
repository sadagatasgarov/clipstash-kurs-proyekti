use crate::domain::time::Time;
use super::super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use derive_more::Constructor;


#[derive(Debug, Clone, Deserialize, Serialize, Constructor)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}
