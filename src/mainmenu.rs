use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct TutorialButton;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct TutorialText;

#[derive(Component)]
struct MainMenuBckg;

#[derive(Component)]
struct Flickering {
    timer: Timer,
}

#[derive(Component)]
struct LoadTimer {
    timer: Timer,
}

#[derive(Component)]
struct MainMenuCredits;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(spawn_credits)
                .with_system(spawn_start_text),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(change_flick)
                .with_system(click2play)
                .with_system(load_game)
                .with_system(tutorial_button),
        )
        .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(despawn_menu));
    }
}

fn spawn_credits(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::WHITE,
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section("Music by Vojtech Klhufek", text_style.clone())
                .with_alignment(TextAlignment::BOTTOM_CENTER),
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),))
        .insert(MainMenu)
        .insert(MainMenuCredits);
}

fn spawn_start_text(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 55.0,
        color: Color::WHITE,
    };
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(2000.0), Val::Px(2000.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(1.0, 0.8, 0.9, 0.7).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle {
                        text: Text::from_section("Left click to start", text_style.clone())
                            .with_alignment(TextAlignment::BOTTOM_CENTER),
                        ..default()
                    }
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect::all(Val::Auto),
                        ..default()
                    }),
                )
                .insert(Flickering {
                    timer: Timer::from_seconds(0.6, TimerMode::Repeating),
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        // center button
                        margin: UiRect {
                            bottom: Val::Percent(42.),
                            right: Val::Auto,
                            left: Val::Auto,
                            top: Val::Percent(88.),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0., 0., 0., 0.1).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            "How to play",
                            TextStyle {
                                font: assets.load("fonts\\Love_Letters.ttf"),
                                font_size: 35.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .insert(TutorialText);
                })
                .insert(TutorialButton);
        })
        .insert(PlayButton)
        .insert(MainMenu);
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
fn click2play(
    mut commands: Commands,
    mut click2play_interaction: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for interaction in &mut click2play_interaction {
        match *interaction {
            Interaction::Clicked => {
                let loadtimer = commands
                    .spawn(LoadTimer {
                        timer: Timer::from_seconds(0.05, TimerMode::Once),
                    })
                    .id();
                commands.entity(loadtimer);
            }
            _ => {}
        }
    }
}

fn tutorial_button(
    mut commands: Commands,
    loadtimer: Query<Entity, With<LoadTimer>>,
    mut tutorial_interaction: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<TutorialButton>),
    >,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut color) in &mut tutorial_interaction {
        match *interaction {
            Interaction::Clicked => {
                for loadtimer in loadtimer.iter() {
                    commands.entity(loadtimer).despawn();
                }
                state.set(GameState::Tutorial).expect("Failed to set state");
            }
            Interaction::Hovered => {
                *color = Color::rgba(0., 0., 0., 0.7).into();
            }
            Interaction::None => {
                *color = Color::rgba(0., 0., 0., 0.1).into();
            }
        }
    }
}
fn change_flick(
    time: Res<Time>,
    mut query: Query<(&mut Flickering, &mut Visibility), With<Flickering>>,
) {
    for (mut flickering, mut visibility) in &mut query {
        flickering.timer.tick(time.delta());
        if flickering.timer.just_finished() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

fn load_game(
    time: Res<Time>,
    mut query: Query<&mut LoadTimer, With<LoadTimer>>,
    mut state: ResMut<State<GameState>>,
) {
    for mut timer in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            state.set(GameState::Game).expect("Failed to set state");
        }
    }
}
