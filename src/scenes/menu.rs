use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, EaseFunction, Tween};
use bevy_tweening::lens::TransformPositionLens;
use rand::Rng;
use crate::sprite_animation::{SpriteAnimationIndices, SpriteAnimationTimer};
use crate::{AppState, SCREEN_HEIGHT};
use crate::assets::SpriteSheet;
use crate::transitions::TransitionTo;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(menu_setup.in_schedule(OnEnter(AppState::Menu)))
			.add_system(menu_loop.in_set(OnUpdate(AppState::Menu)))
			.add_system(menu_teardown.in_schedule(OnExit(AppState::Menu)))
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
struct MenuRoot;

// Systems
// =========================================================================

fn menu_setup (
	mut commands : Commands,
	sprite_sheet : Res<SpriteSheet>,
) {
	commands.spawn((
		MenuRoot,
		Visibility::default(),
		ComputedVisibility::default(),
		Transform::default(),
		GlobalTransform::default(),
	))
	.with_children(|commands| {
		let mut rng = rand::thread_rng();
		
		let mut spawn = |o : f32, l : char| {
			debug_assert!(l.is_uppercase(), "Char must be upper case!");
			let mut c = String::from("letter");
			c.push(l);
			
			let pos = Transform::from_xyz(o * 40., SCREEN_HEIGHT * 0.6, 0.)
				.with_rotation(Quat::from_rotation_z(
					rng.gen_range(-5.0f32 ..= 5.0).to_radians()
				));
			
			let mut end = pos.translation;
			end.y = 20.;
			
			commands.spawn((
				SpriteSheetBundle {
					texture_atlas: sprite_sheet.handle.clone(),
					sprite: sprite_sheet.get(c.as_str()),
					transform: pos,
					..default()
				},
				Animator::new(Delay::new(
					Duration::from_millis(rng.gen_range(1..1000))
				).then(Tween::new(
					EaseFunction::BounceOut,
					Duration::from_secs(1),
					TransformPositionLens {
						start: pos.translation,
						end,
					},
				))),
			));
		};
		
		spawn(-5., 'T');
		spawn(-4., 'A');
		spawn(-3., 'P');
		spawn(-2., 'P');
		spawn(-1., 'Y');
		
		spawn(1., 'P');
		spawn(2., 'L');
		spawn(3., 'A');
		spawn(4., 'N');
		spawn(5., 'E');
		
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: sprite_sheet.handle.clone(),
				sprite: sprite_sheet.get("tap"),
				transform: Transform::from_xyz(0., SCREEN_HEIGHT * -0.7, 0.),
				..default()
			},
			SpriteAnimationIndices::new(vec![
				sprite_sheet.get("tap").index,
				sprite_sheet.get("tapTick").index,
			]),
			SpriteAnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
			Animator::new(Delay::new(Duration::from_secs(1)).then(Tween::new(
				EaseFunction::QuarticOut,
				Duration::from_secs(1),
				TransformPositionLens {
					start: Vec3::new(0., SCREEN_HEIGHT * -0.5, 0.),
					end: Vec3::new(0., -80., 0.),
				},
			))),
		));
	});
}

fn menu_loop (
	mouse : Res<Input<MouseButton>>,
	touch : Res<Touches>,
	mut to_state : ResMut<TransitionTo>,
) {
	if mouse.just_pressed(MouseButton::Left) || touch.any_just_pressed() {
		to_state.0 = Some(AppState::Game);
	}
}

fn menu_teardown (
	mut commands : Commands,
	query : Query<Entity, With<MenuRoot>>,
) {
	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}
}
