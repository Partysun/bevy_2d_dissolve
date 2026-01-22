use bevy::{prelude::*, time::Stopwatch};

pub mod material;
pub use material::*;

#[derive(Default, Clone, Debug)]
pub struct Dissolve2DEffect;

impl Plugin for Dissolve2DEffect {
    fn build(&self, app: &mut App) {
        app.add_plugins(Dissolve2DMaterialPlugin)
            .add_systems(Update, animate_flash_in);
    }
}

// FlashIn Behavior

/// A component that animates the dissolve effect to create a fade-in animation.
///
/// When attached to an entity with a [`Dissolve2DMaterial`], it will automatically
/// animate the `dissolve_value` field in the material's `props` from 0 to 1 over the specified duration.
#[derive(Component, Debug)]
pub struct FlashIn {
    pub duration: f32,
    pub stopwatch: Stopwatch,
}

impl FlashIn {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            stopwatch: Stopwatch::new(),
        }
    }
}

fn animate_flash_in(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FlashIn, &MeshMaterial2d<Dissolve2DMaterial>)>,
    mut materials: ResMut<Assets<Dissolve2DMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut flash_in, material_handle) in &mut query {
        flash_in.stopwatch.tick(time.delta());

        let progress = (flash_in.stopwatch.elapsed_secs() / flash_in.duration).min(1.0);

        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.props.dissolve_value = progress;
        }

        if progress >= 1.0 {
            commands.entity(entity).remove::<FlashIn>();
        }
    }
}
