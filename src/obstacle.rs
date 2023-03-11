use bevy::prelude::*;
use rand::Rng;
use crate::assets::SpriteSheet;
use crate::physics::SATCollider;
use crate::{SCREEN_WIDTH, Z_OBSTACLE};

const SPAWN_OFFSET : f32 = SCREEN_WIDTH * 0.5 + 200.;
const NEG_SPAWN_OFFSET : f32 = SCREEN_WIDTH * -0.5 - 200.;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ObstacleSpawner {
				speed: 150.,
				timer: Timer::from_seconds(2., TimerMode::Repeating),
				gap_min: 150.,
				gap_max: 200.,
			})
			.add_system(spawn_obstacle)
			.add_system(move_obstacle)
			.add_system(despawn_obstacle)
		;
	}
}

// Resources
// =========================================================================

#[derive(Resource)]
pub struct ObstacleSpawner {
	pub speed : f32,
	pub timer : Timer,
	pub gap_min : f32,
	pub gap_max : f32,
}

// Components
// =========================================================================

#[derive(Component)]
pub struct Obstacle;

// Systems
// =========================================================================

pub fn spawn_obstacle (
	mut commands : Commands,
	sprite_sheet : Res<SpriteSheet>,
	time : Res<Time>,
	mut spawner : ResMut<ObstacleSpawner>,
) {
	spawner.timer.tick(time.delta());
	
	if spawner.timer.just_finished() {
		spawn(
			&mut commands,
			&sprite_sheet,
			SPAWN_OFFSET,
			spawner.gap_min,
			spawner.gap_max,
		);
	}
}

pub fn move_obstacle (
	mut query : Query<&mut Transform, With<Obstacle>>,
	time : Res<Time>,
	spawner : Res<ObstacleSpawner>,
) {
	for mut transform in &mut query {
		transform.translation.x -= spawner.speed * time.delta_seconds();
	}
}

pub fn despawn_obstacle (
	mut commands : Commands,
	query : Query<(Entity, &Transform), With<Obstacle>>,
) {
	for (entity, transform) in &query {
		if transform.translation.x < NEG_SPAWN_OFFSET {
			commands.entity(entity).despawn_recursive();
		}
	}
}

// Helpers
// =========================================================================

fn spawn(
	commands : &mut Commands,
	sprite_sheet : &Res<SpriteSheet>,
	start_x : f32,
	gap_min : f32,
	gap_max : f32,
) {
	let mut rng = rand::thread_rng();
	
	commands.spawn((
		Transform::from_xyz(start_x, 0., Z_OBSTACLE),
		GlobalTransform::default(),
		Visibility::default(),
		ComputedVisibility::default(),
		Obstacle,
	)).with_children(|commands| {
		let half_min = gap_min * 0.5;
		let half_max = gap_max * 0.5;
		
		// TODO: change this so the obstacles are placed a random dist apart, then shifted
		//   up/down randomly by the remainder of the max height
		let top_y = rng.gen_range(half_min..=half_max);
		let bottom_y = rng.gen_range(half_min..=half_max);
		
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: sprite_sheet.handle.clone(),
				sprite: sprite_sheet.get("rockDown"),
				transform: Transform::from_xyz(rng.gen_range(-10.0..=10.), 119.5 + top_y, Z_OBSTACLE),
				..default()
			},
			SATCollider(vec![
				Vec2::new(-50., 119.5),
				Vec2::new(50., 119.5),
				Vec2::new(15., -119.5),
				Vec2::new(10., -119.5),
			]),
		));
		
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: sprite_sheet.handle.clone(),
				sprite: sprite_sheet.get("rock"),
				transform: Transform::from_xyz(rng.gen_range(-10.0..=10.), -(119.5 + bottom_y), Z_OBSTACLE),
				..default()
			},
			SATCollider(vec![
				Vec2::new(10., 119.5),
				Vec2::new(15., 119.5),
				Vec2::new(50., -119.5),
				Vec2::new(-50., -119.5),
			]),
		));
	});
}
