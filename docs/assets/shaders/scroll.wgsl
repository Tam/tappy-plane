#import bevy_sprite::mesh2d_view_bindings

struct ScrollMaterial {
	scroll_speed : f32,
	rect : vec4<f32>,
//#ifdef SIXTEEN_BYTE_ALIGNMENT
//    // WebGL2 structs must be 16 byte aligned.
//    _webgl2_padding : f32,
//#endif
}

@group(1) @binding(0)
var<uniform> material : ScrollMaterial;

@group(1) @binding(1)
var texture : texture_2d<f32>;
@group(1) @binding(2)
var tex_sampler : sampler;

@fragment
fn fragment (
	#import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
	var uv : vec2<f32> = uv;
	uv.x += material.scroll_speed * globals.time;
	uv.x = fract(uv.x);

	uv.x = mix(material.rect.x, material.rect.z, uv.x);
	uv.y = mix(material.rect.y, material.rect.w, uv.y);

	return textureSample(texture, tex_sampler, uv);
}
