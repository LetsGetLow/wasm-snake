use crate::board::Board;
use crate::GameObject;
use crate::snake::Snake;

struct Food {
    pub x: usize,
    pub y: usize,
}

impl Food {
    pub fn new(x: usize, y: usize) -> Self {
        Food { x, y }
    }
}

pub struct FoodManager {
    foods: Vec<Food>,
}

impl FoodManager {
    pub fn new() -> Self {
        FoodManager { foods: Vec::new() }
    }

    fn add_food(&mut self, x: usize, y: usize) {
        self.foods.push(Food::new(x, y));
    }

    pub fn take_food(&mut self, x: usize, y: usize) {
        self.foods.retain(|food| food.x != x || food.y != y);
    }

    pub fn is_food_at(&self, x: usize, y: usize) -> bool {
        self.foods.iter().any(|food| food.x == x && food.y == y)
    }

    pub fn render_foods_to_board(&self, board: &mut Board) {
        for food in &self.foods {
            board.set_cell(food.x, food.y, GameObject::Food);
        }
    }

    pub fn spawn_food(&mut self, board: &Board, snake: &Snake) {
        loop {
            let x = fastrand::usize(0..board.get_width());
            let y = fastrand::usize(0..board.get_height());
            if !board.is_wall_at(x, y) && !snake.is_snake_at(x, y) && !self.is_food_at(x, y) {
                self.add_food(x, y);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn food_manager_initializes_empty() {
        let manager = FoodManager::new();
        assert_eq!(manager.foods.len(), 0);
    }

    #[test]
    fn food_manager_checks_food_existence() {
        let mut manager = FoodManager::new();
        manager.add_food(3, 4);
        assert!(manager.is_food_at(3, 4));
        assert!(!manager.is_food_at(1, 1));
    }

    #[test]
    fn food_manager_can_take_food() {
        let mut manager = FoodManager::new();
        manager.add_food(5, 5);
        assert!(manager.is_food_at(5, 5));
        manager.take_food(5, 5);
        assert!(!manager.is_food_at(5, 5));
    }

    #[test]
    fn food_manager_renders_foods_to_board() {
        let mut manager = FoodManager::new();
        manager.add_food(0, 0);
        manager.add_food(9, 9);
        assert_eq!(manager.foods.len(), 2);
        let mut board = Board::new(10, 10, 1, 1);
        manager.render_foods_to_board(&mut board);
        assert_eq!(board.get_cell(0, 0), Some(GameObject::Food));
        assert_eq!(board.get_cell(9, 9), Some(GameObject::Food));
    }

    #[test]
    fn food_manager_spawns_food_randomly() {
        let mut board = Board::new(10, 10, 1, 1);
        let level_data = b"###########        ##  ##    ##        ##        ##    ##  ##        ##        ##        ###########".to_vec();
        board.set_level_data(&level_data);
        let mut snake = Snake::new(1, 1);
        snake.grow(5);

        let mut manager = FoodManager::new();
        for i in 0..10 {
            manager.spawn_food(&board, &snake);
            let food = &manager.foods[i];
            assert!(!board.is_wall_at(food.x, food.y));
            assert!(!snake.is_snake_at(food.x, food.y));
        }
        assert_eq!(manager.foods.len(), 10);
    }
}


