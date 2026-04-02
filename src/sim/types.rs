use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct MarkovNode {
    pub word: String,
}

impl MarkovNode {
    pub fn new(word: String) -> Self {
        MarkovNode { word }
    }
}
