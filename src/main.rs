mod assets;
mod animation;
mod physics;
#[cfg(feature = "debug")]
mod debug;
mod shaders;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::animation::{AnimationIndices, AnimationPlugin, AnimationTimer};
use crate::assets::{AssetsPlugin, SpriteSheet};
#[cfg(feature = "debug")]
use crate::debug::DebugPlugin;
use crate::physics::{Collider, PhysicsPlugin, Velocity};
use crate::shaders::{ScrollMaterial, ShadersPlugin};

const SCREEN_WIDTH : f32 = 800.;
const SCREEN_HEIGHT : f32 = 480.;

fn main() {
    let mut app = App::new();
    
    app
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tappy Plane".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ShadersPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
    ;
    
    #[cfg(feature = "debug")]
    app.add_plugin(DebugPlugin);
    
    app.run();
}

fn setup (
    mut commands : Commands,
    sprite_sheet: Res<SpriteSheet>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut scroll_material_assets: ResMut<Assets<ScrollMaterial>>,
) {
    // Camera
    // =========================================================================
    
    commands.spawn(Camera2dBundle::default());
    
    // Sprites
    // =========================================================================
    
    // Background
    // -------------------------------------------------------------------------
    
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)))).into(),
        material: scroll_material_assets.add(ScrollMaterial {
            scroll_speed: 0.1,
            rect: ScrollMaterial::rect(0., 355., 800. - 0.3, 480.),
            texture: sprite_sheet.texture_handle.clone(),
        }),
        transform: Transform::from_xyz(0., 0., 1.0),
        ..default()
    });
    
    // Ceiling Collider
    // -------------------------------------------------------------------------
    
    commands.spawn((
        Transform::from_xyz(0., SCREEN_HEIGHT * 0.5 + 15., 0.),
        GlobalTransform::default(),
        Collider(Vec2::new(SCREEN_WIDTH, 30.)),
    ));
    
    // Ground
    // -------------------------------------------------------------------------
    
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2::new(SCREEN_WIDTH, 71.)))).into(),
            material: scroll_material_assets.add(ScrollMaterial {
                scroll_speed: 0.3,
                rect: ScrollMaterial::rect(0., 142.3, 808., 71.),
                texture: sprite_sheet.texture_handle.clone(),
            }),
            transform: Transform::from_xyz(0., (SCREEN_HEIGHT - 71.) / 2. * -1., 2.),
            ..default()
        },
        Collider(Vec2::new(SCREEN_WIDTH, 30.)),
    ));
    
    // Plane
    // -------------------------------------------------------------------------
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprite_sheet.handle.clone(),
            sprite: sprite_sheet.get("planeBlue1"),
            transform: Transform::from_xyz(SCREEN_WIDTH * -0.2, 0., 2.),
            ..default()
        },
        AnimationIndices::new(vec![
            sprite_sheet.get("planeBlue1").index,
            sprite_sheet.get("planeBlue2").index,
            sprite_sheet.get("planeBlue3").index,
        ]),
        AnimationTimer(Timer::from_seconds(0.04, TimerMode::Repeating)),
        Velocity::default(),
        Collider(Vec2::new(88., 73.) * 0.6),
    ));
    
}
