use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_tweening::{Animator, Delay, EaseFunction, Tracks, Tween, TweenCompleted};
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use crate::sprite_animation::{SpriteAnimationIndices, SpriteAnimationTimer};
use crate::{AppState, BASE_LEVEL, DIST_PER_SECOND, DistanceTravelled, GAME_IN_ANIM_COMPLETE, GAME_OUT_ANIM_COMPLETE, GAME_OVER_ANIM_COMPLETE, GameState, Level, LevelTheme, SCREEN_HEIGHT, SCREEN_WIDTH, z};
use crate::assets::SpriteSheet;
use crate::obstacle::SpawnTimer;
use crate::physics::{AABBCollider, Velocity};
use crate::shaders::ScrollMaterial;
use crate::transitions::TransitionTo;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(GroundSpeed(300.))
			.insert_resource(DeathSpeed(0.))
			
			.add_system(setup_game.in_schedule(OnEnter(AppState::Game)))
			.add_system(teardown_game.in_schedule(OnExit(AppState::Game)))
			
			.add_system(animate_in.in_schedule(OnEnter(GameState::Enter)))
			.add_system(handle_anim_event.in_set(OnUpdate(AppState::Game)))
			.add_system(animate_out.in_schedule(OnEnter(GameState::Exit)))
			
			.add_system(early_start.in_set(OnUpdate(GameState::Enter)))
			.add_system(travel.in_set(OnUpdate(GameState::Play)))
		
			.add_system(dead_enter.in_schedule(OnEnter(GameState::Dead)))
			.add_system(dead_loop.in_set(OnUpdate(GameState::Dead)))
		;
	}
}

// Resources
// =========================================================================

#[derive(Resource)]
pub struct GroundSpeed (pub f32);

#[derive(Resource)]
pub struct DeathSpeed (pub f32);

// Components
// =========================================================================

#[derive(Component)]
pub struct GameRoot;

#[derive(Component)]
struct PlaneRoot;

#[derive(Component)]
struct Plane;

#[derive(Component)]
struct ProgressBarRoot;

#[derive(Component)]
struct ProgressBar;

#[derive(Component)]
struct ProgressPlane;

#[derive(Component)]
struct LevelIndex;

// Systems
// =========================================================================

// Setup
// -------------------------------------------------------------------------

fn setup_game(
	mut commands : Commands,
	sprite_sheet : Res<SpriteSheet>,
	mut mesh_assets : ResMut<Assets<Mesh>>,
	mut scroll_material_assets : ResMut<Assets<ScrollMaterial>>,
	mut state : ResMut<NextState<GameState>>,
	mut ground_speed : ResMut<GroundSpeed>,
	level : Res<Level>,
	mut timer : ResMut<SpawnTimer>,
	mut death_speed : ResMut<DeathSpeed>,
	mut distance_travelled : ResMut<DistanceTravelled>,
) {
	// Reset counters
	death_speed.0 = 0.;
	distance_travelled.0 = 0.;
	
	// Setup timer
	timer.0.set_duration(Duration::from_secs_f32(level.spawner.interval));
	
	// Ground speed
	let computed_ground_speed = 300. + level.spawner.speed;
	ground_speed.0 = computed_ground_speed;
	
	let theme = level.theme;
	
	commands.spawn((
		GameRoot,
		Transform::default(),
		GlobalTransform::default(),
		Visibility::default(),
		ComputedVisibility::default(),
	)).with_children(|commands| {
		// Background
		// -------------------------------------------------------------------------
		
		// Top
		
		let top_slice = 210.15;
		
		commands.spawn(MaterialMesh2dBundle {
			mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2::new(SCREEN_WIDTH, top_slice)))).into(),
			material: scroll_material_assets.add(ScrollMaterial {
				scroll_speed: 0.05,
				rect: ScrollMaterial::rect(0., 355., 800. - 0.4, top_slice),
				texture: sprite_sheet.texture_handle.clone(),
			}),
			transform: Transform::from_xyz(0., (SCREEN_HEIGHT - top_slice) * 0.5, z::BACKGROUND),
			..default()
		});
		
		// Bottom
		
		commands.spawn(MaterialMesh2dBundle {
			mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT - top_slice)))).into(),
			material: scroll_material_assets.add(ScrollMaterial {
				scroll_speed: 0.1,
				rect: ScrollMaterial::rect(0., 355. + top_slice, 800. - 0.4, 480. - top_slice),
				texture: sprite_sheet.texture_handle.clone(),
			}),
			transform: Transform::from_xyz(0., top_slice * -0.5, z::BACKGROUND),
			..default()
		});
		
		// Ceiling Collider
		// -------------------------------------------------------------------------
		
		commands.spawn((
			Transform::from_xyz(0., SCREEN_HEIGHT * 0.5 + 15., 0.),
			GlobalTransform::default(),
			AABBCollider(Vec2::new(SCREEN_WIDTH, 30.), None),
		));
		
		// Ground
		// -------------------------------------------------------------------------
		
		let ground_y = match theme {
			LevelTheme::Grass => 142.3,
			LevelTheme::Snow => 213.,
			LevelTheme::Ice => 71.,
		};
		
		commands.spawn((
			MaterialMesh2dBundle {
				mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2::new(SCREEN_WIDTH, 71.)))).into(),
				material: scroll_material_assets.add(ScrollMaterial {
					scroll_speed: computed_ground_speed * 0.001,
					rect: ScrollMaterial::rect(0., ground_y, 808. - 0.4, 71.),
					texture: sprite_sheet.texture_handle.clone(),
				}),
				transform: Transform::from_xyz(0., (SCREEN_HEIGHT - 71.) / 2. * -1., z::GROUND),
				..default()
			},
			AABBCollider(Vec2::new(SCREEN_WIDTH, 30.), Some(Vec2::new(0., -10.))),
		));
		
		// Plane
		// -------------------------------------------------------------------------
		
		commands.spawn((
			PlaneRoot,
			Transform::from_xyz(SCREEN_WIDTH * -0.2, 0., z::PLANE),
			GlobalTransform::default(),
			Visibility::default(),
			ComputedVisibility::default(),
		)).with_children(|commands| {
			commands.spawn((
				Plane,
				SpriteSheetBundle {
					texture_atlas: sprite_sheet.handle.clone(),
					sprite: sprite_sheet.get("planeBlue1"),
					transform: Transform::from_xyz(88. * -0.5, 73. * 0.5, 0.),
					..default()
				},
				SpriteAnimationIndices::new(vec![
					sprite_sheet.get("planeBlue1").index,
					sprite_sheet.get("planeBlue2").index,
					sprite_sheet.get("planeBlue3").index,
				]),
				SpriteAnimationTimer(Timer::from_seconds(0.04, TimerMode::Repeating)),
				Velocity::default(),
				AABBCollider(
					Vec2::new(80., 73.) * 0.6,
					Some(Vec2::new(10., 0.)),
				),
			));
		});
		
		// Progress Bar
		// -------------------------------------------------------------------------
		
		commands.spawn((
			ProgressBarRoot,
			Transform::from_xyz(0., SCREEN_HEIGHT * 0.7, z::UI),
			GlobalTransform::default(),
			Visibility::default(),
			ComputedVisibility::default(),
		)).with_children(|commands| {
			// Background
			commands.spawn(SpriteBundle {
				sprite: Sprite {
					color: Color::hex("#BC9C33").unwrap(),
					custom_size: Some(Vec2::new(SCREEN_WIDTH - 70., 25.)),
					..default()
				},
				transform: Transform::from_xyz(0., -40., 0.),
				..default()
			});
			
			// Fill
			commands.spawn((
				ProgressBar,
				SpriteBundle {
					sprite: Sprite {
						color: Color::hex("#EBCC56").unwrap(),
						custom_size: Some(Vec2::new(SCREEN_WIDTH - 80., 15.)),
						..default()
					},
					transform: Transform::from_xyz(0., -40., 1.)
						.with_scale(Vec3::new(0., 1., 1.)),
					..default()
				},
			));
			
			// Tiny Plane
			commands.spawn((
				ProgressPlane,
				SpriteSheetBundle {
					texture_atlas: sprite_sheet.handle.clone(),
					sprite: sprite_sheet.get("planeBlue1"),
					transform: Transform::from_xyz(SCREEN_WIDTH * -0.5 + 45., -38., 2.).with_scale(Vec3::splat(0.5)),
					..default()
				},
				SpriteAnimationIndices::new(vec![
					sprite_sheet.get("planeBlue1").index,
					sprite_sheet.get("planeBlue2").index,
					sprite_sheet.get("planeBlue3").index,
				]),
				SpriteAnimationTimer(Timer::from_seconds(0.04, TimerMode::Repeating)),
			));
		});
		
		// Level Index
		// -------------------------------------------------------------------------
		
		commands.spawn((
			LevelIndex,
			SpatialBundle::from_transform(Transform::from_xyz(0., 0., z::UI)),
		)).with_children(|commands| {
			let index = level.index.to_string();
			let mut i = index.len() / 2;
			for c in index.chars() {
				let mut name = String::from("number");
				name.push(c);
				
				commands.spawn((
					SpriteSheetBundle {
						texture_atlas: sprite_sheet.handle.clone(),
						sprite: sprite_sheet.get(name.as_str()),
						transform: Transform::from_xyz(
							45. * (i as f32) - ((45. * (index.len() as f32)) * 0.25),
							30.,
							0.
						),
						..default()
					},
				));
				
				i += 1;
			}
		});
	});
	
	state.set(GameState::Enter);
}

fn early_start (
	mut state : ResMut<NextState<GameState>>,
	mouse : Res<Input<MouseButton>>,
	touch : Res<Touches>,
) {
	if mouse.just_pressed(MouseButton::Left) || touch.any_just_pressed() {
		state.set(GameState::Play);
	}
}

// Animations
// -------------------------------------------------------------------------

fn animate_in (
	mut commands : Commands,
	mut plane_query : Query<Entity, With<PlaneRoot>>,
	mut progress_query : Query<Entity, (With<ProgressBarRoot>, Without<PlaneRoot>)>,
	mut level_index_query : Query<Entity, (With<LevelIndex>, Without<PlaneRoot>, Without<ProgressBarRoot>)>,
) {
	if let Ok(entity) = plane_query.get_single_mut() {
		let tween = Tween::new(
			EaseFunction::QuarticOut,
			Duration::from_secs(2),
			TransformPositionLens {
				start: Vec3::new(SCREEN_WIDTH * -0.8, 200., z::PLANE),
				end: Vec3::new(SCREEN_WIDTH * -0.2, 0., z::PLANE),
			},
		).with_completed_event(GAME_IN_ANIM_COMPLETE);
		
		commands.entity(entity).insert(Animator::new(tween));
	}
	
	if let Ok(entity) = progress_query.get_single_mut() {
		let tween = Delay::new(Duration::from_secs_f32(0.5)).then(Tween::new(
			EaseFunction::QuarticOut,
			Duration::from_secs_f32(1.5),
			TransformPositionLens {
				start: Vec3::new(0., SCREEN_HEIGHT * 0.7, z::UI),
				end: Vec3::new(0., SCREEN_HEIGHT * 0.5, z::UI),
			},
		));
		
		commands.entity(entity).insert(Animator::new(tween));
	}
	
	if let Ok(entity) = level_index_query.get_single_mut() {
		let tween = Tween::new(
			EaseFunction::QuadraticInOut,
			Duration::from_secs(3),
			TransformPositionLens {
				start: Vec3::new(0., SCREEN_HEIGHT * 0.7, z::UI),
				end: Vec3::new(0., SCREEN_HEIGHT * -0.7, z::UI),
			},
		);
		
		commands.entity(entity).insert(Animator::new(tween));
	}
}

fn handle_anim_event (
	mut reader: EventReader<TweenCompleted>,
	mut state : ResMut<NextState<GameState>>,
	mut to_state : ResMut<TransitionTo>,
	mut level : ResMut<Level>,
) {
	for event in reader.iter() {
		match event.user_data {
			GAME_IN_ANIM_COMPLETE => { state.set(GameState::Play) }
			GAME_OUT_ANIM_COMPLETE => {
				to_state.0 = Some(AppState::Menu);
				level.index += 1;
				level.theme = rand::random();
				level.distance += 100.;
				level.spawner.speed += 5.;
				level.spawner.interval -= 0.1;
			}
			GAME_OVER_ANIM_COMPLETE => { /* Handled in dead_loop */ }
			_ => {}
		}
	}
}

fn animate_out (
	mut commands : Commands,
	mut query : Query<Entity, With<PlaneRoot>>,
	mut progress_query : Query<Entity, (With<ProgressBarRoot>, Without<PlaneRoot>)>,
) {
	if let Ok(entity) = query.get_single_mut() {
		let tween = Tween::new(
			EaseFunction::CircularIn,
			Duration::from_secs(2),
			TransformPositionLens {
				start: Vec3::new(SCREEN_WIDTH * -0.2, 0., z::OBSTACLE - 1.),
				end: Vec3::new(SCREEN_WIDTH * 0.8, 200., z::OBSTACLE - 1.),
			},
		);
		
		let scale_tween = Tween::new(
			EaseFunction::CircularIn,
			Duration::from_secs(2),
			TransformScaleLens {
				start: Vec3::new(1., 1., 1.),
				end: Vec3::new(0., 0., 1.),
			},
		).with_completed_event(GAME_OUT_ANIM_COMPLETE);
		
		commands.entity(entity).insert(
			Animator::new(Tracks::new([
				tween,
				scale_tween,
			]))
		);
	}
	
	if let Ok(entity) = progress_query.get_single_mut() {
		let tween = Tween::new(
			EaseFunction::QuarticIn,
			Duration::from_secs(1),
			TransformPositionLens {
				start: Vec3::new(0., SCREEN_HEIGHT * 0.5, z::UI),
				end: Vec3::new(0., SCREEN_HEIGHT * 0.7, z::UI),
			},
		);
		
		commands.entity(entity).insert(Animator::new(tween));
	}
}

// Travel
// -------------------------------------------------------------------------

fn travel (
	mut distance_travelled : ResMut<DistanceTravelled>,
	time : Res<Time>,
	level : Res<Level>,
	mut state : ResMut<NextState<GameState>>,
	mut bar : Query<&mut Transform, With<ProgressBar>>,
	mut plane : Query<&mut Transform, (With<ProgressPlane>, Without<ProgressBar>)>,
) {
	let mut bar = bar.single_mut();
	let mut plane = plane.single_mut();
	distance_travelled.0 += DIST_PER_SECOND * time.delta_seconds();
	
	bar.scale.x = distance_travelled.0 / level.distance;
	bar.translation.x = ((SCREEN_WIDTH - 80.) * -0.5) * (1. - bar.scale.x);
	plane.translation.x = (SCREEN_WIDTH * -0.5 + 45.) + (SCREEN_WIDTH - 80.) * bar.scale.x;
	
	if distance_travelled.0 >= level.distance {
		state.set(GameState::Exit);
	}
}

// Teardown
// -------------------------------------------------------------------------

fn teardown_game (
	mut commands : Commands,
	query : Query<Entity, With<GameRoot>>,
	mut state : ResMut<NextState<GameState>>,
	mut timer : ResMut<SpawnTimer>,
) {
	// Remove all entities
	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}
	
	// Reset game state
	state.set(GameState::default());
	
	// Reset spawn timer
	timer.0.reset();
}

// Dead
// -------------------------------------------------------------------------

fn dead_enter (
	mut commands : Commands,
	root_query : Query<Entity, With<GameRoot>>,
	sprite_sheet : Res<SpriteSheet>,
) {
	let root = root_query.single();
	
	commands.entity(root).with_children(|commands| {
		// Game over text
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: sprite_sheet.handle.clone(),
				sprite: sprite_sheet.get("textGameOver"),
				..default()
			},
			Animator::new(Tween::new(
				EaseFunction::QuarticOut,
				Duration::from_secs(1),
				TransformPositionLens {
					start: Vec3::new(0., SCREEN_HEIGHT * 0.5, z::GAME_TEXT),
					end: Vec3::new(0., 30., z::GAME_TEXT),
				},
			).with_completed_event(GAME_OVER_ANIM_COMPLETE)),
		));
		
		// Tap prompt
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: sprite_sheet.handle.clone(),
				sprite: sprite_sheet.get("tap"),
				..default()
			},
			SpriteAnimationIndices::new(vec![
				sprite_sheet.get("tap").index,
				sprite_sheet.get("tapTick").index,
			]),
			SpriteAnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
			Animator::new(
				Delay::new(Duration::from_millis(500)).then(Tween::new(
					EaseFunction::QuarticOut,
					Duration::from_secs(1),
					TransformPositionLens {
						start: Vec3::new(0., SCREEN_HEIGHT * -0.5, z::GAME_TEXT),
						end: Vec3::new(0., -50., z::GAME_TEXT),
					},
				)),
			),
		));
	});
}

fn dead_loop (
	mut query : Query<&mut Transform, With<Plane>>,
	death_speed : Res<DeathSpeed>,
	time : Res<Time>,
	mouse : Res<Input<MouseButton>>,
	touch : Res<Touches>,
	mut can_restart : Local<bool>,
	mut reader : EventReader<TweenCompleted>,
	mut to_state : ResMut<TransitionTo>,
	mut level : ResMut<Level>,
) {
	if let Ok(mut transform) = query.get_single_mut() {
		transform.translation.x -= death_speed.0 * time.delta_seconds();
	}
	
	for event in reader.iter() {
		if event.user_data == GAME_OVER_ANIM_COMPLETE {
			*can_restart = true;
		}
	}
	
	if *can_restart && (mouse.just_pressed(MouseButton::Left) || touch.any_just_pressed()) {
		to_state.0 = Some(AppState::Menu);
		*level = BASE_LEVEL;
	}
}
