use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct NewsBatchInfo {
    pub max_id: i32,
    pub amount: i64
}
