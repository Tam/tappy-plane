use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::assets::SpriteSheet;
use crate::physics::SATCollider;
use crate::{AppState, GameState, SCREEN_WIDTH, Z_OBSTACLE};
use crate::scenes::GameRoot;

const SPAWN_OFFSET : f32 = SCREEN_WIDTH * 0.5 + 100.;
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
			.add_systems((
				move_obstacle,
				despawn_obstacle,
			).in_set(OnUpdate(AppState::Game)))
			.add_system(
				spawn_obstacle
					.in_set(OnUpdate(AppState::Game))
					.in_set(OnUpdate(GameState::Play))
			)
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
	root_query : Query<Entity, With<GameRoot>>,
	time : Res<Time>,
	mut spawner : ResMut<ObstacleSpawner>,
	mut has_run : Local<bool>,
) {
	let root = root_query.single();
	spawner.timer.tick(time.delta());
	
	if spawner.timer.just_finished() || !*has_run {
		*has_run = true;
		commands.entity(root).with_children(|commands| {
			spawn(
				commands,
				&sprite_sheet,
				SPAWN_OFFSET,
				spawner.gap_min,
				spawner.gap_max,
			);
		});
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
	commands : &mut ChildBuilder,
	sprite_sheet : &Res<SpriteSheet>,
	start_x : f32,
	gap_min : f32,
	gap_max : f32,
) {
	let mut rng = rand::thread_rng();
	let down = vec!["rockDown", "rockGrassDown"];
	let up = vec!["rock", "rockGrass"];
	
	commands.spawn((
		Transform::from_xyz(start_x, 0., Z_OBSTACLE),
		GlobalTransform::default(),
		Visibility::default(),
		ComputedVisibility::default(),
		Obstacle,
	)).with_children(|commands| {
		let gap = rng.gen_range(gap_min ..= gap_max);
		
		let top_y = rng.gen_range(0. ..= gap);
		let bottom_y = gap - top_y;
		
		// Down
		// -------------------------------------------------------------------------
		
		let mut spawn_top = |x : f32, y : f32, z : f32, sprite : &str| {
			commands.spawn((
				SpriteSheetBundle {
					texture_atlas: sprite_sheet.handle.clone(),
					sprite: sprite_sheet.get(sprite),
					transform: Transform::from_xyz(x, 119.5 + top_y + y, Z_OBSTACLE + z),
					..default()
				},
				SATCollider(vec![
					Vec2::new(-50., 119.5),
					Vec2::new(50., 119.5),
					Vec2::new(15., -119.5),
					Vec2::new(10., -119.5),
				]),
			));
		};
		
		spawn_top(
			rng.gen_range(-10.0..=10.),
			0.,
			0.,
			down.choose(&mut rng).unwrap(),
		);
		
		// Down child before
		if rng.gen_bool(0.45) {
			spawn_top(
				rng.gen_range(-80.0..=-30.),
				rng.gen_range(50.0 ..= 100.),
				0.1,
				down.choose(&mut rng).unwrap(),
			);
		}
		
		// Down child after
		if rng.gen_bool(0.45) {
			spawn_top(
				rng.gen_range(30.0..=80.),
				rng.gen_range(50.0 ..= 100.),
				0.2,
				down.choose(&mut rng).unwrap(),
			);
		}
		
		// Up
		// -------------------------------------------------------------------------
		
		let mut spawn_bottom = |x : f32, y : f32, z : f32, sprite : &str| {
			commands.spawn((
				SpriteSheetBundle {
					texture_atlas: sprite_sheet.handle.clone(),
					sprite: sprite_sheet.get(sprite),
					transform: Transform::from_xyz(x, -(119.5 + bottom_y + y), Z_OBSTACLE + z),
					..default()
				},
				SATCollider(vec![
					Vec2::new(10., 119.5),
					Vec2::new(15., 119.5),
					Vec2::new(50., -119.5),
					Vec2::new(-50., -119.5),
				]),
			));
		};
		
		spawn_bottom(
			rng.gen_range(-10.0..=10.),
			0.,
			0.,
			up.choose(&mut rng).unwrap(),
		);
		
		// Up child before
		if rng.gen_bool(0.45) {
			spawn_bottom(
				rng.gen_range(-80.0..=-30.),
				rng.gen_range(50.0 ..= 100.),
				0.1,
				up.choose(&mut rng).unwrap(),
			);
		}
		
		// Up child after
		if rng.gen_bool(0.45) {
			spawn_bottom(
				rng.gen_range(30.0..=80.),
				rng.gen_range(50.0 ..= 100.),
				0.2,
				up.choose(&mut rng).unwrap(),
			);
		}
	});
}
