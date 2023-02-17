use crate::GameState;
use bevy::prelude::*;

///[Plugin] taking care of functionalities corelating with [MainMenu]
pub struct MenuPlugin;

///Labels all [entities](Entity) which are spawned on enter to the [Game::MainMenu] and despawn on exit.
#[derive(Component)]
struct MainMenu;

///The "How to play" button. When clicked, [GameState] changes to [GameState::Tutorial].
/// When hovered, its background color changes until unhovered.
#[derive(Component)]
struct TutorialButton;

///A clickable "Left click to play" background. When clicked, spawns [LoadTimer].
#[derive(Component)]
struct PlayButton;

///[Timer] indicating that [Entity] oscilates between visible and invisible each 0.3s
#[derive(Component)]
struct Flickering {
    timer: Timer,
}

///[Timer] that is activated once player clicks the [PlayButton]. When timer runs out,
/// [GameState] is changed to [GameState::Game]. It is handled this way so the game
///  can react if player clicked on [TutorialButton] too (buttons overlap).
#[derive(Component)]
struct LoadTimer {
    timer: Timer,
}

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
                .with_system(tutorial_button_interaction),
        )
        .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(despawn_menu));
    }
}

///Spawns the "Music by Vojtech Klhufek" text.
/// # Arguments
/// * `commands` - [Commands].
/// * `assets` - [AssetServer]. Used to load font.
fn spawn_credits(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section("Music by Vojtech Klhufek", text_style)
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
        .insert(MainMenu);
}

///Spawns clickable background button together with the "How to play" button as its child
/// # Arguments
/// * `commands` - [Commands].
/// * `assets` - [AssetServer]. Used to load font.
fn spawn_start_text(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 55.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(2000.0), Val::Px(2000.0)),
                // center button and children
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(1.0, 0.8, 0.9, 0.7).into(), //pink
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle {
                        text: Text::from_section("Left click to start", text_style)
                            .with_alignment(TextAlignment::BOTTOM_CENTER),
                        ..default()
                    }
                    .with_style(Style {
                        position_type: PositionType::Absolute,
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
                            top: Val::Percent(88.),
                            ..default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0., 0., 0., 0.1).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "How to play",
                        TextStyle {
                            font: assets.load("fonts\\Love_Letters.ttf"),
                            font_size: 35.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(TutorialButton);
        })
        .insert(PlayButton)
        .insert(MainMenu);
}

///Despawns all [MainMenu] [entities](Entity)
/// # Arguments
/// * `commands` - [Commands].
/// * `query` - [Query] for [MainMenu].
fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

///Handles interactions with the [PlayButton].
/// # Arguments
/// * `commands` - [Commands].
/// * `click2play_interaction` - [Query] for [PlayButton] and its [Interaction] when changed.
fn click2play(
    mut commands: Commands,
    mut click2play_interaction: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for interaction in &mut click2play_interaction {
        if *interaction == Interaction::Clicked {
            let loadtimer = commands
                .spawn(LoadTimer {
                    timer: Timer::from_seconds(0.05, TimerMode::Once),
                }) //Done via timer so game can react if player clicked on "How to play" too (buttons overlap)
                .id();
            commands.entity(loadtimer);
        }
    }
}

///Handles interactions with the [TutorialButton].
/// # Arguments
/// * `commands` - [Commands].
/// * `loadtimer` - [Query] for [LoadTimer].
/// * `tutorial_interaction` - [Query] for [TutorialButton] and its [Interaction] when changed.
/// * `state` - Resource containing [State]. This game's states are defined in the [GameState] enum.
fn tutorial_button_interaction(
    mut commands: Commands,
    loadtimer: Query<Entity, With<LoadTimer>>,
    mut tutorial_interaction: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<TutorialButton>),
    >,
    mut state: ResMut<State<GameState>>,
) {
    //Reacts to interactions with the "How to play" button
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

///Handles the fuctionality of [Flickering].
/// # Arguments
/// * `time` - [Time].
/// * `query` - [Query] for [Flickering].
fn change_flick(
    time: Res<Time>,
    mut query: Query<(&mut Flickering, &mut Visibility), With<Flickering>>,
) {
    //Makes entities containing Flickering component flicker
    for (mut flickering, mut visibility) in &mut query {
        flickering.timer.tick(time.delta());
        if flickering.timer.just_finished() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

///Loads game once [LoadTimer] finishes.
/// # Arguments
/// * `time` - [Time].
/// * `query` - [Query] for [LoadTimer].
/// * `state` - Resource containing [State]. This game's states are defined in the [GameState] enum.
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
