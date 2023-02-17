use crate::{
    energybars::EnergyBar,
    plane::{create_plane, PlaneSensor},
    player::{Hidden, Player, ScreenShaker},
    GameState, PlanesSheet, Settings,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// [Plugin] taking care of collision related functionalities. This plugin contains
/// * [Damaging]
/// * [manage_special_collisions]
/// * [deal_damage]
pub struct CollPlugin;

///Labels an [Entity] which can damage the player by collision.
#[derive(Component)]
pub struct Damaging;

impl Plugin for CollPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game) //Both functions activate only if unicorn colides
                .with_system(manage_special_collisions)
                .with_system(deal_damage),
        );
    }
}

/// Takes care of all collisions with [entities](Entity) that have a unique special effect. It is run on update in the [GameState::Game].
/// # Arguments
/// * `commands` - [Commands].
/// * `events` - Reads [CollisionEvent]s.
/// * `player_q` - [Query] for [entities](Entity) with the [Player] component.
/// * `plane_sensor_q` - [Query] for [entities](Entity) with the [PlaneSensor] component.
/// * `energybar_q` - [Query] for [entities](Entity) with the [EnergyBar] component.
/// * `plane_texture` - [Resource] containing handle for plane [TextureAtlas].
fn manage_special_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut player_q: Query<&mut Hidden, With<Player>>,
    plane_sensor_q: Query<(&PlaneSensor, &Transform), With<PlaneSensor>>,
    energybar_q: Query<&EnergyBar, With<EnergyBar>>,
    plane_texture: Res<PlanesSheet>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(handle1, handle2, _) = event {
            //properly name handles
            let (_player, thing) = if let Ok(_player) = player_q.get(*handle1) {
                (handle1, handle2)
            } else {
                (handle2, handle1)
            };

            if let Ok((plane_sensor, transform)) = plane_sensor_q.get(*thing) {
                create_plane(
                    plane_sensor.dir,
                    transform.translation.y,
                    &mut commands,
                    &plane_texture.0,
                );
                commands.entity(*thing).despawn();
            } else if let Ok(_bar) = energybar_q.get(*thing) {
                if let Ok(mut player) = player_q.get_single_mut() {
                    if player.energy > 40. {
                        player.energy = 100.;
                    } else {
                        player.energy += 60.;
                    }
                };
                commands.entity(*thing).despawn();
            }
        }
    }
}

/// Takes care of all collisions with [entities](Entity) that have a unique special effect. It is run on update in the [GameState::Game].
/// # Arguments
/// * `commands` - [Commands].
/// * `events` - Reads [CollisionEvent]s.
/// * `player_q` - [Query] for [entities](Entity) with the [Player] component.
/// * `damaging_q` - [Query] for [entities](Entity) with the [Damaging] component.
/// * `state` - Resource containing [State]. This game's states are defined in the [GameState] enum.
/// * `settings` - [Resource] containing [Settings].
fn deal_damage(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut player_q: Query<(&mut Player, &mut Hidden), With<Player>>,
    damaging_q: Query<&Damaging, With<Damaging>>,
    mut state: ResMut<State<GameState>>,
    settings: Res<Settings>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(handle1, handle2, _) = event {
            //identify handles
            let ((mut player, mut hidden), damaging_handle) =
                if let Ok(player) = player_q.get_mut(*handle1) {
                    (player, handle2)
                } else {
                    (
                        player_q
                            .get_mut(*handle2)
                            .expect("Non-player collider activated collision function."),
                        handle1,
                    )
                };

            if let Ok(_dmging) = damaging_q.get(*damaging_handle) {
                if !hidden.hit {
                    player.hp -= 1;
                    hidden.hit = true;
                    hidden.hit_energy = settings.hit_resistence;

                    commands.spawn(ScreenShaker {
                        shakes: settings.shakes * 2 - 1, //must be odd to not shift camera
                    });

                    if player.hp < 0 {
                        state
                            .set(GameState::EndScreen)
                            .expect("Unexpected state set error.");
                    }
                }
            }
        }
    }
}
