use bevy::prelude::*;
use rand::RngExt;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct MarkovNode {
    pos: Vec2,
    word: String,
}

impl MarkovNode {
    pub fn new(word: String) -> Self {
        let mut rng = rand::rng();
        let pos = Vec2::new(rng.random_range(0. ..1.), rng.random_range(0. ..1.));
        MarkovNode { pos, word }
    }
}
