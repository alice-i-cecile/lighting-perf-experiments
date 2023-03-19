use std::f64::{consts::PI, EPSILON};

use bevy::{
    math::{DVec2, DVec3},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const WIDTH: usize = 200;
    const HEIGHT: usize = 200;
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let material = materials.add(StandardMaterial {
        base_color: Color::PINK,
        ..default()
    });

    // NOTE: This pattern is good for testing performance of culling as it provides roughly
    // the same number of visible meshes regardless of the viewing angle.
    const N_POINTS: usize = WIDTH * HEIGHT * 4;
    // NOTE: f64 is used to avoid precision issues that produce visual artifacts in the distribution
    let radius = WIDTH as f64 * 2.5;
    let golden_ratio = 0.5f64 * (1.0f64 + 5.0f64.sqrt());
    for i in 0..N_POINTS {
        let spherical_polar_theta_phi = fibonacci_spiral_on_sphere(golden_ratio, i, N_POINTS);
        let unit_sphere_p = spherical_polar_to_cartesian(spherical_polar_theta_phi);
        commands.spawn(PbrBundle {
            mesh: mesh.clone_weak(),
            material: material.clone_weak(),
            transform: Transform::from_translation((radius * unit_sphere_p).as_vec3()),
            ..default()
        });
    }

    // camera
    commands.spawn(Camera3dBundle::default());
}

fn fibonacci_spiral_on_sphere(golden_ratio: f64, i: usize, n: usize) -> DVec2 {
    DVec2::new(
        PI * 2. * (i as f64 / golden_ratio),
        (1.0 - 2.0 * (i as f64 + EPSILON) / (n as f64 - 1.0 + 2.0 * EPSILON)).acos(),
    )
}

fn spherical_polar_to_cartesian(p: DVec2) -> DVec3 {
    let (sin_theta, cos_theta) = p.x.sin_cos();
    let (sin_phi, cos_phi) = p.y.sin_cos();
    DVec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi)
}
