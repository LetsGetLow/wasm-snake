use crate::board::Board;
use crate::{Direction, GameObject, Key};
use alloc::collections::VecDeque;

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
            speed: 6.0,
            grow_pending: 0,
        }
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

    pub fn move_forward(&mut self, board: &Board, delta_time: f32) -> bool {
        if self.direction == Direction::Invalid || self.direction == Direction::Freeze {
            return true;
        }
        let (head_x, head_y) = self.body[0];

        let delta_secconds = delta_time / 1000.0;
        self.movement_accumulator += self.speed * delta_secconds;
        let distance = self.movement_accumulator.floor() as usize;
        if distance == 0 {
            return true;
        }
        self.movement_accumulator -= distance as f32;

        for _ in 0..distance {
            let new_head = match self.direction {
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
                _ => (head_x, head_y), // will never happen
            };

            if board.wall_collides(new_head.0, new_head.1) {
                return false
            }

            self.body.push_front(new_head);
            if self.grow_pending > 0 {
                self.grow_pending -= 1;
                continue;
            }
            self.body.pop_back();
        }

        return true;
    }

    pub fn grow(&mut self, num_blocks: usize) {
        self.grow_pending += num_blocks;
    }

    pub fn render(&self, board: &mut Board) {
        for &(x, y) in &self.body {
            board.set_cell(x, y, GameObject::Snake);
        }
    }
}
