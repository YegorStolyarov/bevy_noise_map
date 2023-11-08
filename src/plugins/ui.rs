//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::prelude::*;
use crate::common::AppState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Setup), setup_button)
            .add_systems(Update, button_system.run_if(
                in_state(AppState::Finished)
            ))
            .add_event::<ResetMapEvent>();
    }
}

#[derive(Event)]
pub struct ResetMapEvent;

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut events: EventWriter<ResetMapEvent>,
    mut mouse_buttons: ResMut<Input<MouseButton>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse_buttons.clear_just_pressed(MouseButton::Left);
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                events.send(ResetMapEvent);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>
) {
    commands
    .spawn(NodeBundle {
        style: Style {
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| { parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Reset",
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    });
    next_state.set(AppState::Build);
}