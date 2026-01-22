use bevy::{
    prelude::*,
    render::render_resource::*,
    shader::*,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// A 2D material for rendering dissolve and burn effects on meshes.
///
/// The material uses a `DissolveParams` uniform struct to control the dissolve effect properties
/// (burn color, dissolve amount, and burn edge size), along with a color texture and optional
/// noise texture for the dissolve pattern.
///
/// # Example
///
/// ```ignore
/// # use bevy::prelude::*;
/// # use bevy_2d_dissolve::Dissolve2DMaterial;
/// # use bevy_asset::Assets;
///
/// fn setup(
///     mut commands: Commands,
///     mut meshes: ResMut<Assets<Mesh>>,
///     mut materials: ResMut<Assets<Dissolve2DMaterial>>,
///     asset_server: Res<AssetServer>,
/// ) {
///     commands.spawn((
///         Mesh2d(meshes.add(Rectangle::default())),
///         MeshMaterial2d(materials.add(Dissolve2DMaterial {
///             color_texture: Some(asset_server.load("texture.png")),
///             noise_texture: Some(asset_server.load("textures/noise.png")),
///             props: DissolveParams {
///                 burn_color: Vec4::new(1.0, 0.4, 0.0, 1.0),
///                 burn_size: 0.03,
///                 dissolve_value: 0.5,
///                 _padding: Vec2::ZERO,
///             },
///         })),
///     ));
/// }
/// ```
/// Shader uniforms for controlling the dissolve and burn edge effect.
///
/// # Fields
///
/// - `burn_color`: The RGBA color to apply at the dissolving edges (default: orange `[1.0, 0.4, 0.0, 1.0]`)
/// - `dissolve_value`: Controls the amount of dissolution, from 0.0 (fully visible) to 1.0 (fully dissolved) (default: 0.0)
/// - `burn_size`: The width of the burn edge effect as a fraction of texture space (default: 0.04)
/// - `_padding`: Alignment padding for 16-byte struct alignment required by Webgl2 build
#[derive(ShaderType, Debug, Clone)]
pub struct DissolveParams {
    pub burn_color: Vec4,
    pub dissolve_value: f32,
    pub burn_size: f32,
    pub _padding: Vec2,
}

impl Default for DissolveParams {
    fn default() -> Self {
        Self {
            burn_color: Vec4::new(1.0, 0.4, 0.0, 1.0),
            dissolve_value: 0.0,
            burn_size: 0.04,
            // explicit padding for 16-byte aligned struct
            _padding: Vec2::default(),
        }
    }
}

/// A 2D material for dissolve and burn effects on meshes.
///
/// # Fields
///
/// - `color_texture`: The base texture to render with the dissolve effect applied
/// - `noise_texture`: A noise texture for dissolve pattern generation (automatically loaded from `textures/noise.png` if not provided)
/// - `props`: The `DissolveParams` uniform controlling burn color, dissolve amount, and burn edge size
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Dissolve2DMaterial {
    /// Base color texture with bind group index 0 and sampler index 1
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Option<Handle<Image>>,
    /// Noise texture for dissolve pattern with bind group index 2 and sampler index 3
    #[texture(2)]
    #[sampler(3)]
    pub noise_texture: Option<Handle<Image>>,
    /// Dissolve effect parameters with bind group index 4
    #[uniform(4)]
    pub props: DissolveParams,
}

impl Material2d for Dissolve2DMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("shaders/dissolve_2d.wgsl".into())
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

impl Default for Dissolve2DMaterial {
    fn default() -> Self {
        Self {
            color_texture: Some(Handle::default()),
            noise_texture: None,
            props: DissolveParams { ..default() },
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Dissolve2DMaterialPlugin;

impl Plugin for Dissolve2DMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<Dissolve2DMaterial>::default())
            .add_systems(PostStartup, apply_default_dissolve_texture);
    }
}

/// Automatically loads the default dissolve noise texture for materials that don't have one specified.
///
/// This system runs in `PostStartup` and applies the default noise texture to all
/// [`Dissolve2DMaterial`] instances that have `noise_texture` set to `None`.
/// This ensures the dissolve effect works without requiring users to manually specify a noise texture.
///
/// The default texture is loaded from `textures/noise.png`.
fn apply_default_dissolve_texture(
    mut materials: ResMut<Assets<Dissolve2DMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("textures/noise.png");
    for (_, material) in materials.iter_mut() {
        if material.noise_texture.is_none() {
            material.noise_texture = Some(texture.clone());
        }
    }
}
