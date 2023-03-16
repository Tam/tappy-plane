#import bevy_sprite::mesh2d_view_bindings

struct SliceMaterial {
	border : vec4<f32>,
	rect : vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material : SliceMaterial;

@group(1) @binding(1)
var texture : texture_2d<f32>;
@group(1) @binding(2)
var tex_sampler : sampler;


@fragment
fn fragment (
	#import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
	var uv : vec2<f32> = uv;

	var texture_size_int : vec2<i32> = textureDimensions(texture);
	var texture_size : vec2<f32> = vec2<f32>(
		f32(texture_size_int.x),
		f32(texture_size_int.y),
	);

	uv.x = mix(material.rect.x / texture_size.x, material.rect.z / texture_size.x, uv.x);
    uv.y = mix(material.rect.y / texture_size.y, material.rect.w / texture_size.y, uv.y);

    // FIXME: plz why no 9-slice

	return textureSample(texture, tex_sampler, uv);
}
