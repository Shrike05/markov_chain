use bevy::prelude::*;
use rand::RngExt;

use crate::{markov::MarkovChain, sim::types::MarkovNode};

pub fn spawn_words(
    mut command: Commands,
    mc: Res<MarkovChain>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2d(meshes.add(Circle::new(5.)));
    let mat = MeshMaterial2d(materials.add(Color::from(LinearRgba::rgb(1., 1., 1.))));

    let mut rng = rand::rng();

    for word in &mc.words {
        command.spawn((
            MarkovNode::new(word.clone()),
            mesh.clone(),
            mat.clone(),
            Transform::from_xyz(
                rng.random_range(0_f32..100_f32),
                rng.random_range(0_f32..100_f32),
                0.,
            ),
        ));
    }
}

pub fn update_position(
    mut mc_nodes_query: Query<(&mut Transform, &MarkovNode)>,
    mc_chain: Res<MarkovChain>,
) {
    let mut new_nodes: Vec<Vec3> = mc_nodes_query.iter().map(|(t, _)| t.translation).collect();
    let unit = 500.;

    for (i, (mc_t, mc)) in mc_nodes_query.iter().enumerate() {
        for (other_mc_t, other_mc) in mc_nodes_query.iter() {
            if other_mc_t.eq(mc_t) {
                continue;
            }

            let diff = other_mc_t.translation - mc_t.translation;
            let dir = diff.normalize_or_zero();
            let dist = diff.length().max(1.);

            let tran1 = mc_chain
                .transition_from_x_to_y(&mc.word, &other_mc.word)
                .unwrap_or(0.);
            let tran2 = mc_chain
                .transition_from_x_to_y(&other_mc.word, &mc.word)
                .unwrap_or(0.);

            let tran = (tran1 + tran2) / 2.;
            let force = (dist - tran * unit) / unit;

            new_nodes[i] += force * dir;
        }
    }

    for (i, (mut mc, _)) in mc_nodes_query.iter_mut().enumerate() {
        mc.translation = new_nodes[i];
    }
}

pub fn draw_lines(
    mc_nodes_query: Query<(&Transform, &MarkovNode)>,
    mc_chain: Res<MarkovChain>,
    mut gizmos: Gizmos,
) {
    let white = LinearRgba::rgb(1., 1., 1.);

    for (mc_t, mc) in mc_nodes_query.iter() {
        for (other_mc_t, other_mc) in mc_nodes_query.iter() {
            if other_mc_t.eq(mc_t) {
                continue;
            }

            let tran = mc_chain
                .transition_from_x_to_y(&mc.word, &other_mc.word)
                .unwrap_or(0.);

            if tran != 0. {
                gizmos.line_2d(mc_t.translation.xy(), other_mc_t.translation.xy(), white);
            }
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0., 0., 1.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
