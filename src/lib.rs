pub use game::GameWasm;

mod board;
mod game;
mod snake;
mod food;
mod level;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
/// Represents keys that can be pressed by the user.
pub enum Key {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Space,
    Invalid,
}


impl From<&str> for Key {
    fn from(value: &str) -> Self {
        match value {
            "ArrowUp" => Key::ArrowUp,
            "ArrowDown" => Key::ArrowDown,
            "ArrowLeft" => Key::ArrowLeft,
            "ArrowRight" => Key::ArrowRight,
            "Space" => Key::Space,
            _ => Key::Invalid,
        }
    }
}

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
/// Represents movement directions for the snake.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

impl From<Key> for Direction {
    fn from(value: Key) -> Self {
        match value {
            Key::ArrowUp => Direction::Up,
            Key::ArrowDown => Direction::Down,
            Key::ArrowLeft => Direction::Left,
            Key::ArrowRight => Direction::Right,
            _ => Direction::Invalid,
        }
    }
}

#[wasm_bindgen]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<GameObject> for Color {
    fn from(value: GameObject) -> Self {
        match value {
            GameObject::Snake => Color {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            },
            GameObject::Food => Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
            GameObject::Wall => Color { // brown wall
                r: 139,
                g: 69,
                b: 19,
                a: 255,
            },
            GameObject::Empty => Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// Represents different objects that can exist on the game board.
pub enum GameObject {
    Snake,
    Food,
    Wall,
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
#[wasm_bindgen]
/// Represents the current state of the game.
pub enum GameState {
    Running,
    Paused,
    GameOver,
}

#[wasm_bindgen]
/// Represents different events that can occur in the game.
pub enum GameEvent {
    EatFood,
    GameOver,
    GameStart,
    GamePause,
}