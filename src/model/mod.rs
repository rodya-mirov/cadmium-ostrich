use crate::utils::numerics::AbsDiff;

pub const GAME_COLS: usize = 500;
pub const GAME_ROWS: usize = 500;

pub const VIEW_COLS: usize = 70;
pub const VIEW_ROWS: usize = 30;

pub type ViewChars = Vec<Vec<char>>;

pub struct GameState {
    pub tiles: Vec<Vec<Tile>>,
    pub view_port: ViewPort,
    pub agents: Vec<Agent>,
}

pub struct ViewPort {
    pub x: usize,
    pub y: usize,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Tile {
    X,
    O,
    Boundary,
}

pub enum Agent {
    Mob(MobData),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MobData {
    pub x: usize,
    pub y: usize,
    pub x_vel: i32,
    pub y_vel: i32,
}

impl Default for GameState {
    fn default() -> GameState {
        let mut tiles = Vec::with_capacity(GAME_ROWS);
        for y in 0..GAME_ROWS {
            let mut row = Vec::with_capacity(GAME_COLS);
            for x in 0..GAME_COLS {
                row.push(default_tile(x, y));
            }
            tiles.push(row);
        }

        let agents = vec![
            Agent::Mob(MobData {
                x: 2,
                y: 3,
                x_vel: 2,
                y_vel: 1,
            }),
            Agent::Mob(MobData {
                x: 20,
                y: 6,
                x_vel: -1,
                y_vel: 1,
            }),
            Agent::Mob(MobData {
                x: 20,
                y: 6,
                x_vel: -1,
                y_vel: 1,
            }),
            Agent::Mob(MobData {
                x: 20,
                y: 6,
                x_vel: -1,
                y_vel: 1,
            }),
            Agent::Mob(MobData {
                x: 30,
                y: 16,
                x_vel: 1,
                y_vel: 1,
            }),
            Agent::Mob(MobData {
                x: 17,
                y: 2,
                x_vel: 1,
                y_vel: -1,
            }),
        ];

        GameState {
            tiles,
            view_port: ViewPort { x: 0, y: 0 },
            agents,
        }
    }
}

fn default_tile(x: usize, y: usize) -> Tile {
    if x.abs_diff(0) < 5
        || x.abs_diff(GAME_COLS - 1) < 5
        || y.abs_diff(0) < 5
        || y.abs_diff(GAME_ROWS - 1) < 5
    {
        Tile::Boundary
    } else if x > 30 && y > 25 {
        Tile::X
    } else {
        Tile::O
    }
}

impl Tile {
    fn display_char(&self) -> char {
        match &self {
            Tile::X => 'x',
            Tile::O => ' ',
            Tile::Boundary => '!',
        }
    }
}

impl GameState {
    pub fn to_display_string(&self) -> ViewChars {
        let view_port = &self.view_port;

        let mut tiles: ViewChars = (view_port.y..view_port.y + VIEW_ROWS)
            .map(|y| {
                (view_port.x..view_port.x + VIEW_COLS)
                    .map(|x| self.tiles[y][x].display_char())
                    .collect()
            })
            .collect();

        for agent in &self.agents {
            if let Some((x, y, dc)) = agent.display_char() {
                if let Some((rel_x, rel_y)) = view_port.to_relative(x, y) {
                    tiles[rel_y][rel_x] = dc;
                }
            }
        }

        tiles
    }
}

impl ViewPort {
    fn to_relative(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x < self.x || x >= self.x + VIEW_COLS || y < self.y || y >= self.y + VIEW_ROWS {
            return None;
        }

        Some((x - self.x, y - self.y))
    }
}

impl Agent {
    fn display_char(&self) -> Option<(usize, usize, char)> {
        match self {
            Agent::Mob(mob) => mob.display_char(),
        }
    }
}

impl MobData {
    fn display_char(&self) -> Option<(usize, usize, char)> {
        Some((self.x, self.y, 'M'))
    }
}

pub enum UserEvent {
    KeyPress(KeyEvent),
}

pub enum KeyEvent {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
