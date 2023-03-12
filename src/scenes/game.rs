use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::animation::{AnimationIndices, AnimationTimer};
use crate::{AppState, GameState, SCREEN_HEIGHT, SCREEN_WIDTH, Z_BACKGROUND, Z_GROUND, Z_PLANE};
use crate::assets::SpriteSheet;
use crate::physics::{AABBCollider, Velocity};
use crate::shaders::ScrollMaterial;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(setup_game.in_schedule(OnEnter(AppState::Game)))
			.add_system(teardown_game.in_schedule(OnExit(AppState::Game)))
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
struct GameRoot;

// Systems
// =========================================================================

fn setup_game(
	mut commands : Commands,
	sprite_sheet : Res<SpriteSheet>,
	mut mesh_assets : ResMut<Assets<Mesh>>,
	mut scroll_material_assets : ResMut<Assets<ScrollMaterial>>,
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
			AABBCollider(Vec2::new(SCREEN_WIDTH, 30.)),
		));
		
		// Ground
		// -------------------------------------------------------------------------
		
		commands.spawn((
			MaterialMesh2dBundle {
				mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2::new(SCREEN_WIDTH, 71.)))).into(),
				material: scroll_material_assets.add(ScrollMaterial {
					scroll_speed: 0.3,
					rect: ScrollMaterial::rect(0., 142.3, 808. - 0.4, 71.),
					texture: sprite_sheet.texture_handle.clone(),
				}),
				transform: Transform::from_xyz(0., (SCREEN_HEIGHT - 71.) / 2. * -1., Z_GROUND),
				..default()
			},
			AABBCollider(Vec2::new(SCREEN_WIDTH, 30.)),
		));
		
		// Plane
		// -------------------------------------------------------------------------
		
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: sprite_sheet.handle.clone(),
				sprite: sprite_sheet.get("planeBlue1"),
				transform: Transform::from_xyz(SCREEN_WIDTH * -0.2, 0., Z_PLANE),
				..default()
			},
			AnimationIndices::new(vec![
				sprite_sheet.get("planeBlue1").index,
				sprite_sheet.get("planeBlue2").index,
				sprite_sheet.get("planeBlue3").index,
			]),
			AnimationTimer(Timer::from_seconds(0.04, TimerMode::Repeating)),
			Velocity::default(),
			AABBCollider(Vec2::new(88., 73.) * 0.6),
		));
	});
}

fn teardown_game (
	mut commands : Commands,
	query : Query<Entity, With<GameRoot>>,
	mut state : ResMut<NextState<GameState>>,
) {
	// Remove all entities
	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}
	
	// Reset game state
	state.set(GameState::default());
}
