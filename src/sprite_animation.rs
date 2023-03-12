use bevy::prelude::*;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(animate_sprite)
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
pub struct SpriteAnimationIndices {
	pub indices : Vec<usize>,
	current : usize,
}

impl SpriteAnimationIndices {
	pub fn new (indices : Vec<usize>) -> Self {
		SpriteAnimationIndices {
			current: indices[0],
			indices,
		}
	}
}

#[derive(Component, Deref, DerefMut)]
pub struct SpriteAnimationTimer(pub Timer);

// Systems
// =========================================================================

fn animate_sprite (
	mut query : Query<(&mut SpriteAnimationIndices, &mut SpriteAnimationTimer, &mut TextureAtlasSprite)>,
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
