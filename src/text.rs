use crate::player::{Hidden, Player};
use bevy::prelude::*;
pub struct TextPlugin;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct EnergyText;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(score_counter)
            .add_startup_system(energy_counter)
            .add_system(score_update)
            .add_system(energy_update);
    }
}

fn score_counter(mut commands: Commands, assets: Res<AssetServer>) {
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

fn score_update(mut query: Query<&mut Text, With<ScoreText>>, time: Res<Time>) {
    for mut text in &mut query {
        text.sections[0].value = format!("{}", (time.elapsed_seconds() * 50.) as usize);
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
