use crate::{text::Score, GameState, Settings, Speed};
use bevy::prelude::*;
pub struct EndScreenPlugin;

#[derive(Component)]
struct EndScreen;

#[derive(Component)]
struct EndScreenBckg;

#[derive(Component)]
struct EndScreenCredits;

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct LoadTimer {
    timer: Timer,
}

impl Plugin for EndScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::EndScreen).with_system(spawn_endscreen))
            .add_system_set(
                SystemSet::on_update(GameState::EndScreen)
                    .with_system(load_main_menu)
                    .with_system(click2play),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::EndScreen)
                    .with_system(despawn_endscreen)
                    .with_system(reset_speed),
            );
    }
}

fn spawn_endscreen(
    mut commands: Commands,
    assets: Res<AssetServer>,
    score: Query<&Score, With<Score>>,
) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let continue_text_style = TextStyle {
        font: font.clone(),
        font_size: 35.0,
        color: Color::WHITE,
    };
    let score_text_style = TextStyle {
        font,
        font_size: 55.0,
        color: Color::WHITE,
    };
    let score = score.single().score.floor().to_string();

    commands
        //Clickable background
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(2000.0), Val::Px(2000.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.25, 0.15, 0.15, 0.7).into(),
            ..default()
        })
        .with_children(|parent| {
            //Click to continue text
            parent.spawn(
                TextBundle {
                    text: Text::from_section("Left click to continue", continue_text_style.clone())
                        .with_alignment(TextAlignment::BOTTOM_CENTER),
                    ..default()
                }
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::all(Val::Auto),
                    ..default()
                }),
            );
            //Achieveved score
            parent.spawn((TextBundle {
                text: Text::from_section("Score: ".to_owned() + &score, score_text_style.clone())
                    .with_alignment(TextAlignment::BOTTOM_CENTER),
                ..default()
            }
            .with_style(Style {
                position_type: PositionType::Relative,
                position: UiRect {
                    bottom: Val::Percent(55.),
                    right: Val::Auto,
                    left: Val::Auto,
                    top: Val::Percent(50.),
                },
                ..default()
            }),));
        })
        .insert(ContinueButton)
        .insert(EndScreen);
}

fn click2play(
    mut commands: Commands,
    mut click2play_interaction: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
) {
    for interaction in &mut click2play_interaction {
        match *interaction {
            Interaction::Clicked => {
                let loadtimer = commands
                    .spawn(LoadTimer {
                        timer: Timer::from_seconds(0.1, TimerMode::Once),
                    })
                    .id();
                commands.entity(loadtimer);
            }
            _ => {}
        }
    }
}

fn load_main_menu(
    time: Res<Time>,
    mut query: Query<&mut LoadTimer, With<LoadTimer>>,
    mut state: ResMut<State<GameState>>,
) {
    for mut timer in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            state.set(GameState::MainMenu).expect("Failed to set state");
        }
    }
}

fn despawn_endscreen(mut commands: Commands, query: Query<Entity, With<EndScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn reset_speed(settings: Res<Settings>, mut speed: Query<&mut Speed, With<Speed>>) {
    speed.single_mut().num = settings.startup_speed;
}
