use indexmap::IndexMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type Level = Vec<u8>;

pub struct LevelManager<'a> {
    level_size: usize,
    levels: IndexMap<&'a str, Level>,
}

impl<'a> LevelManager<'a> {
    pub fn new(level_size: usize) -> Self {
        LevelManager {
            level_size,
            levels: IndexMap::new(),
        }
    }

    pub fn get_level<'b>(&self, level_name: &str) -> Option<&Level> {
        self.levels.get(level_name)
    }

    pub fn add_level(&mut self, level_name: &'a str, level_data: &[u8]) -> Result<()>   {
        let level_data: Level = level_data
            .into_iter()
            .filter(|b| **b != b'\n' && **b != b'\r')
            .copied()
            .collect();
        let expected_size = self.level_size;
        let got_size = level_data.len();

        if expected_size == got_size {
            self.levels.insert(level_name, level_data);
            Ok(())
        } else {
            Err(format!("Invalid level size. Expected: {expected_size}, got: {got_size}").into())
        }
    }

    pub(crate) fn get_level_names(&self) -> Vec<String> {
        self.levels.keys().map(|&name| name.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_manager_adds_and_retrieves_levels() {
        let mut manager = LevelManager::new(81);
        let level_data = b"##########       ## ##### ## #   # ## # # # ## #   # ## ##### ##       ##########";
        manager.add_level("level1", level_data).unwrap();
        let retrieved_level = manager.get_level("level1").unwrap();
        assert_eq!(retrieved_level, level_data);
    }

    #[test]
    fn level_manager_can_add_level_with_line_breaks() {
        let mut manager = LevelManager::new(81);
        let level_data_with_breaks = b"##########\n       ##\n ##### ##\n #   # ##\n # # # ##\n #   # ##\n ##### ##\n       ##########";
        let clean_level_data: Vec<u8> = level_data_with_breaks
            .iter()
            .filter(|b| **b != b'\n' && **b != b'\r')
            .copied()
            .collect();
        manager.add_level("level2", level_data_with_breaks).unwrap();
        let retrieved_level = manager.get_level("level2").unwrap();
        assert_eq!(retrieved_level, &clean_level_data);
    }

    #[test]
    fn level_manager_returns_none_for_nonexistent_level() {
        let manager = LevelManager::new(81);
        let retrieved_level = manager.get_level("nonexistent_level");
        assert!(retrieved_level.is_none());
    }

    #[test]
    fn level_manager_rejects_invalid_level_size() {
        let mut manager = LevelManager::new(81);
        let invalid_level_data = b"##########       ## ##### ## #   # ## # # # ## #   # ## ##### ##       ########";
        let result = manager.add_level("level1", invalid_level_data);
        assert!(result.is_err());
    }

    #[test]
    fn level_manager_can_determine_level_names() {
        let mut manager = LevelManager::new(81);
        let level_data1 = b"##########       ## ##### ## #   # ## # # # ## #   # ## ##### ##       ##########";
        let level_data2 = b"##########       ## ##### ## #   # ## # # # ## #   # ## ##### ##       ##########";
        manager.add_level("level 1", level_data1).unwrap();
        manager.add_level("level 2", level_data2).unwrap();
        let level_names = manager.get_level_names();
        assert_eq!(level_names, vec!["level 1", "level 2"]);
    }
}