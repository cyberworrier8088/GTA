struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct ModelUniform {
    model: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> model_transform: ModelUniform;

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
    @location(3) normal: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.uv = model.uv;
    out.color = model.color;
    
    let world_pos4 = model_transform.model * vec4<f32>(model.position, 1.0);
    out.world_pos = world_pos4.xyz;
    out.normal = (model_transform.model * vec4<f32>(model.normal, 0.0)).xyz;
    out.clip_position = camera.view_proj * world_pos4;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Directional sunlight (Liberty City overcast sun)
    let light_dir = normalize(vec3<f32>(0.5, 0.8, 0.3));
    let n = normalize(in.normal);
    let diff = max(dot(n, light_dir), 0.0);
    let ambient = 0.45;
    let light_intensity = ambient + diff * 0.55;

    // Classic Liberty City distance atmospheric fog
    let fog_color = vec3<f32>(0.24, 0.30, 0.38);
    let dist = length(in.world_pos);
    let fog_factor = clamp((dist - 40.0) / 180.0, 0.0, 0.90);

    let lit_rgb = in.color.rgb * light_intensity;
    let final_rgb = mix(lit_rgb, fog_color, fog_factor);

    return vec4<f32>(final_rgb, in.color.a);
}
