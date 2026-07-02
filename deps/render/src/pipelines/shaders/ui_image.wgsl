
struct RectView {
    resolution: vec2<f32>,
    _padding: vec2<u32>,
}

struct Vertex {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct UIImageInstance {
    @location(2) position:      vec2<f32>,
    @location(3) size:          vec2<f32>,
    @location(4) border_color:  vec4<f32>,
    @location(5) border_width:  f32,
    @location(6) corner_radii:  vec4<f32>,
    @location(7) z_position:    f32,
    @location(8) flags:         u32,
    @location(9) scale:         f32,
}

@group(0) @binding(0)
var<uniform> view: RectView;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) corner_uv: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) corner_radii: vec4<f32>,
    @location(5) border_width: f32,
}

@vertex
fn v_main(
    model: Vertex,
    instance: UIImageInstance,
) -> VertexOutput {
    let flip_x: bool = ((instance.flags >> 0u) & 1u) != 0u;
    let flip_y: bool = ((instance.flags >> 1u) & 1u) != 0u;

    var pos = model.pos;

    if flip_x {
        pos.x *= -1.0;
    }

    if flip_y {
        pos.y *= -1.0;
    }

    var out_pos: vec4<f32> = vec4<f32>(pos, instance.z_position, 1.0);

    out_pos.y = -out_pos.y;

    out_pos.x /= 2.0;
    out_pos.y /= 2.0;

    out_pos.x += 0.5;
    out_pos.y += 0.5;

    out_pos.x /= view.resolution.x;
    out_pos.y /= view.resolution.y;

    out_pos.x *= instance.size.x * instance.scale;
    out_pos.y *= instance.size.y * instance.scale;

    out_pos.x += instance.position.x * instance.scale / view.resolution.x;
    out_pos.y += instance.position.y * instance.scale / view.resolution.y;

    out_pos.y *= -1.0;

    out_pos.x -= 0.5;
    out_pos.y += 0.5;

    out_pos.x *= 2.0;
    out_pos.y *= 2.0;

    var out: VertexOutput;
    out.pos = out_pos;
    out.uv  = model.uv;
    // Screen orientation, not texture orientation: x follows the
    // flipped pos, y is negated to undo the extra negation above,
    // so negative y is the top like in the other UI shaders.
    out.corner_uv = vec2<f32>(pos.x, -pos.y) * 0.5;
    out.size = instance.size;
    out.corner_radii = instance.corner_radii;
    out.border_color = instance.border_color;
    out.border_width = instance.border_width;
    return out;
}

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

// Radii order: top left, top right, bottom left, bottom right.
// Local coordinates have negative y at the top.
fn pick_radius(p: vec2<f32>, radii: vec4<f32>) -> f32 {
    if p.y < 0.0 {
        if p.x < 0.0 {
            return radii.x;
        }
        return radii.y;
    }
    if p.x < 0.0 {
        return radii.z;
    }
    return radii.w;
}

// Signed distance to a rounded box centered at the origin. Negative
// inside. With radius 0 the inside distance is minus the distance to
// the nearest edge, so the border band below works for square corners.
fn rounded_box_sdf(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius, radius);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureSample(t_diffuse, s_diffuse, in.uv);
    let local_pos: vec2<f32> = in.corner_uv * in.size;
    let radius: f32 = pick_radius(local_pos, in.corner_radii);
    let dist: f32 = rounded_box_sdf(local_pos, in.size * 0.5, radius);

    if dist > 0.0 {
        discard;
    }

    if in.border_width > 0.0 && dist > -in.border_width {
        return in.border_color;
    }

    if tex.a == 0.0 {
        discard;
    }

    return tex;
}
