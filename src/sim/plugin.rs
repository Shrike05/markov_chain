use bevy::prelude::*;

use crate::sim::systems::*;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_words, setup));
        app.add_systems(Update, (update_position, draw_lines));
    }
}
