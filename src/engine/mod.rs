use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Player {
    Black,
    White,
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    world_map: [ [Option<Player>; 8]; 8],
}

impl Game {
    pub fn new() -> Game {
        let game: Game = Game { world_map: [
            [None, None, None, None, None, None, None, None],
            [None, None, Some(Player::Black), None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, Some(Player::White), None, None],
            [None, None, None, None, None, None, None, None],
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
                Some(idx) => { return_coords = (i, idx) }
            };
        }

        return_coords
    }

    fn move_is_valid(&self, p: &Player, d: &MoveDirection) -> bool {
        let (x, y) = self.find_player(p);

        const LOWER_WORLD_BOUND: usize = 0;
        const UPPER_WORLD_BOUND: usize = 7;

        if let MoveDirection::Down = d {
            if y == LOWER_WORLD_BOUND { return false };
        }

        if let MoveDirection::Up = d {
            if y == UPPER_WORLD_BOUND { return false };
        }

        if let MoveDirection::Left = d {
            if x == LOWER_WORLD_BOUND { return false };
        }

        if let MoveDirection::Right = d {
            if x == UPPER_WORLD_BOUND { return false };
        }

        true
    }

    pub fn player_move(&mut self, p: Player, direction: MoveDirection) {
        if !self.move_is_valid(&p, &direction) {
            return;
        } else {
            let (x, y) = self.find_player(&p);
            self.world_map[x][y] = None;
            match direction {
                MoveDirection::Down => {
                    self.world_map[x][y - 1] = Some(p);
                },
                MoveDirection::Up => {
                    self.world_map[x][y + 1] = Some(p);
                },
                MoveDirection::Left => {
                    self.world_map[x - 1][y] = Some(p);
                },
                MoveDirection::Right => {
                    self.world_map[x + 1][y] = Some(p);
                },
            }
        }
    }
}
