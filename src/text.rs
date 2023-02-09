use crate::{
    player::{Hidden, Player},
    GameState, Settings, Speed,
};
use bevy::prelude::*;
pub struct TextPlugin;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
pub struct Score {
    pub score: f32,
}

#[derive(Component)]
struct EnergyText;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
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
        .add_system_set(SystemSet::on_exit(GameState::EndScreen).with_system(despawn_score));
    }
}

fn score_counter(mut commands: Commands, assets: Res<AssetServer>, settings: Res<Settings>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 50.0,
        color: Color::WHITE,
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section("0", text_style.clone())
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
    commands.spawn(Score {
        score: settings.startup_score,
    });
}

fn energy_counter(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let desc_text_style = TextStyle {
        font: font.clone(),
        font_size: 25.,
        color: Color::WHITE,
    };
    commands.spawn((TextBundle {
        text: Text::from_section("ENERGY", desc_text_style.clone())
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
        color: Color::WHITE,
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section("", energy_text_style.clone())
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

fn score_update(
    mut query: Query<&mut Text, With<ScoreText>>,
    mut score: Query<&mut Score, With<Score>>,
    speed: Query<&Speed, With<Speed>>,
    time: Res<Time>,
) {
    for mut text in &mut query {
        let mut score = score.single_mut();
        score.score += time.delta_seconds() * 50. * speed.single().num;
        text.sections[0].value = format!("{}", (score.score) as usize);
    }
}

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

fn despawn_text(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
fn despawn_score(mut commands: Commands, query: Query<Entity, With<Score>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
