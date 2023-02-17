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
        app.add_startup_system(score_counter)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(score_counter)
                    .with_system(energy_counter),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(score_update)
                    .with_system(energy_update),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn_text))
            .add_system_set(SystemSet::on_exit(GameState::EndScreen).with_system(reset_score));
    }
}

///Spawns [ScoreText] and [Score].
/// # Arguments
/// * `commands` - [Commands]
/// * `assets` - [AssetServer]. Used to load font.
/// * `settings` - [Settings]. Used to access `startup_score` field.
fn score_counter(mut commands: Commands, assets: Res<AssetServer>, settings: Res<Settings>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 50.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section(settings.startup_score.to_string(), text_style)
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
        .insert(ScoreText);
    commands.insert_resource(Score {
        score: settings.startup_score,
    });
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
        text: Text::from_section("ENERGY", desc_text_style)
            .with_alignment(TextAlignment::BOTTOM_CENTER),
        ..default()
    }
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        ..default()
    }),));

    let energy_text_style = TextStyle {
        font,
        font_size: 35.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section("", energy_text_style)
                .with_alignment(TextAlignment::BOTTOM_CENTER),
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(30.0),
                left: Val::Px(15.0),
                ..default()
            },
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
