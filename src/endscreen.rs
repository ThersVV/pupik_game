use crate::{text::Score, GameState};
use bevy::prelude::*;

/// [Plugin] taking care of the endsceen appearance. This plugin contains
pub struct EndScreenPlugin;

///Labels an [Entity] which is spawned on enter to the [Game::EndScreen] and despawn on exit.
#[derive(Component)]
struct EndScreen;

///Labels [Entity] which is a clickable background that takes one to the main menu.
#[derive(Component)]
struct ContinueButton;

impl Plugin for EndScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EndScreen), spawn_endscreen)
            .add_systems(
                Update,
                continue_interaction.run_if(in_state(GameState::EndScreen)),
            )
            .add_systems(OnExit(GameState::EndScreen), despawn_endscreen);
    }
}

/// Takes care of all collisions with [entities](Entity) that have a unique special effect. It is run on update in the [GameState::Game].
/// # Arguments
/// * `commands` - [Commands].
/// * `assets` - [AssetServer]. Used to load font.
/// * `score` - [Score].
fn spawn_endscreen(mut commands: Commands, assets: Res<AssetServer>, score: Res<Score>) {
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
    let score = score.score.floor().to_string();

    commands
        //Clickable background
        .spawn(ButtonBundle {
            style: Style {
                height: Val::Px(1920.0 / 3. + 100.),
                width: Val::Px(660.0),
                // center button and children
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.25, 0.15, 0.15, 0.7).into(), //Grey
            ..default()
        })
        .with_children(|parent| {
            //"Click to continue" text
            parent.spawn(
                TextBundle {
                    text: Text::from_section("Left click to continue", continue_text_style.clone())
                        .with_alignment(TextAlignment::Center),
                    ..default()
                }
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                }),
            );
            //Achieved score
            parent.spawn((TextBundle {
                text: Text::from_section("Score: ".to_owned() + &score, score_text_style.clone())
                    .with_alignment(TextAlignment::Center),
                ..default()
            }
            .with_style(Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::Center,
                top: Val::Percent(55.),
                ..default()
            }),));
        })
        .insert(ContinueButton)
        .insert(EndScreen);
}

/// Handles [Interaction], specifically [Interaction::Clicked], of the [ContinueButton]. If clicked, game sets the state to [GameState::MainMenu]
/// # Arguments
/// * `continue_button` - [Query] for [Interaction] with [Entity] containing the [ContinueButton].
/// * `state` - Resource containing [State]. This game's states are defined in the [GameState] enum.
fn continue_interaction(
    mut continue_button: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    mut next: ResMut<NextState<GameState>>,
) {
    for interaction in &mut continue_button {
        if *interaction == Interaction::Pressed {
            next.set(GameState::MainMenu);
        }
    }
}

/// Despawns all [entities](Entity) containing [EndScreen].
/// # Arguments
/// * `commands` - [Commands].
/// * `query` - [Query] for [entities](Entity) with the [EndScreen] component.
fn despawn_endscreen(mut commands: Commands, query: Query<Entity, With<EndScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
