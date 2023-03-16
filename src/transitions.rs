use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;
use crate::{AppState, SCREEN_HEIGHT, SCREEN_WIDTH, TRANSITION_END_COMPLETE, TRANSITION_START_COMPLETE, z};

pub struct TransitionsPlugin;

impl Plugin for TransitionsPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_state::<TransitionState>()
			.insert_resource(TransitionTo(None))
			.add_system(transition_watcher.in_set(OnUpdate(TransitionState::None)))
			.add_system(transition_start.in_schedule(OnEnter(TransitionState::Start)))
			.add_system(transition_events)
			.add_system(transition_end.in_schedule(OnEnter(TransitionState::End)))
		;
	}
}

// States
// =========================================================================

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
enum TransitionState {
	#[default]
	None,
	Start,
	End,
}

// Resources
// =========================================================================

#[derive(Resource)]
pub struct TransitionTo (pub Option<AppState>);

// Components
// =========================================================================

#[derive(Component)]
struct TransitionOverlay;

// Systems
// =========================================================================

fn transition_watcher (
	to : Res<TransitionTo>,
	current_transition_state : Res<State<TransitionState>>,
	mut transition_state: ResMut<NextState<TransitionState>>,
) {
	if to.0.is_none() || !current_transition_state.0.eq(&TransitionState::None) { return; }
	transition_state.set(TransitionState::Start);
}

fn transition_start (
	mut commands : Commands,
) {
	let start_pos = Vec3::new(-SCREEN_WIDTH, 5., z::TRANSITION);
	
	commands.spawn((
		TransitionOverlay,
		SpriteBundle {
			sprite: Sprite {
				color: Color::hex("#EBCC56").unwrap(),
				custom_size: Some(Vec2::new(-SCREEN_WIDTH, SCREEN_HEIGHT + 10.)),
				..default()
			},
			transform: Transform::from_translation(start_pos),
			..default()
		},
		Animator::new(Tween::new(
			EaseFunction::CircularIn,
			Duration::from_secs(1),
			TransformPositionLens {
				start: start_pos,
				end: Vec3::new(0., 5., z::TRANSITION),
			}
		).with_completed_event(TRANSITION_START_COMPLETE)),
	));
}

fn transition_events (
	mut app_state: ResMut<NextState<AppState>>,
	mut transition_state: ResMut<NextState<TransitionState>>,
	mut reader : EventReader<TweenCompleted>,
	mut commands : Commands,
	query : Query<Entity, With<TransitionOverlay>>,
	mut to_state : ResMut<TransitionTo>,
) {
	if let Ok(entity) = query.get_single() {
		for event in reader.iter() {
			match event.user_data {
				TRANSITION_START_COMPLETE => {
					if let Some(to) = to_state.0 {
						app_state.set(to);
						to_state.0 = None;
					}
					
					transition_state.set(TransitionState::End);
				},
				TRANSITION_END_COMPLETE => {
					commands.entity(entity).despawn_recursive();
					transition_state.set(TransitionState::None);
				},
				_ => {}
			}
		}
	}
}

fn transition_end (
	mut commands : Commands,
	query : Query<Entity, With<TransitionOverlay>>,
) {
	if let Ok(entity) = query.get_single() {
		commands.entity(entity)
			.remove::<Animator<Transform>>()
			.insert(Animator::new(Tween::new(
			EaseFunction::CircularOut,
			Duration::from_secs(1),
			TransformPositionLens {
				start: Vec3::Y * 5.,
				end: Vec3::new(SCREEN_WIDTH, 5., z::TRANSITION),
			}
		).with_completed_event(TRANSITION_END_COMPLETE)));
	}
}
