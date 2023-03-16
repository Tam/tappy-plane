mod scroll_material;
// mod slice_material;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
pub use scroll_material::ScrollMaterial;
// pub use slice_material::SliceMaterial;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(Material2dPlugin::<ScrollMaterial>::default())
			// .add_plugin(Material2dPlugin::<SliceMaterial>::default())
		;
	}
}
