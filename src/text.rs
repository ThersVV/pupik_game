use crate::{
    player::{Hidden, Player},
    speed::Speed,
    GameState, Settings,
};
use bevy::prelude::*;

///[Plugin] taking care of functionalities corelating with ingame [Text].
pub struct TextPlugin;

///[Text] in the bottom right corner showing current [Score]
#[derive(Component)]
struct ScoreText;

///Current achieved score, increments with time.
/// # Fields
/// `score` - current score.
#[derive(Resource)]
pub struct Score {
    pub score: f32,
}

///Labels text in the bottom left corner showing current [Hidden].energy level.
#[derive(Component)]
struct EnergyText;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_score)
            .add_systems(OnEnter(GameState::Game), (score_counter, energy_counter))
            .add_systems(
                Update,
                (score_update, energy_update).run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_text)
            .add_systems(OnExit(GameState::EndScreen), reset_score);
    }
}

///Spawns [Score].
/// # Arguments
/// * `commands` - [Commands]
/// * `settings` - [Settings], used to access the `startup_score` field.
fn spawn_score(mut commands: Commands, settings: Res<Settings>) {
    commands.insert_resource(Score {
        score: settings.startup_score,
    });
}

///Spawns [ScoreText].
/// # Arguments
/// * `commands` - [Commands]
/// * `assets` - [AssetServer]. Used to load font.
fn score_counter(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 50.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section(0.to_string(), text_style)
                .with_alignment(TextAlignment::Center),
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),))
        .insert(ScoreText);
}

///Spawns [EnergyText].
/// # Arguments
/// * `commands` - [Commands]
/// * `assets` - [AssetServer]. Used to load font.
fn energy_counter(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let desc_text_style = TextStyle {
        font: font.clone(),
        font_size: 25.,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands.spawn((TextBundle {
        text: Text::from_section("ENERGY", desc_text_style).with_alignment(TextAlignment::Center),
        ..default()
    }
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(5.0),
        left: Val::Px(15.0),
        ..default()
    }),));

    let energy_text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section("", energy_text_style).with_alignment(TextAlignment::Center),
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(15.0),
            ..default()
        }),))
        .insert(EnergyText);
}

///Increments score with time and updates [ScoreText] appropriately. Incrementing scales with [Speed].
/// # Arguments
/// * `query` - [Query] for [ScoreText].
/// * `score` - [Score].
/// * `speed` - [Speed].
/// * `time` - [Time].
fn score_update(
    mut query: Query<&mut Text, With<ScoreText>>,
    mut score: ResMut<Score>,
    speed: Res<Speed>,
    time: Res<Time>,
) {
    for mut text in &mut query {
        score.score += time.delta_seconds() * 50. * speed.speed;
        text.sections[0].value = format!("{}", score.score as usize);
    }
}

///Updates [EnergyText] appropriately.
/// # Arguments
/// * `query` - [Query] for [EnergyText].
/// * `player` - [Query] for [Player].
fn energy_update(
    mut query: Query<&mut Text, With<EnergyText>>,
    player: Query<&Hidden, With<Player>>,
) {
    for mut text in &mut query {
        for players in &player {
            text.sections[0].value = format!("{}", players.energy as usize);
        }
    }
}

///Despawns [Text] on exit from [GameState::Game].
/// # Arguments
/// * `commands` - [Commands].
/// * `player_query` - [Query] for [Text].
fn despawn_text(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

///Spawns [Score].
/// # Arguments
/// * `settings` - [Settings], used to access the `startup_score` field.
/// * `score` - [Score].
fn reset_score(settings: Res<Settings>, mut score: ResMut<Score>) {
    score.score = settings.startup_score;
}
