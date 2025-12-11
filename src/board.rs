use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::error::Error;
use crate::{Color, GameObject};

type Result<T> = core::result::Result<T, Box<dyn Error>>;

#[derive(PartialEq)]
pub struct Board {
    width: usize,
    height: usize,
    cell_width: usize,
    cell_height: usize,
    cells: Vec<GameObject>,
    level_data: Vec<u8>,
}

impl Board {
    pub fn new(width: usize, height: usize, cell_width: usize, cell_height: usize) -> Self {
        let size = width * height;
        let cells = vec![GameObject::Empty; size];

        Board {
            width,
            height,
            cell_width,
            cell_height,
            cells,
            level_data: vec![b' '; size],
        }
    }

    pub fn set_level_data(&mut self, data: &[u8]) -> Result<()> {
        let clean_data = data.iter().fold(Vec::new(), |mut acc, &b| {
            if b != b'\n' && b != b'\r' {
                acc.push(b);
            }
            acc
        });

        if clean_data.len() != self.width * self.height {
            Err("Level data size does not match board dimensions".into())
        } else {
            self.level_data = clean_data;
            Ok(())
        }
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: GameObject) {
        if x < self.width && y < self.height {
            let idx = self.xy_to_index(x, y);
            self.cells[idx] = value;
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<GameObject> {
        if x < self.width && y < self.height {
            let idx = self.xy_to_index(x, y);
            Some(self.cells[idx])
        } else {
            None
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn clear(&mut self) {
        self.cells.fill(GameObject::Empty);
    }

    pub fn draw_level(&mut self) {
        self.level_data.iter().enumerate().for_each(|(idx, char_byte)| {
            let game_object = if *char_byte == b'#' {
                GameObject::Wall
            } else {
                GameObject::Empty
            };
            self.cells[idx] = game_object;
        });
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        let idx = self.xy_to_index(x, y);
        self.level_data[idx] == b'#'
    }

    pub fn render_to_buffer(&mut self, buffer: &mut [u8]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.xy_to_index(x, y);
                let color = Color::from(self.cells[idx]);

                for cy in 0..self.cell_height {
                    for cx in 0..self.cell_width {
                        let buffer_x = x * self.cell_width + cx;
                        let buffer_y = y * self.cell_height + cy;
                        let index = (buffer_y * self.width * self.cell_width + buffer_x) * 4;
                        buffer[index] = color.r;
                        buffer[index + 1] = color.g;
                        buffer[index + 2] = color.b;
                        buffer[index + 3] = color.a;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_initializes_correctly() {
        let board = Board::new(10, 10, 5, 5);
        assert_eq!(board.get_width(), 10);
        assert_eq!(board.get_height(), 10);
        for y in 0..10 {
            for x in 0..10 {
                assert_eq!(board.get_cell(x, y), Some(GameObject::Empty));
            }
        }
    }

    #[test]
    fn board_can_manage_cells() {
        let mut board = Board::new(5, 5, 2, 2);
        board.set_cell(2, 2, GameObject::Wall);
        assert_eq!(board.get_cell(2, 2), Some(GameObject::Wall));
    }

    #[test]
    fn board_handles_out_of_bounds_cells() {
        let mut board = Board::new(5, 5, 2, 2);
        board.set_cell(10, 10, GameObject::Wall); // out of bounds
        assert_eq!(board.get_cell(10, 10), None);
    }

    #[test]
    fn board_clears_correctly() {
        let mut board = Board::new(5, 5, 2, 2);
        board.set_cell(1, 1, GameObject::Wall);
        board.clear();
        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(board.get_cell(x, y), Some(GameObject::Empty));
            }
        }
    }

    #[test]
    fn board_fails_on_invalid_level_data() {
        let mut board = Board::new(5, 5, 2, 2);
        let invalid_level_data = b"#####
#   #
# # #"; // too short
        let result = board.set_level_data(invalid_level_data);
        assert!(result.is_err());
    }

    #[test]
    fn board_can_detect_wall_collision() {
        let mut board = Board::new(5, 5, 2, 2);
        let level_data = b"#####
#   #
# # #
#   #
#####";
        board.set_level_data(level_data).unwrap();
        assert!(board.is_wall(0, 0));
        assert!(!board.is_wall(1, 1));
        assert!(board.is_wall(2, 2));
    }

    #[test]
    fn board_draws_level_correctly() {
        let mut board = Board::new(10, 10, 2, 2);
        let level_data = b"##########
#        #
#  ##    #
#        #
#        #
#    ##  #
#        #
#        #
#        #
##########";

        board.set_level_data(level_data).unwrap();
        board.draw_level();

        for y in 0..10 {
            for x in 0..10 {
                if board.is_wall(x, y) {
                    assert_eq!(board.get_cell(x, y), Some(GameObject::Wall));
                } else {
                    assert_eq!(board.get_cell(x, y), Some(GameObject::Empty));
                }
            }
        }
    }

    #[test]
    fn board_renders_to_buffer_correctly() {
        const CELL_WIDTH: usize = 2;
        const CELL_HEIGHT: usize = 2;
        let mut board = Board::new(2, 2, CELL_WIDTH, CELL_HEIGHT);
        board.set_cell(0, 0, GameObject::Snake);
        board.set_cell(1, 0, GameObject::Food);
        board.set_cell(0, 1, GameObject::Wall);
        board.set_cell(1, 1, GameObject::Empty);

        let mut buffer = vec![0; 2 * 2 * CELL_WIDTH * CELL_HEIGHT * 4]; // width * height * cell_width * cell_height * 4 (RGBA)
        board.render_to_buffer(&mut buffer);

        // Check colors in the buffer
        let snake_color = Color::from(GameObject::Snake);
        let food_color = Color::from(GameObject::Food);
        let wall_color = Color::from(GameObject::Wall);
        let empty_color = Color::from(GameObject::Empty);

        // Top-left cell (Snake)
        for y in 0..2 {
            for x in 0..2 {
                let index = (y * 4 + x) * 4;
                assert_eq!(buffer[index], snake_color.r);
                assert_eq!(buffer[index + 1], snake_color.g);
                assert_eq!(buffer[index + 2], snake_color.b);
                assert_eq!(buffer[index + 3], snake_color.a);
            }
        }

        // Top-right cell (Food)
        for y in 0..2 {
            for x in 2..4 {
                let index = (y * 4 + x) * 4;
                assert_eq!(buffer[index], food_color.r);
                assert_eq!(buffer[index + 1], food_color.g);
                assert_eq!(buffer[index + 2], food_color.b);
                assert_eq!(buffer[index + 3], food_color.a);
            }
        }

        // Bottom-left cell (Wall)
        for y in 2..4 {
            for x in 0..2 {
                let index = (y * 4 + x) * 4;
                assert_eq!(buffer[index], wall_color.r);
                assert_eq!(buffer[index + 1], wall_color.g);
                assert_eq!(buffer[index + 2], wall_color.b);
                assert_eq!(buffer[index + 3], wall_color.a);
            }
        }

        // Bottom-right cell (Empty)
        for y in 2..4 {
            for x in 2..4 {
                let index = (y * 4 + x) * 4;
                assert_eq!(buffer[index], empty_color.r);
                assert_eq!(buffer[index + 1], empty_color.g);
                assert_eq!(buffer[index + 2], empty_color.b);
                assert_eq!(buffer[index + 3], empty_color.a);
            }
        }
    }
}
