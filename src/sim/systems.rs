use bevy::prelude::*;

use crate::{markov::MarkovChain, sim::types::MarkovNode};

pub fn spawn_words(
    mut command: Commands, 
    mc: Res<MarkovChain>
    meshes: ResMut<Assets<Mesh>>
) {
    for word in &mc.words {
        let mesh = Mesh2d()

        command.spawn(MarkovNode::new(word.clone()));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. Spawn the Camera
    // Without this, you will just see a black screen!
    commands.spawn(Camera2dBundle::default());

    // 2. Spawn a Circle
    commands.spawn();
}
