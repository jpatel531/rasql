#[macro_use]
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}
