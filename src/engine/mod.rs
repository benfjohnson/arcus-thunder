use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Player {
    Black,
    White,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    // Map is such that (0, 0) denotes the top left corner, x and y grow to the right and bottom, respectively
    world_map: [ [Option<Player>; 8]; 8],
}

impl Game {
    pub fn new() -> Game {
        let game: Game = Game { world_map: [
            [Some(Player::Black), None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, Some(Player::White)],
        ] };
        game
    }

    fn find_player(&self, p: &Player) -> (usize, usize) {
        let mut return_coords = (0, 0);
        for (i, row) in self.world_map.iter().enumerate() {
            let player_col_idx = row.iter().position(|pos| {
                match pos {
                    Some(found_player) => {
                        match found_player {
                            Player::Black => {
                                match p {
                                    Player::Black => true,
                                    Player::White => false,
                                }
                            },
                            Player::White => {
                                match p {
                                    Player::White => true,
                                    Player::Black => false,
                                }
                            },
                        }
                    },
                    None => false,
                }
            });

            match player_col_idx {
                None => {},
                Some(idx) => { return_coords = (idx, i) }
            };
        }

        return_coords
    }

    fn move_is_valid(&self, p: &Player, d: &MoveDirection) -> bool {
        let (x, y) = self.find_player(p);

        const LOWER_WORLD_BOUND: usize = 0;
        const UPPER_WORLD_BOUND: usize = 7;

        match d {
            MoveDirection::Down => y != UPPER_WORLD_BOUND,
            MoveDirection::Up => y != LOWER_WORLD_BOUND,
            MoveDirection::Left => x != LOWER_WORLD_BOUND,
            MoveDirection::Right => x != UPPER_WORLD_BOUND,
        }
    }

    pub fn player_move(&mut self, p: Player, direction: MoveDirection) {
        if !self.move_is_valid(&p, &direction) {
            return;
        } else {
            let (x, y) = self.find_player(&p);
            self.world_map[y][x] = None;
            match direction {
                MoveDirection::Down => {
                    self.world_map[y + 1][x] = Some(p);
                },
                MoveDirection::Up => {
                    self.world_map[y - 1][x] = Some(p);
                },
                MoveDirection::Left => {
                    self.world_map[y][x - 1] = Some(p);
                },
                MoveDirection::Right => {
                    self.world_map[y][x + 1] = Some(p);
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_is_valid() {
        let test_g = Game::new();

        assert_eq!(test_g.world_map[0][0], Some(Player::Black));
        // Player starting at 0, 0 cannot move left or up, but can move right or down
        assert_eq!(test_g.move_is_valid(&Player::Black, &MoveDirection::Left), false);
        assert_eq!(test_g.move_is_valid(&Player::Black, &MoveDirection::Up), false);
        assert_eq!(test_g.move_is_valid(&Player::Black, &MoveDirection::Down), true);
        assert_eq!(test_g.move_is_valid(&Player::Black, &MoveDirection::Right), true);

    }

    #[test]
    fn test_player_move() {
        let mut test_g = Game::new();

        // White starts at (8, 8), with empty space to his or her left at (7, 8)
        assert_eq!(test_g.world_map[7][7], Some(Player::White));
        assert_eq!(test_g.world_map[7][6], None);

        test_g.player_move(Player::White, MoveDirection::Left);

        assert_eq!(test_g.world_map[7][7], None);
        assert_eq!(test_g.world_map[7][6], Some(Player::White));
    }
}
