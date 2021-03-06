use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub enum Level {
    One,
    Two,
    Three,
}
