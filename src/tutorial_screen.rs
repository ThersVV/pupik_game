use crate::{player::despawn_player, GameState};
use bevy::prelude::*;
///[Plugin] taking care of functionalities corelating with [Tutorial]
pub struct TutorialPlugin;

///Labels all [entities](Entity) that are spawned only when player enter [GameState::Tutorial]
#[derive(Component)]
struct Tutorial;

///Button in the top left corner that takes player back to the main menu. When clicked, [GameState] changes
/// to [GameState::MainMenu]. When hovered, its background color changes until unhovered.
#[derive(Component)]
pub struct BackButton;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Tutorial),
            (spawn_background, despawn_player, back_button, explain_game),
        )
        .add_systems(
            Update,
            backbutton_system.run_if(in_state(GameState::Tutorial)),
        )
        .add_systems(OnExit(GameState::Tutorial), despawn_tutorial);
    }
}

///Spawns gray transparent background.
/// # Arguments
/// * `commands` - [Commands].
pub fn spawn_background(mut commands: Commands) {
    let sprite = Sprite {
        color: Color::rgba(0.2, 0., 0.1, 0.75), //pink-ish gray
        custom_size: Some(Vec2::new(1920.0 / 3., 700.0)),
        ..Default::default()
    };
    let background = commands
        .spawn(SpriteBundle {
            sprite,
            transform: Transform {
                translation: Vec3::new(0., 0., 990.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tutorial)
        .id();

    commands.entity(background);
}

///Despawns all [Tutorial] [entities](Entity)
/// # Arguments
/// * `commands` - [Commands].
/// * `query` - [Query] for [MainMenu].
fn despawn_tutorial(mut commands: Commands, query: Query<Entity, With<Tutorial>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

///Spawns [BackButton].
/// # Arguments
/// * `commands` - [Commands].
/// * `assets` - [AssetServer]. Used to load font.
pub fn back_button(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Px(65.0),
                // center button and children
                margin: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
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
                "Return to menu",
                TextStyle {
                    font: assets.load("fonts\\Love_Letters.ttf"),
                    font_size: 35.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(BackButton)
        .insert(Tutorial);
}

///Spawns the whole explanation of how the game is played.
/// # Arguments
/// * `commands` - [Commands].
/// * `assets` - [AssetServer]. Used to load font.
fn explain_game(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 26.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section(
                "\n
                You control your unicorn solely using your mouse.

                There are many enemies that can inconvenience
                you...
                    Planes (damage unicorn)
                    Candy (damage unicorn)
                    Rainbows (damage unicorn)
                    Planets (suck the unicorn closer)
                    Black holes (suck in the unicorn)

                Press the left mouse button to hide the unicorn,
                negating any efects.
                Hiding uses energy, shown in the lower left corner.
                Your energy slowly regenerates over time.
                Collecting protein bars refills energy.

                Score increases the further you get.
                There is no end.",
                text_style,
            )
            .with_alignment(TextAlignment::Left),
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(25.0),
            right: Val::Px(50.),
            ..default()
        }),))
        .insert(Tutorial);
}

///Handles interactions with the [BackButton].
/// # Arguments
/// * `interaction_query` - [Query] for [BackButton] and its [Interaction] when changed.
/// * `state` - Resource containing [State]. This game's states are defined in the [GameState] enum.
fn backbutton_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut next: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next.set(GameState::MainMenu);
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
