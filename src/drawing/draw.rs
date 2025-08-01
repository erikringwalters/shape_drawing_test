use crate::cursor::Cursor;
use crate::drawing::dot::*;
use crate::drawing::line::*;

use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use super::arc::{ArcPlugin, handle_draw_arc};
use super::circle::{CirclePlugin, handle_draw_circle};
use super::rectangle::{RectanglePlugin, handle_draw_rectangle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default, Reflect)]
pub enum DrawMode {
    #[default]
    None,
    Dot,
    Line,
    Rectangle,
    Circle,
    Arc,
}

pub const DEFAULT_RESOLUTION: u32 = 64;
pub const DEFAULT_POS: Vec3 = Vec3::splat(f32::MIN);

#[derive(Resource, Debug, PartialEq)]
pub struct CurrentDrawing {
    pub position: [Vec3; 3],
}

impl Default for CurrentDrawing {
    fn default() -> Self {
        CurrentDrawing {
            position: [DEFAULT_POS, DEFAULT_POS, DEFAULT_POS],
        }
    }
}

#[derive(Resource, Default, Debug, PartialEq, PartialOrd)]
pub struct LineChain {
    pub count: u32,
}

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DrawMode>()
            .insert_resource(CurrentDrawing::default())
            .insert_resource(LineChain::default())
            .add_plugins(DotPlugin)
            .add_plugins(LinePlugin)
            .add_plugins(RectanglePlugin)
            .add_plugins(CirclePlugin)
            .add_plugins(ArcPlugin)
            .add_systems(Update, (change_draw_mode, handle_drawing).chain());
    }
}

#[hot]
fn change_draw_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<DrawMode>>,
    current_drawing: ResMut<CurrentDrawing>,
    line_chain: ResMut<LineChain>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        reset_drawing(current_drawing, line_chain);
        state.set(DrawMode::None);
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        reset_drawing(current_drawing, line_chain);
        state.set(DrawMode::Dot);
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        reset_drawing(current_drawing, line_chain);
        state.set(DrawMode::Line);
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        reset_drawing(current_drawing, line_chain);
        state.set(DrawMode::Rectangle);
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        reset_drawing(current_drawing, line_chain);
        state.set(DrawMode::Circle);
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        reset_drawing(current_drawing, line_chain);
        state.set(DrawMode::Arc);
    }
}

#[hot]
fn handle_drawing(
    commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    state: Res<State<DrawMode>>,
    cursor: Res<Cursor>,
    current_drawing: ResMut<CurrentDrawing>,
    line_chain: ResMut<LineChain>,
) {
    match state.get() {
        DrawMode::Dot => {
            handle_draw_dot(commands, mouse_input, cursor);
        }
        DrawMode::Line => {
            handle_draw_line(commands, mouse_input, cursor, current_drawing, line_chain);
        }
        DrawMode::Rectangle => {
            handle_draw_rectangle(commands, mouse_input, cursor, current_drawing);
        }
        DrawMode::Circle => {
            handle_draw_circle(commands, mouse_input, cursor, current_drawing);
        }
        DrawMode::Arc => handle_draw_arc(commands, mouse_input, cursor, current_drawing),
        _ => {
            return;
        }
    }
}

#[hot]
pub fn reset_drawing(current_drawing: ResMut<CurrentDrawing>, mut line_chain: ResMut<LineChain>) {
    reset_current_drawing(current_drawing);
    line_chain.count = 0
}

#[hot]
pub fn reset_current_drawing(mut current_drawing: ResMut<CurrentDrawing>) {
    *current_drawing = CurrentDrawing::default();
}
