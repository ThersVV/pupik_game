use crate::{
    energybars::Bar,
    plane::{create_plane, PlaneSensor},
    player::{Hidden, Player, ScreenShaker},
    Damaging, GameState, PlanesSheet, Settings,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollPlugin;

impl Plugin for CollPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(manage_single_collisions)
                .with_system(deal_damage),
        );
    }
}
fn manage_single_collisions(
    mut events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut player_q: Query<&mut Hidden, With<Player>>,
    sensor_q: Query<(&PlaneSensor, &Transform), With<PlaneSensor>>,
    plane_texture: Res<PlanesSheet>,
    bar_q: Query<&Bar, With<Bar>>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(handle1, handle2, _) => {
                let (_player, thing) = if let Ok(_player) = player_q.get(handle1.clone()) {
                    (handle1, handle2)
                } else {
                    (handle2, handle1)
                };

                let thing_cl = thing.clone();

                if let Ok((plane_sensor, transform)) = sensor_q.get(thing_cl) {
                    create_plane(
                        plane_sensor.dir,
                        transform.translation.y,
                        &mut commands,
                        &plane_texture.0,
                    );
                    commands.entity(thing_cl).despawn();
                } else if let Ok(_bar) = bar_q.get(thing_cl) {
                    commands.entity(thing_cl).despawn();
                    if let Ok(mut player) = player_q.get_single_mut() {
                        if player.energy > 40. {
                            player.energy = 100.;
                        } else {
                            player.energy += 60.;
                        }
                    };
                }
            }
            _ => {}
        }
    }
}

fn deal_damage(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut player_q: Query<(&mut Player, &mut Hidden), With<Player>>,
    dmging: Query<&Damaging, With<Damaging>>,
    mut state: ResMut<State<GameState>>,
    settings: Res<Settings>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(handle1, handle2, _) => {
                let mut func = |player: &mut (Mut<Player>, Mut<Hidden>), _dmging: &Damaging| {
                    let (player, hidden) = player;
                    if !hidden.hit {
                        player.hp -= 1;
                        hidden.hit = true;
                        hidden.hit_energy = settings.hit_resistence;
                        commands.spawn(ScreenShaker {
                            shakes: settings.shakes * 2 - 1,
                        });
                        if player.hp == 0 {
                            state
                                .set(GameState::EndScreen)
                                .expect("Unexpected state set error.");
                        }
                    }
                };
                if let (Ok(mut player), Ok(dmg)) = (
                    player_q.get_mut(handle1.clone()),
                    dmging.get(handle2.clone()),
                ) {
                    func(&mut player, dmg);
                } else if let (Ok(dmg), Ok(mut player)) = (
                    dmging.get(handle1.clone()),
                    player_q.get_mut(handle2.clone()),
                ) {
                    func(&mut player, dmg);
                };
            }
            _ => {}
        }
    }
}
