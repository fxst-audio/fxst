@group(0) @binding(0) var output_sampler: sampler;
@group(0) @binding(1) var output_frame: texture_2d<f32>;

struct Parameters {
    size: vec2<f32>,
    origin: vec2<f32>
}

@group(0) @binding(2)
var<uniform> params: Parameters;

struct FsVertex {
    @location(0) uv: vec2<f32>,
    @builtin(position) pos: vec4<f32>
}

@vertex
fn vs(@builtin(vertex_index) index: u32) -> FsVertex {
    var vertices = array<vec2<f32>, 6>(
        vec2(params.origin.x, params.origin.y),
        vec2(params.origin.x + params.size.x, params.origin.y),
        vec2(params.origin.x, params.origin.y - params.size.y),

        vec2(params.origin.x + params.size.x, params.origin.y),
        vec2(params.origin.x, params.origin.y - params.size.y),
        vec2(params.origin.x + params.size.x, params.origin.y - params.size.y)
    );

    var uvs = array<vec2<f32>, 6>(
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(0.0, 1.0),

        vec2(1.0, 0.0),
        vec2(0.0, 1.0),
        vec2(1.0, 1.0)
    );

    var out: FsVertex;
    out.pos = vec4(vertices[index], 0.0, 1.0);
    out.uv = uvs[index];
    return out;
}

@fragment
fn fs(in: FsVertex) -> @location(0) vec4<f32> {
    return textureSample(output_frame, output_sampler, in.uv);
}