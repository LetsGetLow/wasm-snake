use alloc::vec;
use alloc::vec::Vec;
use crate::board::Board;
use crate::snake::Snake;
use crate::{GameObject, GameState, Key};
use wasm_bindgen::prelude::wasm_bindgen;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;

const INITIAL_SNAKE_LENGTH: usize = 5;

#[wasm_bindgen]
pub struct GameWasm {
    score: u32,
    screen_buffer: Vec<u8>,
    game_state: GameState,
    snake: Snake,
    board: Board,
}

#[wasm_bindgen]
impl GameWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> GameWasm {
        let size = width * height * 4;
        let cell_width = width / GRID_WIDTH;
        let cell_height = height / GRID_HEIGHT;

        let level_data = include_bytes!("../assets/levels/level01.txt");
        let mut board = Board::new(GRID_WIDTH, GRID_HEIGHT, cell_width, cell_height);
        board.set_level_data(level_data).unwrap();

        let mut snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
        snake.grow(INITIAL_SNAKE_LENGTH);

        GameWasm {
            score: 0,
            screen_buffer: vec![0; size],
            game_state: GameState::Paused,
            snake,
            board,
        }
    }

    fn reset(&mut self) {
        self.score = 0;
        self.game_state = GameState::Running;
        self.snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
        self.snake.grow(INITIAL_SNAKE_LENGTH);
    }

    #[wasm_bindgen]
    pub fn get_screen_buffer(&self) -> *const u8 {
        self.screen_buffer.as_ptr()
    }

    fn update_score(&mut self, points: u32) {
        self.score += points;
    }

    #[wasm_bindgen]
    pub fn get_score(&self) -> u32 {
        self.score
    }

    #[wasm_bindgen]
    pub fn update(&mut self, delta_time: f32) {
        if self.game_state != GameState::Running {
            return;
        }

        if !self.snake.move_forward(&self.board, delta_time) {
            self.game_state = GameState::GameOver;
            return;
        }
        self.board.set_cell(0, 0, GameObject::Food);
        self.board.set_cell(
            self.board.get_width(),
            self.board.get_height(),
            GameObject::Wall,
        );
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        self.board.draw_level();
        self.snake.render(&mut self.board);
        self.board
            .render_to_buffer(self.screen_buffer.as_mut_slice());
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, key: &str) {
        let key = key.into();
        if (key == Key::Space) && self.game_state == GameState::Running {
            self.game_state = GameState::Paused;
            return;
        } else if (key == Key::Space) && self.game_state == GameState::Paused {
            self.game_state = GameState::Running;
            return;
        } else if (key == Key::Space) && self.game_state == GameState::GameOver {
            self.reset();
            return;
        }

        if self.game_state != GameState::Running {
            return;
        }
        self.snake.change_direction(key);
    }

    #[wasm_bindgen]
    pub fn get_board_width(&self) -> usize {
        self.board.get_width()
    }

    #[wasm_bindgen]
    pub fn get_board_height(&self) -> usize {
        self.board.get_height()
    }

    #[wasm_bindgen]
    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }
}
