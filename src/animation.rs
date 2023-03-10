use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(animate_sprite)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
pub struct AnimationIndices {
	pub indices : Vec<usize>,
	current : usize,
}

impl AnimationIndices {
	pub fn new (indices : Vec<usize>) -> Self {
		AnimationIndices {
			current: indices[0],
			indices,
		}
	}
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer (pub Timer);

// Systems
// =========================================================================

fn animate_sprite (
	mut query : Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite)>,
	time : Res<Time>,
) {
	for (mut indices, mut timer, mut sprite) in query.iter_mut() {
		timer.tick(time.delta());
		
		if timer.just_finished() {
			indices.current += 1;
			indices.current %= indices.indices.len();
			sprite.index = indices.indices[indices.current];
		}
	}
}
