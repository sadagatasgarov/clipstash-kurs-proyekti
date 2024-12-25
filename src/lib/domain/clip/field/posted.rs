use crate::domain::time::Time;
use serde::{Deserialize, Serialize};
use derive_more::Constructor;


#[derive(Debug, Clone, Deserialize, Serialize, Constructor)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}
