use crate::board::Board;
use crate::{Direction, GameObject, Key};
use alloc::collections::VecDeque;

const INITIAL_SPEED: f32 = 3.0; // cells per second

pub struct Snake {
    body: VecDeque<(usize, usize)>,
    direction: Direction,
    movement_accumulator: f32,
    speed: f32,
    grow_pending: usize,
}

impl Snake {
    pub fn new(x: usize, y: usize) -> Snake {
        Snake {
            body: VecDeque::from([(x, y)]),
            direction: Direction::Right,
            movement_accumulator: 0.0,
            speed: INITIAL_SPEED,
            grow_pending: 0,
        }
    }

    pub fn increase_speed(&mut self, increment: f32) {
        self.speed += increment;
    }

    pub fn change_direction(&mut self, key: Key) {
        let new_direction: Direction = key.into();
        // Prevent the snake from reversing
        if new_direction == Direction::Invalid
            || (self.direction == Direction::Up && new_direction == Direction::Down)
            || (self.direction == Direction::Down && new_direction == Direction::Up)
            || (self.direction == Direction::Left && new_direction == Direction::Right)
            || (self.direction == Direction::Right && new_direction == Direction::Left)
        {
            return;
        }
        self.direction = new_direction;
    }

    pub fn move_forward(&mut self, board: &Board, delta_miliseconds: f32) -> bool {
        let delta_secconds = delta_miliseconds / 1000.0;
        self.movement_accumulator += self.speed * delta_secconds;
        let distance = self.movement_accumulator.floor() as usize;
        if distance == 0 {
            return true;
        }
        self.movement_accumulator -= distance as f32;

        for _ in 0..distance {
            let (head_x, head_y) = self.body[0];
            let (new_head_x, new_head_y) = self.new_head_position(board, head_x, head_y);

            if board.is_wall_at(new_head_x, new_head_y) {
                return false;
            }

            if self.is_snake_at(new_head_x, new_head_y) {
                return false;
            }

            self.body.push_front((new_head_x, new_head_y));
            if self.grow_pending > 0 {
                self.grow_pending -= 1;
                continue;
            }
            self.body.pop_back();
        }

        true
    }

    fn new_head_position(&mut self, board: &Board, head_x: usize, head_y: usize) -> (usize, usize) {
        match self.direction {
            Direction::Up => {
                if head_y == 0 {
                    (head_x, board.get_height() - 1)
                } else {
                    (head_x, head_y - 1)
                }
            }
            Direction::Down => {
                if head_y == board.get_height() - 1 {
                    (head_x, 0)
                } else {
                    (head_x, head_y + 1)
                }
            }
            Direction::Left => {
                if head_x == 0 {
                    (board.get_width() - 1, head_y)
                } else {
                    (head_x - 1, head_y)
                }
            }
            Direction::Right => {
                if head_x == board.get_width() - 1 {
                    (0, head_y)
                } else {
                    (head_x + 1, head_y)
                }
            }
            _ => (head_x, head_y), // Should not happen, but makes the compiler happy
        }
    }

    pub fn grow(&mut self, num_blocks: usize) {
        self.grow_pending += num_blocks;
    }

    pub fn get_head_pos(&self) -> (usize, usize) {
        self.body[0]
    }

    pub fn is_snake_at(&self, x: usize, y: usize) -> bool {
        self.body.iter().any(|&(sx, sy)| sx == x && sy == y)
    }

    pub fn render_to_board(&self, board: &mut Board) {
        for &(x, y) in &self.body {
            board.set_cell(x, y, GameObject::Snake);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_initializes_correctly() {
        let snake = Snake::new(5, 5);
        assert_eq!(snake.body.len(), 1);
        assert_eq!(snake.body[0], (5, 5));
        assert_eq!(snake.direction, Direction::Right);
    }

    #[test]
    fn snake_head_position_is_correct() {
        let snake = Snake::new(3, 4);
        assert_eq!(snake.get_head_pos(), (3, 4));
    }

    #[test]
    fn snake_changes_direction_correctly() {
        let mut snake = Snake::new(5, 5);
        snake.change_direction(Key::ArrowUp);
        assert_eq!(snake.direction, Direction::Up);
        snake.change_direction(Key::ArrowDown);
        assert_eq!(snake.direction, Direction::Up);

        snake.change_direction(Key::ArrowLeft);
        assert_eq!(snake.direction, Direction::Left);
        snake.change_direction(Key::ArrowRight);
        assert_eq!(snake.direction, Direction::Left);

        snake.change_direction(Key::ArrowDown);
        assert_eq!(snake.direction, Direction::Down);
        snake.change_direction(Key::ArrowUp);
        assert_eq!(snake.direction, Direction::Down);

        snake.change_direction(Key::ArrowRight);
        assert_eq!(snake.direction, Direction::Right);
        snake.change_direction(Key::ArrowLeft);
        assert_eq!(snake.direction, Direction::Right);
    }

    #[test]
    fn snake_moves_forward_correctly() {
        let mut board = Board::new(10, 10, 1, 1);
        let mut snake = Snake::new(5, 5);
        snake.speed = 5.0; // 5 blocks per second
        snake.move_forward(&board, 200.0); // 200
        assert_eq!(snake.body[0], (6, 5)); // wraps around to (1, 5)

        snake.change_direction(Key::ArrowDown);
        snake.move_forward(&board, 200.0); // 200 ms
        assert_eq!(snake.body[0], (6, 6));

        snake.change_direction(Key::ArrowLeft);
        snake.move_forward(&board, 400.0); // 400 ms
        assert_eq!(snake.body[0], (4, 6));

        snake.change_direction(Key::ArrowUp);
        snake.move_forward(&board, 1000.0); // 1 second
        assert_eq!(snake.body[0], (4, 1));
    }

    #[test]
    fn snake_head_show_up_on_the_opposite_side_if_leave_board() {
        let mut board = Board::new(10, 10, 1, 1);
        let mut snake = Snake::new(9, 0);
        snake.speed = 5.0; // 5 blocks per second
        snake.move_forward(&board, 200.0);
        assert_eq!(snake.body[0], (0, 0));

        snake.change_direction(Key::ArrowUp);
        snake.move_forward(&board, 200.0);
        assert_eq!(snake.body[0], (0, 9));

        snake.change_direction(Key::ArrowLeft);
        snake.move_forward(&board, 200.0);
        assert_eq!(snake.body[0], (9, 9));

        snake.change_direction(Key::ArrowDown);
        snake.move_forward(&board, 200.0);
        assert_eq!(snake.body[0], (9, 0));
    }

    #[test]
    fn snake_grows_correctly() {
        let mut snake = Snake::new(5, 5);
        snake.speed = 5.0;
        snake.grow(3);

        assert_eq!(snake.grow_pending, 3);

        let mut board = Board::new(10, 10, 1, 1);
        snake.move_forward(&board, 1000.0); // Move 5 blocks
        assert_eq!(snake.body.len(), 4); // Initial + 3 grown
    }

    #[test]
    fn snake_detects_wall_collision() {
        let mut board = Board::new(5, 5, 1, 1);
        let level_data = b"#####
#   #
# # #
#   #
#####";
        board.set_level_data(level_data).unwrap();

        let mut snake = Snake::new(1, 1);
        snake.speed = 5.0; // 5 blocks per second
        assert!(snake.move_forward(&board, 200.0));
        snake.change_direction(Key::ArrowDown);
        assert!(!snake.move_forward(&board, 200.0));
    }

    #[test]
    fn snake_renders_to_board_correctly() {
        let mut board = Board::new(10, 10, 1, 1);
        let mut snake = Snake::new(2, 2);
        snake.speed = 2.0;
        snake.grow(2);
        let mut board = Board::new(10, 10, 1, 1);
        snake.move_forward(&board, 1000.0);
        snake.render_to_board(&mut board);
        assert_eq!(board.get_cell(4, 2), Some(GameObject::Snake));
        assert_eq!(board.get_cell(3, 2), Some(GameObject::Snake));
        assert_eq!(board.get_cell(2, 2), Some(GameObject::Snake));
    }

    #[test]
    fn snake_detects_its_own_body_correctly() {
        let mut snake = Snake::new(5, 5);
        snake.speed = 3.0;
        snake.grow(3);
        let mut board = Board::new(10, 10, 1, 1);
        snake.move_forward(&board, 1000.0);

        assert!(snake.is_snake_at(6, 5));
        assert!(snake.is_snake_at(7, 5));
        assert!(snake.is_snake_at(8, 5));
        assert!(snake.is_snake_at(5, 5));
        assert!(!snake.is_snake_at(4, 5));
    }
}
