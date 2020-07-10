use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const MINIMUM_PLAYER_COUNT: usize = 3;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub color: String,
    id: Uuid,
}

impl Player {
    fn new(id: Uuid) -> Player {
        Player {
            id,
            color: rand::thread_rng().gen_range(0x000000, 0xFFFFFF).to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

struct PlayerLocData {
    coords: (usize, usize),
    player: Option<Player>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GameState {
    NotStarted,
    InProgress,
    Winner(Uuid),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    state: GameState,
    // Map is such that (0, 0) denotes the top left corner, x and y grow to the right and bottom, respectively
    world_map: [[Option<Player>; 8]; 8],
}

impl Game {
    pub fn new() -> Game {
        let game: Game = Game {
            state: GameState::NotStarted,
            world_map: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
        };
        game
    }

    fn find_player(&self, id: Uuid) -> PlayerLocData {
        let mut return_coords = (0, 0);
        let mut player: Option<Player> = None;
        for (i, row) in self.world_map.iter().enumerate() {
            let player_col_idx = row.iter().position(|pos| match pos {
                Some(found_player) => {
                    if found_player.id == id {
                        player = Some(found_player.clone());
                        true
                    } else {
                        false
                    }
                }
                None => false,
            });

            match player_col_idx {
                None => {}
                Some(idx) => return_coords = (idx, i),
            };
        }

        PlayerLocData {
            coords: return_coords,
            player,
        }
    }

    fn move_is_valid(&self, id: Uuid, d: &MoveDirection) -> bool {
        // Players can only move during an in progress game
        if self.state != GameState::InProgress {
            eprintln!("tried to move during a game not in progress...");
            return false;
        }

        let PlayerLocData {
            coords: (x, y),
            player: _,
        } = self.find_player(id);

        const LOWER_WORLD_BOUND: usize = 0;
        const UPPER_WORLD_BOUND: usize = 7;

        match d {
            MoveDirection::Down => y != UPPER_WORLD_BOUND,
            MoveDirection::Up => y != LOWER_WORLD_BOUND,
            MoveDirection::Left => x != LOWER_WORLD_BOUND,
            MoveDirection::Right => x != UPPER_WORLD_BOUND,
        }
    }

    pub fn player_move(&mut self, id: Uuid, direction: MoveDirection) -> bool {
        if !self.move_is_valid(id, &direction) {
            return false;
        }

        let PlayerLocData {
            coords: (x, y),
            player,
        } = self.find_player(id);

        if let None = player {
            return false;
        }
        self.world_map[y][x] = None;

        match direction {
            MoveDirection::Down => {
                self.world_map[y + 1][x] = player;
            }
            MoveDirection::Up => {
                self.world_map[y - 1][x] = player;
            }
            MoveDirection::Left => {
                self.world_map[y][x - 1] = player;
            }
            MoveDirection::Right => {
                self.world_map[y][x + 1] = player;
            }
        };

        true
    }

    pub fn add_player(&mut self, id: Uuid) -> Uuid {
        let p = Player::new(id);

        // if player with this id exists, just return the id, don't add to the world map
        if let Some(_) = self.world_map.iter().position(|row| {
            match row.iter().position(|comp_player| match comp_player {
                None => false,
                Some(cp) => cp.id == id,
            }) {
                None => false,
                Some(_) => true,
            }
        }) {
            return id;
        }

        // insert new player with passed in id into the first open spot
        for (y, row) in self.world_map.iter().enumerate() {
            let empty_space_idx = row.iter().position(|pos| match pos {
                None => true,
                Some(_) => false,
            });

            if let Some(x) = empty_space_idx {
                println!("adding a player at ({}, {})!", x, y);
                self.world_map[y][x] = Some(p);
                // if we have the minimum number of required players, set game to in progress!
                if self.player_count() >= MINIMUM_PLAYER_COUNT && self.state == GameState::NotStarted {
                    self.state = GameState::InProgress;
                }
                break;
            }
        }

        // successfully added our player, return her id
        id
    }

    fn player_count(&self) -> usize {
        self.world_map.iter().fold(0, |map_accum, row| {
            map_accum + row.iter().fold(0, |row_accum, pos| {
                match pos {
                    None => row_accum,
                    Some(_) => row_accum + 1,
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_move_prevention() {
        let mut test_g = Game::new();
        let new_player_id = test_g.add_player(Uuid::new_v4());

        // player at 0,0 should be able to move down, but only if game has begun!
        assert_eq!(test_g.player_move(new_player_id, MoveDirection::Down), false);
        assert_eq!(test_g.state, GameState::NotStarted);
    }

    #[test]
    fn test_move_is_valid() {
        let mut test_g = Game::new();

        // initialize with at three players so that the game is considered in progress
        let new_player_id = test_g.add_player(Uuid::new_v4());
        for _ in 0..2 {
            test_g.add_player(Uuid::new_v4());
        }

        match &test_g.world_map[0][0] {
            Some(matched_player) => {
                assert_eq!(matched_player.id, new_player_id);
            }
            None => panic!("Did not add player as expected"),
        }

        // player starting at 0, 0 cannot move left or up, but can move right or down
        assert_eq!(
            test_g.move_is_valid(new_player_id, &MoveDirection::Left),
            false
        );
        assert_eq!(
            test_g.move_is_valid(new_player_id, &MoveDirection::Up),
            false
        );
        assert_eq!(
            test_g.move_is_valid(new_player_id, &MoveDirection::Down),
            true
        );
        assert_eq!(
            test_g.move_is_valid(new_player_id, &MoveDirection::Right),
            true
        );
    }

    #[test]
    fn test_player_move() {
        let mut test_g = Game::new();

        let new_player_id = test_g.add_player(Uuid::new_v4());
        for _ in 0..2 {
            test_g.add_player(Uuid::new_v4());
        }

        match &test_g.world_map[0][0] {
            Some(matched_player) => {
                assert_eq!(matched_player.id, new_player_id);
            }
            None => panic!("Did not add player as expected"),
        }

        // player hasn't moved down yet
        assert_eq!(test_g.world_map[1][0], None);
        let success = test_g.player_move(new_player_id, MoveDirection::Down);
        assert_eq!(success, true);
        assert_eq!(test_g.world_map[0][0], None);
        assert_ne!(test_g.world_map[1][0], None);
    }

    #[test]
    fn test_player_count() {
        let mut test_g = Game::new();

        // add 14 players
        for _ in 0..14 {
            test_g.add_player(Uuid::new_v4());
        }

        println!("here it is: {:?}", test_g.world_map);

        assert_eq!(test_g.player_count(), 14);
    }
}
