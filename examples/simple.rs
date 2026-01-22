use bevy::prelude::*;
use bevy_2d_dissolve::{Dissolve2DEffect, Dissolve2DMaterial, DissolveParams, FlashIn};

#[derive(Component)]
struct DissolveValueDisplay;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Dissolve2DEffect))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_dissolve_input, update_dissolve_display).chain(),
        );

    app.run();
}

// Setup a simple 2d scene with dissolve effect
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Dissolve2DMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Quad with dissolve material
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Dissolve2DMaterial {
            color_texture: Some(asset_server.load("bevy_logo.png")),
            props: DissolveParams {
                burn_size: 0.03,
                // Image is not visible
                dissolve_value: 0.,
                ..default()
            },
            ..default()
        })),
        Transform::default().with_scale(Vec3::splat(256.)),
        FlashIn::new(5.0),
    ));

    commands.spawn((
        Text::new("= increase | - decrease dissolve"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("Dissolve value: 0.00"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(12),
            ..default()
        },
        DissolveValueDisplay,
    ));
}

fn handle_dissolve_input(
    mut materials: ResMut<Assets<Dissolve2DMaterial>>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut flash_in_query: Query<Entity, With<FlashIn>>,
) {
    const STEP: f32 = 0.02;
    let mut delta = 0.0;

    if input.pressed(KeyCode::Equal) {
        delta += STEP;
    }
    if input.pressed(KeyCode::Minus) {
        delta -= STEP;
    }

    if delta == 0.0 {
        return;
    }

    // Remove FlashIn component when user starts interacting
    for entity in flash_in_query.iter_mut() {
        commands.entity(entity).remove::<FlashIn>();
    }

    for material in materials.iter_mut() {
        material.1.props.dissolve_value = (material.1.props.dissolve_value + delta).clamp(0.0, 1.0);
    }
}

fn update_dissolve_display(
    mut text_query: Query<&mut Text, With<DissolveValueDisplay>>,
    materials: Res<Assets<Dissolve2DMaterial>>,
) {
    if let Some(material) = materials.iter().next() {
        let dissolve_value = material.1.props.dissolve_value;
        for mut text in text_query.iter_mut() {
            text.0 = format!("Dissolve value: {:.2}", dissolve_value);
        }
    }
}
