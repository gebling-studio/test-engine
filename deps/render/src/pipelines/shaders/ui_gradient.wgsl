
struct RectView {
    resolution: vec2<f32>,
    _padding: vec2<u32>,
}

struct UIGradientInstance {
    @location(2) position:      vec2<f32>,
    @location(3) size:          vec2<f32>,
    @location(4) start_color:   vec4<f32>,
    @location(5) end_color:     vec4<f32>,
    @location(6) corner_radii:  vec4<f32>,
    @location(7) z_position:    f32,
    @location(8) scale:         f32,
}

@group(0) @binding(0)
var<uniform> view: RectView;

struct VertexOutput {
    @builtin(position) pos:   vec4<f32>,
          @location(0) uv:   vec2<f32>,
          @location(1) size: vec2<f32>,
          @location(2) corner_radii: vec4<f32>,
          @location(3) gradient_pos:  f32,
          @location(4) start_color: vec4<f32>,
          @location(5) end_color:   vec4<f32>,
}

@vertex
fn v_main(
    @location(0) model: vec2<f32>,
    instance: UIGradientInstance,
) -> VertexOutput {
    var out_pos: vec4<f32> = vec4<f32>(model, instance.z_position, 1.0);

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
    out.pos   = out_pos;

    out.uv = model * 0.5;
    out.gradient_pos = (model.y + 1.0) / 2.0;
    out.size = instance.size;
    out.corner_radii = instance.corner_radii;
    out.start_color = instance.start_color;
    out.end_color = instance.end_color;

    return out;
}

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

// Signed distance to a rounded box centered at the origin. Negative inside.
fn rounded_box_sdf(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius, radius);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

// One pixel wide analytic edge coverage. See ui_rect.wgsl.
fn edge_coverage(dist: f32) -> f32 {
    return clamp(0.5 - dist / fwidth(dist), 0.0, 1.0);
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = mix(in.start_color, in.end_color, in.gradient_pos);

    let local_pos: vec2<f32> = in.uv * in.size;
    let radius: f32 = pick_radius(local_pos, in.corner_radii);
    let dist: f32 = rounded_box_sdf(local_pos, in.size * 0.5, radius);

    let alpha: f32 = color.a * edge_coverage(dist);

    if alpha < 0.004 {
        discard;
    }

    return vec4<f32>(color.rgb, alpha);
}
