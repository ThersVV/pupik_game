use crate::player::despawn_player;
use crate::GameState;
use bevy::prelude::*;
pub struct TutorialPlugin;

#[derive(Component)]
struct Tutorial;

#[derive(Component)]
struct TutorialBckg;

#[derive(Component)]
pub struct BackButton;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Tutorial)
                .with_system(spawn_background)
                .with_system(despawn_player)
                .with_system(back_button)
                .with_system(spawn_credits),
        )
        .add_system_set(SystemSet::on_update(GameState::Tutorial).with_system(button_system))
        .add_system_set(SystemSet::on_exit(GameState::Tutorial).with_system(despawn_tutorial));
    }
}

pub fn spawn_background(mut commands: Commands) {
    let sprite = Sprite {
        color: Color::rgba(0.2, 0., 0.1, 0.75),
        custom_size: Some(Vec2::new(2000.0, 2000.0)),
        ..Default::default()
    };
    let background = commands
        .spawn(SpriteBundle {
            sprite: sprite,
            transform: Transform {
                translation: Vec3::new(100., 100., 990.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tutorial)
        .insert(TutorialBckg)
        .id();

    commands.entity(background);
}

fn despawn_tutorial(mut commands: Commands, query: Query<Entity, With<Tutorial>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
pub fn back_button(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                // center button
                margin: UiRect {
                    bottom: Val::Auto,
                    right: Val::Auto,
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
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

fn spawn_credits(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts\\Love_Letters.ttf");
    let text_style = TextStyle {
        font,
        font_size: 28.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    commands
        .spawn((TextBundle {
            text: Text::from_section(
                "\n
                You control your unicorn solely using your mouse.

                There are many enemies that can inconvenience
                you, including...
                    Planes (damages unicorn)
                    Candy (damages unicorn)
                    Rainbows (damages unicorn)
                    Planets (sucks the unicorn closer)
                    Black holes (sucks in the unicorn)

                Press the left mouse button to hide the unicorn,
                negating any efects.
                Hiding uses energy, shown in the lower left corner.
                Your energy slowly regenerates over time.
                Collecting protein bars refills energy.

                Score increases the further you get.
                There is no end.",
                text_style.clone(),
            )
            .with_alignment(TextAlignment::TOP_LEFT),
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Percent(20.0),
                left: Val::Px(-50.),
                ..default()
            },
            ..default()
        }),))
        .insert(Tutorial);
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state
                    .set(GameState::MainMenu)
                    .expect("Unexpected state set error.");
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
