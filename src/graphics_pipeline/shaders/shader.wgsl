struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    // Fullscreen triangle vertices
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0), // bottom-left
        vec2<f32>( 3.0, -1.0), // bottom-right (offscreen)
        vec2<f32>(-1.0,  3.0), // top-left (offscreen)
    );

    // Colors assigned per-vertex
    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0), // red
        vec3<f32>(0.0, 1.0, 0.0), // green
        vec3<f32>(0.0, 0.0, 1.0), // blue
    );

    let pos = positions[in_vertex_index];
    var out: VertexOutput;
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.color = colors[in_vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0); // Triangle color
}
