struct RectView {
    resolution: vec2<f32>,
    _padding: vec2<u32>,
}

struct UIRectInstance {
    @location(2) position:      vec2<f32>,
    @location(3) size:          vec2<f32>,
    @location(4) color:         vec4<f32>,
    @location(5) border_color:  vec4<f32>,
    @location(6) border_width:  f32,
    @location(7) corner_radii:  vec4<f32>,
    @location(8) z_position:    f32,
    @location(9) scale:         f32,
}

@group(0) @binding(0)
var<uniform> view: RectView;

@group(1) @binding(0) var blurred: texture_2d<f32>;
@group(1) @binding(1) var blurred_sampler: sampler;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) border_color: vec4<f32>,
    @location(4) corner_radii: vec4<f32>,
    @location(5) border_width: f32,
}

@vertex
fn v_main(
    @location(0) model: vec2<f32>,
    instance: UIRectInstance,
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
    out.color = instance.color;

    out.uv = model * 0.5;
    out.size = instance.size;
    out.corner_radii = instance.corner_radii;
    out.border_color = instance.border_color;
    out.border_width = instance.border_width;

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

// Signed distance to a rounded box centered at the origin. Negative
// inside. With radius 0 the inside distance is minus the distance to
// the nearest edge, so the border band below works for square corners.
fn rounded_box_sdf(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius, radius);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

// One pixel wide analytic edge coverage. See ui_rect.wgsl.
fn edge_coverage(dist: f32) -> f32 {
    return clamp(0.5 - dist / fwidth(dist), 0.0, 1.0);
}

// The blurred scene is sampled at the fragment's own screen position,
// so the quad shows the blur of exactly what sits under it. The
// instance color is a tint mixed on top by its alpha.
@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let local_pos: vec2<f32> = in.uv * in.size;
    let radius: f32 = pick_radius(local_pos, in.corner_radii);
    let dist: f32 = rounded_box_sdf(local_pos, in.size * 0.5, radius);

    let coverage: f32 = edge_coverage(dist);
    if coverage < 0.004 {
        discard;
    }

    let screen_uv = in.pos.xy / view.resolution;
    let blur = textureSampleLevel(blurred, blurred_sampler, screen_uv, 0.0);
    var rgb: vec3<f32> = mix(blur.rgb, in.color.rgb, in.color.a);

    if in.border_width > 0.0 {
        let fill: f32 = clamp(0.5 - (dist + in.border_width) / fwidth(dist), 0.0, 1.0);
        rgb = mix(in.border_color.rgb, rgb, fill);
    }

    return vec4<f32>(rgb, coverage);
}
