use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "20ed177f-fb8c-451a-a688-0978e4147e8e"]
pub struct SliceMaterial {
	#[uniform(0)]
	pub border : Vec4,
	#[uniform(0)]
	pub rect : Vec4,
	#[texture(1)]
	#[sampler(2)]
	pub texture: Handle<Image>,
}

impl SliceMaterial {
	pub fn rect (x : f32, y : f32, w : f32, h : f32) -> Vec4 {
		Vec4::new(
			x,
			y,
			x + w,
			y + h,
		)
	}
}

impl Material2d for SliceMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/slice.wgsl".into()
	}
}
