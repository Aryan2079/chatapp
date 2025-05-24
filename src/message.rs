use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Messageformat{
    pub to: String,
    pub from: String,
    pub message: String
}
#[allow(dead_code)]
impl Messageformat{
    pub fn new()->Self{
        Messageformat{
            to: String::new(),
            from: String::new(),
            message: String::new()
        }
    }
}