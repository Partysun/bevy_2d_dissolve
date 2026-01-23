# Bevy 2D Dissolve Effect

A Bevy plugin providing dissolve and burn edge effects
for 2D meshes with customizable burn color and size.

![showcase](demo-example-simple.webp)

## Installation

Add the plugin to your `Cargo.toml`:

```toml
# From Git with embedded noise texture (recommended)
bevy_2d_dissolve = { git = "https://github.com/Partysun/bevy_2d_dissolve", features = ["embedded_noise"] }
```

### Feature Flags

- **`embedded_shader`** (enabled by default): Embeds the WGSL shader in the binary. Disable with `default-features = false` if you provide external shader files.

> **⚠️ Important Notice about `embedded_noise`**
>
> - **Enabled**: Includes the default noise texture in the binary. Use this if you want the bundled noise texture without needing external asset files.
> - **Disabled**: Allows you to provide your own noise texture. You must either load a custom texture or ensure `textures/noise.png` is available in your assets directory.

## Summary

Add the plugin to your app and use `Dissolve2DMaterial`
to render meshes with dissolve effects:

[Web demo](https://partysun.itch.io/bevy-2d-shader-dissolve-effect)

```rust
use bevy::prelude::*;
use bevy_2d_dissolve::{Dissolve2DEffect, Dissolve2DMaterial, FlashIn};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Dissolve2DEffect))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Dissolve2DMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Spawn a mesh with dissolve material and flash-in animation
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Dissolve2DMaterial {
            color_texture: Some(asset_server.load("my_texture.png")),
            noise_texture: Some(asset_server.load("textures/noise.png")),
            props: DissolveParams {
                burn_color: Vec4::new(1.0, 0.5, 0.0, 1.0),
                burn_size: 0.05,
                dissolve_value: 0.0,
                ..default()
            },
        })),
        Transform::default().with_scale(Vec3::splat(256.)),
        FlashIn::new(2.0), // 2 second fade-in
    ));
}
```

## Quick Start

Run the simple example:

```bash
cargo run --example simple
```

In the example, use:

- **=** key to increase dissolve effect
- **-** key to decrease dissolve effect

## Refs

- <https://godotshaders.com/shader/2d-dissolve-with-burn-edge/>
- <https://godotshaders.com/shader/simple-2d-dissolve/>

## Version Compatibility

| bevy | bevy_2d_dissolve |
| ---- | ---------------- |
| 0.18 | 0.1              |

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
