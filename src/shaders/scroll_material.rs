use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "722aa22d-3a74-4d0d-83d4-de57c6a735e1"]
pub struct ScrollMaterial {
	#[uniform(0)]
	pub scroll_speed : f32,
	#[uniform(0)]
	pub rect : Vec4,
	#[texture(1)]
	#[sampler(2)]
	pub texture: Handle<Image>,
}

impl ScrollMaterial {
	pub fn rect (x : f32, y : f32, w : f32, h : f32) -> Vec4 {
		Vec4::new(
			x / 1024.,
			y / 2048.,
			(x + w) / 1024.,
			(y + h) / 2048.,
		)
	}
}

impl Material2d for ScrollMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/scroll.wgsl".into()
	}
}
