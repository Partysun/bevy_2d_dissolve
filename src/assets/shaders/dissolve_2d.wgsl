#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct DissolveParams {
    burn_color: vec4f,
    dissolve_value: f32,
    burn_size: f32,
    #ifdef SIXTEEN_BYTE_ALIGNMENT
    _padding: vec2f,
    #endif
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var dissolve_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var dissolve_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> props: DissolveParams;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let main_texture: vec4f = textureSample(base_color_texture, base_color_sampler, mesh.uv);
    let noise_texture: vec4f = textureSample(dissolve_texture, dissolve_sampler, mesh.uv);
    
    let burn_size_step = props.burn_size * (1.0 - abs(props.dissolve_value * 2.0 - 1.0));
    let threshold: f32 = smoothstep(noise_texture.x - burn_size_step, noise_texture.x, props.dissolve_value);
    let border: f32 = smoothstep(noise_texture.x, noise_texture.x + burn_size_step, props.dissolve_value);
    let mixed_rgb = mix(props.burn_color.rgb, main_texture.rgb, border);

    return vec4<f32>(mixed_rgb, main_texture.a * threshold);
}
