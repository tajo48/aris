//! Spawn the worlD

use bevy::prelude::*;
use bevy_psx::material::PsxMaterial;
pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_world);
    app.register_type::<World>();
}

#[derive(Event, Debug)]
pub struct SpawnWorld;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct World;

fn spawn_world(
    _trigger: Trigger<SpawnWorld>,
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PsxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let transform =
        Transform::from_scale(Vec3::splat(1.0)).with_translation(Vec3::new(0.0, 0.0, -1.0));

    commands.spawn((MaterialMeshBundle {
        mesh: asset_server.load("artefacts/psxgun.glb#Mesh0/Primitive0"),
        material: materials.add(PsxMaterial {
            color_texture: Some(asset_server.load("artefacts/psxgun.glb#Texture0")),
            // snap_amount: 40000000000.0,
            snap_amount: 40.0,
            fog_distance: Vec2::new(250.0, 750.0),

            ..Default::default()
        }),
        transform,
        ..default()
    },));
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
    // commands.spawn((Name::new("Player"),));
}
