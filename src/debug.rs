use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin, DebugShapes};
use crate::physics::{AABBCollider, SATCollider};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(DebugLinesPlugin::default())
			.add_system(visualise_aabb_colliders)
			.add_system(visualise_sat_colliders)
		;
	}
}

// Systems
// =========================================================================

fn visualise_aabb_colliders(
	query : Query<(&GlobalTransform, &AABBCollider)>,
	mut shapes : ResMut<DebugShapes>,
) {
	for (transform, collider) in &query {
		shapes
			.rect()
			.position(transform.translation() + collider.1.unwrap_or(Vec2::ZERO).extend(0.))
			.size(collider.0)
			.color(Color::RED)
		;
	}
}

fn visualise_sat_colliders(
	query : Query<(&GlobalTransform, &SATCollider)>,
	mut lines : ResMut<DebugLines>,
) {
	for (transform, collider) in &query {
		let t = transform.translation().truncate();
		
		for a in 0..collider.0.len() {
			let pa = collider.0[a];
			let pb = collider.0[(a + 1) % collider.0.len()];
			
			lines.line_colored(
				(pa + t).extend(0.),
				(pb + t).extend(0.),
				0.,
				Color::RED,
			);
		}
	}
}
