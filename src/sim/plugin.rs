use bevy::prelude::*;

use crate::sim::systems::spawn_words;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_words);
    }
}
