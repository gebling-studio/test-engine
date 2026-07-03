struct PathView {
    position: vec2<f32>,
    resolution: vec2<f32>,
    color: vec4<f32>,
    z_position: f32,
}

@group(0) @binding(0) var<uniform> path_view: PathView;

@vertex
fn v_main(
    @location(0) vertex: vec2<f32>,
) -> @builtin(position) vec4<f32>  {
    var x: f32 = (vertex.x + path_view.position.x) * 2.0;
    var y: f32 = (vertex.y + path_view.position.y) * 2.0;

    x /= path_view.resolution.x;
    y /= path_view.resolution.y;

    return vec4<f32>(-1.0 + x, 1.0 - y, path_view.z_position, 1.0);
}

@fragment
fn f_main() -> @location(0) vec4<f32> {
    return path_view.color;
}
