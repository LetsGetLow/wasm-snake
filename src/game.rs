use crate::board::Board;
use crate::food::FoodManager;
use crate::snake::Snake;
use crate::{GameEvent, GameState, Key};
use alloc::vec;
use alloc::vec::Vec;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;

const INITIAL_SNAKE_LENGTH: usize = 5;
const SNAKE_GROWTH_RATE: usize = 8;

const SPEED_INC: f32 = 0.05;

#[wasm_bindgen]
pub struct GameWasm {
    score: u32,
    screen_buffer: Vec<u8>,
    game_state: GameState,
    snake: Snake,
    board: Board,
    food_manager: FoodManager,
    game_event_listener: Option<js_sys::Function>,
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
        snake.grow(INITIAL_SNAKE_LENGTH - 1);

        let mut food_manager = FoodManager::new();
        food_manager.spawn_food(&board, &snake);

        GameWasm {
            score: 0,
            screen_buffer: vec![0; size],
            game_state: GameState::Paused,
            snake,
            board,
            food_manager,
            game_event_listener: None,
        }
    }

    fn reset(&mut self) {
        self.score = 0;
        self.game_state = GameState::Running;
        self.snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
        self.snake.grow(SNAKE_GROWTH_RATE);
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
            self.trigger_event(GameEvent::GameOver);
            self.game_state = GameState::GameOver;
            return;
        }

        let (head_x, head_y) = self.snake.get_head_pos();
        if self.food_manager.is_food_at(head_x, head_y) {
            self.snake_eats_food(head_x, head_y);
        }
    }

    fn snake_eats_food(&mut self, x: usize, y: usize) {
        self.trigger_event(GameEvent::EatFood);
        self.snake.grow(2);
        self.snake.increase_speed(SPEED_INC);
        self.food_manager.take_food(x, y);
        self.update_score(10);
        self.food_manager.spawn_food(&self.board, &self.snake);
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        self.board.draw_level();
        self.food_manager.render_foods_to_board(&mut self.board);
        self.snake.render_to_board(&mut self.board);
        self.board
            .render_to_buffer(self.screen_buffer.as_mut_slice());
    }

    fn trigger_event(&self, event: GameEvent) {
        if let Some(callback) = &self.game_event_listener {
            let this = JsValue::NULL;
            let event = JsValue::from(event);
            let _ = callback.call1(&this, &event);
        }
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, key: &str) {
        let key = key.into();
        if (key == Key::Space) && self.game_state == GameState::Running {
            self.trigger_event(GameEvent::GamePause);
            self.game_state = GameState::Paused;
            return;
        } else if (key == Key::Space) && self.game_state == GameState::Paused {
            self.trigger_event(GameEvent::GameStart);
            self.game_state = GameState::Running;
            return;
        } else if (key == Key::Space) && self.game_state == GameState::GameOver {
            self.trigger_event(GameEvent::GameStart);
            self.reset();
            return;
        }

        if self.game_state != GameState::Running {
            return;
        }
        self.snake.change_direction(key);
    }

    #[wasm_bindgen]
    pub fn add_game_event_listener(&mut self, callback: js_sys::Function) {
        self.game_event_listener = Some(callback);
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
