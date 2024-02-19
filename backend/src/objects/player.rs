use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{Direction, Event, Vector2};
#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: i32,
    username: String,
    positions: Vec<Vector2>,
    direction: Direction,
    state: PlayerState,

    #[ts(skip)]
    diffs: Vec<Event>
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerState {
    Waiting(i32),
    Connecting,
    Dead(i32),
    Running
}

impl Player {
    /**
     * Create a new player
     */
    pub fn new(id: i32) -> Self {
        Self {
            direction: Direction::Up,
            id,
            positions: Vec::new(),
            username: String::new(),
            state: PlayerState::Connecting,
            diffs: Vec::new()
        }
    }
    /**
     * Set the state of the player
     */
    fn set_state(&mut self, state: PlayerState) {
        self.diffs.push(Event::ChangeStatePlayer { state: state.clone(), id: self.id });
        self.state = state
    }
    /**
     * Handle the game logic
     */
    pub fn update(&mut self, size: &Vector2) {
        match self.state {
            PlayerState::Waiting(1) => {
                self.set_state(PlayerState::Running)
            }
            PlayerState::Waiting(n) => {
                self.set_state(PlayerState::Waiting(n-1))
            },
            PlayerState::Running => {
                let dir = match self.direction {
                    Direction::Up => Vector2::new(0, -1),
                    Direction::Down => Vector2::new(0, 1),
                    Direction::Left => Vector2::new(-1, 0),
                    Direction::Right => Vector2::new(1, 0),
                };
                let new_pos = self.head() + dir;
                if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= size.x || new_pos.y >= size.y {
                    self.kill();
                } else {
                    self.positions.push(new_pos);
                    self.positions.remove(0);
                };
            },
            PlayerState::Dead(0) => {},
            PlayerState::Dead(n) => {
                self.set_state(PlayerState::Dead(n-1))
            }
            PlayerState::Connecting => {},
        }
    }
    /**
     * increase the size of the player if it eats an apple
     */
    pub fn increase(&mut self) {
        let pos = self.positions.iter().next().unwrap().clone();
        self.positions.insert(0, pos);
        self.diffs.push(Event::IncreasePlayer(self.id))
    }
    /**
     * Test if the head intersect with a position
     */
    pub fn intersect(&self, pos: &Vector2) -> bool {
        self.positions.iter().any(|p| p == pos)
    }
    /**
     * Test if the head intersect an apple
     */
    pub fn intersect_apple(&self, apple: &Vector2) -> bool {
        &self.head() == apple
    }
    /**
     * Get the head of the player
     */
    fn head(&self) -> Vector2 {
        self.positions.last().unwrap().clone()
    }
    /**
     * Check if the head intersect the head of another player or our body
     */
    pub fn intersect_player(&self, other: &Player) -> bool {
        if other == self {
            other.positions[0..other.positions.len()-1].contains(&self.head())
        } else {
            other.intersect(&self.head())
        }
    }
    /**
     * Kill the player
     */
    pub fn kill(&mut self) {
        if self.state == PlayerState::Running {
            self.set_state(PlayerState::Dead(12));
        }
    }
    /**
     * Change username and change state to running
     */
    pub fn set_username(&mut self, username: String) {
        if let PlayerState::Connecting = self.state {
            self.set_state(PlayerState::Waiting(12));
            self.username = username.clone();
            self.diffs.push(Event::SetUsername { id: self.id, name: username })
        }
    }
    /**
     * Get the id
     */
    pub fn id(&self) -> i32 {
        self.id
    }
    /**
     * Change the direction of the player
     */
    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.reverse() != direction {
            self.diffs.push(Event::MovePlayer { dir: direction.clone(), id: self.id });
            self.direction = direction
        }
    }
    /**
     * Increase the size of the player by adding a specific position
     */
    pub fn add_position(&mut self, position: Vector2) {
        self.positions.insert(0, position)
    }
    /**
     * Get the state
     */
    pub fn state(&self) -> &PlayerState {
        &self.state
    }
    /**
     * Get the username
     */
    pub fn username(&self) -> &String {
        &self.username
    }
    /**
     * Get the changes to send to the client, and clear the list
     */
    pub fn diff(&mut self) -> Vec<Event> {
        let list = self.diffs.to_owned();
        self.diffs = Vec::new();
        return list
    }

    pub fn score(&self) -> i32 {
        self.positions.len() as i32
    }


}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_player() -> Player {
        Player {
            diffs: Default::default(),
            direction: Direction::Up,
            id: Default::default(),
            username: Default::default(),
            state: PlayerState::Running,
            positions: vec!(
                Vector2::new(1, 3),
                Vector2::new(2, 3),
                Vector2::new(3, 3),
            )
        }
    }

    #[test]
    fn intersect() {
        let player = example_player();

        player.positions.iter().for_each(|p| assert_eq!(true, player.intersect(p)));

        assert_eq!(false, player.intersect(&Vector2::new(-1, 0)));
        assert_eq!(false, player.intersect(&Vector2::new(0, -1)));
        assert_eq!(false, player.intersect(&Vector2::new(3, 4)));
    }

    #[test]
    fn intersect_apple() {
        let player = example_player();

        assert_eq!(true, player.intersect_apple(&Vector2::new(3,3)));
        assert_eq!(false, player.intersect_apple(&Vector2::new(3,2)));
        assert_eq!(false, player.intersect_apple(&Vector2::new(3,4)));
        assert_eq!(false, player.intersect_apple(&Vector2::new(2,3)));
        assert_eq!(false, player.intersect_apple(&Vector2::new(4,3)));
        assert_eq!(false, player.intersect_apple(&Vector2::new(0,0)));
    }

    #[test]
    fn intersect_player_same() {
        let player = example_player();
        let other_player = example_player();

        assert_eq!(false, player.intersect_player(&other_player));
    }
    #[test]
    fn intersect_player_not_same() {
        let player = example_player();
        let mut other_player = example_player();

        other_player.id = 1;

        assert_eq!(true,  player.intersect_player(&other_player));
    }
    #[test]
    fn intersect_player_close() {
        let player = example_player();
        let mut other_player = example_player();

        other_player.id = 1;
        other_player.positions = vec!(
            Vector2::new(3, 2),
            Vector2::new(3, 3),
            Vector2::new(3, 4),
        );

        assert_eq!(true,  player.intersect_player(&other_player));


        other_player.positions = vec!(
            Vector2::new(4, 2),
            Vector2::new(4, 3),
            Vector2::new(4, 4),
        );
        assert_eq!(false,  player.intersect_player(&other_player));
    }

    #[test]
    fn intersect_player_too_late() {
        let player = example_player();
        let mut other_player = example_player();

        other_player.id = 1;
        other_player.positions = vec!(
            Vector2::new(2, 2),
            Vector2::new(2, 3),
            Vector2::new(2, 4),
        );

        assert_eq!(false,  player.intersect_player(&other_player));
    }
    #[test]
    fn intersect_player_tail_not_touching() {
        let player = example_player();
        let mut other_player = example_player();

        other_player.positions = vec!(
            Vector2::new(4, 3),
            Vector2::new(5, 3),
            Vector2::new(6, 3),
        );

        assert_eq!(false,  player.intersect_player(&other_player));
    }
    #[test]
    fn intersect_player_tail_touching() {
        let player = example_player();
        let mut other_player = example_player();

        other_player.positions = vec!(
            Vector2::new(3, 3),
            Vector2::new(4, 3),
            Vector2::new(5, 3),
        );

        assert_eq!(true,  player.intersect_player(&other_player));
    }
    #[test]
    fn test_increase() {
        let mut player = example_player();

        let vec = vec!(
            Vector2::new(1, 3),
            Vector2::new(2, 3),
            Vector2::new(3, 3),
        );
        let expected = vec!(
            Vector2::new(1, 3),
            Vector2::new(1, 3),
            Vector2::new(2, 3),
            Vector2::new(3, 3),
        );

        assert_eq!(vec, player.positions);
        player.increase();
        assert_eq!(expected, player.positions);
    }
    #[test]
    fn test_move() {
        let mut player = example_player();

        let vec = vec!(
            Vector2::new(1, 3),
            Vector2::new(2, 3),
            Vector2::new(3, 3),
        );
        let expected = vec!(
            Vector2::new(2, 3),
            Vector2::new(3, 3),
            Vector2::new(3, 2),
        );

        assert_eq!(vec, player.positions);
        player.update(&Vector2::new(100, 100));
        assert_eq!(expected, player.positions);
    }
    #[test]
    fn test_move_outside() {
        let mut player = example_player();

        player.update(&Vector2::new(4, 4));
        assert_eq!(PlayerState::Running, player.state);
        assert_eq!(Vector2::new(3, 2), player.head());

        player.update(&Vector2::new(4, 4));
        assert_eq!(PlayerState::Running, player.state);
        assert_eq!(Vector2::new(3, 1), player.head());

        player.update(&Vector2::new(4, 4));
        assert_eq!(PlayerState::Running, player.state);
        assert_eq!(Vector2::new(3, 0), player.head());

        player.update(&Vector2::new(4, 4));
        assert_eq!(PlayerState::Dead(12), player.state);
        assert_eq!(Vector2::new(3, 0), player.head());
    }
    #[test]
    fn test_move_not_running() {
        let mut player = example_player();
        let pos = player.positions.clone();

        let states = vec!(
            PlayerState::Waiting(0),
            PlayerState::Connecting,
            PlayerState::Dead(0),
        );

        states.into_iter().for_each(|s| {
            player.state = s;
            player.update(&Vector2::new(10, 10));
            assert_eq!(pos, player.positions);
        })
    }
    #[test]
    fn test_kill_running() {
        let mut player = example_player();
        player.kill();
        assert_eq!(PlayerState::Dead(12), player.state);
    }
    #[test]
    fn test_diffs() {
        let mut player = example_player();

        assert_eq!(Vec::<Event>::new(), player.diffs);

        player.update(&Vector2::new(10, 10));

        assert_eq!(Vec::<Event>::new(), player.diffs);

        player.set_direction(Direction::Left);

        assert_eq!(vec!(
            Event::MovePlayer {dir: Direction::Left, id: 0}
        ), player.diffs);

        player.update(&Vector2::new(10, 10));

        assert_eq!(vec!(
            Event::MovePlayer {dir: Direction::Left, id: 0}
        ), player.diffs);

        player.increase();

        assert_eq!(vec!(
            Event::MovePlayer {dir: Direction::Left, id: 0},
            Event::IncreasePlayer(0)
        ), player.diffs);

        player.kill();

        assert_eq!(vec!(
            Event::MovePlayer {dir: Direction::Left, id: 0},
            Event::IncreasePlayer(0),
            Event::ChangeStatePlayer{id: 0, state: PlayerState::Dead(12)},
        ), player.diff());
        assert_eq!(Vec::<Event>::new(), player.diff());


    }
    #[test]
    fn test_opposite_direction(){
        let mut player = example_player();

        assert_eq!(Direction::Up, player.direction);

        player.set_direction(Direction::Left);
        assert_eq!(Direction::Left, player.direction);

        player.set_direction(Direction::Right);
        assert_eq!(Direction::Left, player.direction);

        player.set_direction(Direction::Down);
        assert_eq!(Direction::Down, player.direction);

        player.set_direction(Direction::Up);
        assert_eq!(Direction::Down, player.direction);
    }
}