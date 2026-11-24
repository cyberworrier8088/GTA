struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) world_pos: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.uv = model.uv;
    out.color = model.color;
    out.world_pos = model.position;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // GTA 3 Liberty City classic atmospheric depth fog
    let fog_color = vec3<f32>(0.22, 0.28, 0.35); // Cool blue-grey Liberty City haze
    let dist = length(in.world_pos);
    let fog_factor = clamp((dist - 30.0) / 120.0, 0.0, 0.85);

    let lit_color = in.color.rgb;
    let final_rgb = mix(lit_color, fog_color, fog_factor);

    return vec4<f32>(final_rgb, in.color.a);
}
