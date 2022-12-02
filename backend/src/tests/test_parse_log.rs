#[cfg(test)]
pub mod parse_tests {
    use crate::models::game::Game;

    #[test]
    pub fn try_parse_log() {
        let json_log = std::fs::read_to_string("test_log.json").unwrap();
        let mut game_log: Game = serde_json::from_str(&json_log).unwrap();
        game_log.log = Some(serde_json::from_str(&json_log).unwrap());
        println!("{:?}", game_log);
    }
}
