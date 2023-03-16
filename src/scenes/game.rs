use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;
use crate::sprite_animation::{SpriteAnimationIndices, SpriteAnimationTimer};
use crate::{AppState, GAME_IN_ANIM_COMPLETE, GAME_OVER_ANIM_COMPLETE, GameState, Level, SCREEN_HEIGHT, SCREEN_WIDTH, Z_BACKGROUND, Z_GAME_TEXT, Z_GROUND, Z_PLANE};
use crate::assets::SpriteSheet;
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
			.add_system(early_start.in_set(OnUpdate(GameState::Enter)))
		
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
	ground_speed : Res<GroundSpeed>,
) {
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
			transform: Transform::from_xyz(0., (SCREEN_HEIGHT - top_slice) * 0.5, Z_BACKGROUND),
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
			transform: Transform::from_xyz(0., top_slice * -0.5, Z_BACKGROUND),
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
		
		commands.spawn((
			MaterialMesh2dBundle {
				mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2::new(SCREEN_WIDTH, 71.)))).into(),
				material: scroll_material_assets.add(ScrollMaterial {
					scroll_speed: ground_speed.0 * 0.001,
					rect: ScrollMaterial::rect(0., 142.3, 808. - 0.4, 71.),
					texture: sprite_sheet.texture_handle.clone(),
				}),
				transform: Transform::from_xyz(0., (SCREEN_HEIGHT - 71.) / 2. * -1., Z_GROUND),
				..default()
			},
			AABBCollider(Vec2::new(SCREEN_WIDTH, 30.), Some(Vec2::new(0., -10.))),
		));
		
		// Plane
		// -------------------------------------------------------------------------
		
		commands.spawn((
			PlaneRoot,
			Transform::from_xyz(SCREEN_WIDTH * -0.2, 0., Z_PLANE),
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
	mut query : Query<Entity, With<PlaneRoot>>,
) {
	if let Ok(entity) = query.get_single_mut() {
		let tween = Tween::new(
			EaseFunction::QuarticOut,
			Duration::from_secs(2),
			TransformPositionLens {
				start: Vec3::new(SCREEN_WIDTH * -0.8, 200., Z_PLANE),
				end: Vec3::new(SCREEN_WIDTH * -0.2, 0., Z_PLANE),
			},
		).with_completed_event(GAME_IN_ANIM_COMPLETE);
		
		commands.entity(entity).insert(Animator::new(tween));
	}
}

fn handle_anim_event (
	mut reader: EventReader<TweenCompleted>,
	mut state : ResMut<NextState<GameState>>,
) {
	for event in reader.iter() {
		match event.user_data {
			GAME_IN_ANIM_COMPLETE => state.set(GameState::Play),
			GAME_OVER_ANIM_COMPLETE => { /* Handled in dead_loop */ },
			_ => {},
		}
	}
}

// Teardown
// -------------------------------------------------------------------------

fn teardown_game (
	mut commands : Commands,
	query : Query<Entity, With<GameRoot>>,
	mut state : ResMut<NextState<GameState>>,
	mut level : ResMut<Level>,
) {
	// Remove all entities
	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}
	
	// Reset game state
	state.set(GameState::default());
	
	// Reset spawn timer
	level.spawner.timer.reset();
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
					start: Vec3::new(0., SCREEN_HEIGHT * 0.5, Z_GAME_TEXT),
					end: Vec3::new(0., 30., Z_GAME_TEXT),
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
						start: Vec3::new(0., SCREEN_HEIGHT * -0.5, Z_GAME_TEXT),
						end: Vec3::new(0., -50., Z_GAME_TEXT),
					},
				)),
			),
		));
	});
}

#[allow(clippy::too_many_arguments)]
fn dead_loop (
	mut query : Query<&mut Transform, With<Plane>>,
	death_speed : Res<DeathSpeed>,
	time : Res<Time>,
	mouse : Res<Input<MouseButton>>,
	touch : Res<Touches>,
	mut can_restart : Local<bool>,
	mut reader : EventReader<TweenCompleted>,
	mut to_state : ResMut<TransitionTo>,
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
	}
}
