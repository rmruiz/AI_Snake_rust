use crate::point;
use point::Point;
use ndarray::{Array2,array};

use rand::Rng;

const BOARD_SIZE: usize = 18;
const STEPS_UNTIL_DEATH: usize = 6 * BOARD_SIZE; // 6 x BOARD SIZE

const POINTS_PER_APPLE: usize = 300; // 3 times STEPS_UNTIL_DEATH
const POINTS_PER_STEP: usize = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl Direction {
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => panic!("Invalid direction value: {}", value),
        }
    }
}

pub struct Snakegame {
    pub apples_eaten: usize,
    pub alive: bool,
    steps_until_death: usize,
    total_steps: usize,
    direction: Direction,
    score: usize,
    snake: Vec<Point>,
    apple_position: Point,
    pub killed_by_wall: bool,
    pub killed_by_myself: bool,
    pub killed_by_hunger: bool,
}

impl Snakegame {
    pub fn new() -> Self{        
        let snake: Vec<Point> = vec![
            Point { x: 9, y: 12 },
            Point { x: 9, y: 11 },
            Point { x: 9, y: 10 },
            Point { x: 9, y: 9 }
            ];
        let mut rng = rand::rng();
        let mut apple_position: Point;
        loop {
            apple_position = Point { x: rng.random_range(0..BOARD_SIZE), y: rng.random_range(0..BOARD_SIZE) };
            if !snake.contains(&apple_position) {
                break;
            }
        }

        Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: STEPS_UNTIL_DEATH,
            total_steps: 0,
            direction: Direction::North,
            score: 0,
            snake: snake,
            apple_position: apple_position,
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        }
    }

    pub fn get_current_input(&self) -> Array2<f64> {
        let dir_input = match self.direction {
            Direction::North => -1.0,
            Direction::South => -0.333333333,
            Direction::East  =>  0.333333333,
            Direction::West  => 1.0,
        };

        array![
            [dir_input],
            [self.distance_to_north_south_wall()],
            [self.distance_to_west_east_wall()],
            [self.have_snake_in_direction(Direction::North)],
            [self.have_snake_in_direction(Direction::South)],
            [self.have_snake_in_direction(Direction::East)],
            [self.have_snake_in_direction(Direction::West)],
            [self.get_fruit_north_south_distance()],
            [self.get_fruit_east_west_distance()],
            [self.distance_fruit_infront()],
        ]
    }

    pub fn have_snake_in_direction(&self, direction: Direction) -> f64 {
        let mut pos = self.get_snake_head_pos();
        let scores = [1.0, 0.8, 0.5, 0.1];

        let advance = match direction {
            Direction::North => |p: Point| p.north(),
            Direction::South => |p: Point| p.south(),
            Direction::East  => |p: Point| p.east(),
            Direction::West  => |p: Point| p.west(),
        };

        for &score in &scores {
            pos = advance(pos);
            if self.snake.contains(&pos) {
                return score;
            }
        }
        -1.0
    }

    pub fn distance_to_north_south_wall(&self) -> f64 {
        let y_pos = self.get_snake_head_pos().y as f64;
        let percentage = (y_pos - 1.0) / (BOARD_SIZE as f64 - 1.0);
        percentage * 2.0 - 1.0
    }

    pub fn distance_to_west_east_wall(&self) -> f64 {
        let x_pos = self.get_snake_head_pos().x as f64;
        let percentage = (x_pos - 1.0) / (BOARD_SIZE as f64 - 1.0);
        percentage * 2.0 - 1.0
    }

    pub fn distance_fruit_infront(&self) -> f64 {
        let mut pos = self.get_snake_head_pos();

        let advance: fn(&Point) -> Point = match self.direction {
            Direction::North => |p: &Point| p.north(),
            Direction::South => |p: &Point| p.south(),
            Direction::East  => |p: &Point| p.east(),
            Direction::West  => |p: &Point| p.west(),
        };

        let rewards = [1.0, 0.9, 0.7, 0.4, 0.0];

        for reward in rewards {
            pos = advance(&pos);
            if pos == self.apple_position {
                return reward;
            }
        }

        -1.0
    }

    pub fn get_fruit_north_south_distance(&self) -> f64 {
        let head_y = self.get_snake_head_pos().y as f64;
        let apple_y = self.apple_position.y as f64;
        (head_y - apple_y) / (BOARD_SIZE as f64 - 1.0)
    }

    pub fn get_fruit_east_west_distance(&self) -> f64 {
        let head_x = self.get_snake_head_pos().x as f64;
        let apple_x = self.apple_position.x as f64;
        (head_x - apple_x) / (BOARD_SIZE as f64 - 1.0)
    }

    fn get_snake_head_pos(&self) -> Point {
        self.snake.last().copied().expect("Snake should never be empty")
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn move_snake(&mut self, new_direction: Direction) 
    {
        if new_direction == Direction::North && self.direction == Direction::South ||
        new_direction == Direction::South && self.direction == Direction::North ||
        new_direction == Direction::East && self.direction == Direction::West ||
        new_direction == Direction::West && self.direction == Direction::East {
            self.alive = false;
            self.killed_by_myself = true;
        }
        else {
            self.direction = new_direction;
        }


        let head = self.get_snake_head_pos();

        let next_head_position = match self.direction {
            Direction::North => head.north(),
            Direction::South => head.south(),
            Direction::East  => head.east(),
            Direction::West  => head.west(),
        };

        let mut got_apple: bool = false;

        if self.apple_position == next_head_position {
            self.score += POINTS_PER_APPLE;
            self.apples_eaten += 1;
            self.steps_until_death = STEPS_UNTIL_DEATH + 1;
            got_apple = true;
        }
        else if head == next_head_position { //this means we git the wall
            self.alive = false;
            self.killed_by_wall = true;
        }
        else if self.snake.contains(&next_head_position) {
            self.alive = false;
            self.killed_by_myself = true;
        }

        self.snake.push(next_head_position);

        if got_apple {
            self.apple_position = new_fruit(&self.snake);
        }
        else {
            self.snake.remove(0);
        }

        self.steps_until_death -= 1;

        if self.steps_until_death == 0 {
            self.alive = false;
            self.killed_by_hunger = true;
        }
        self.total_steps +=1;
        self.score += POINTS_PER_STEP;

    }

}

fn new_fruit(snake: &Vec<Point>) -> Point {
        let mut rng = rand::rng();
        loop {
            let x: usize = rng.random_range(0..BOARD_SIZE);
            let y: usize = rng.random_range(0..BOARD_SIZE);
            let point: Point = Point { x: x, y: y };
            if !snake.contains(&point) {
                return point;
            }
        }
    }
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fruit_not_on_snake() {
        let snake: Vec<Point> = vec![
            Point { x: 9, y: 12 },
            Point { x: 9, y: 11 },
            Point { x: 9, y: 10 },
            Point { x: 9, y: 9 }
            ];
        let fruit = new_fruit(&snake);

        // Check that the fruit is not on the snake
        assert!(
            !snake.contains(&fruit),
            "Fruit was placed on the snake at position {:?}",
            fruit
        );

        // Check that the fruit is within board bounds
        assert!(
            fruit.x < BOARD_SIZE && fruit.x < BOARD_SIZE && fruit.y < BOARD_SIZE && fruit.y < BOARD_SIZE,
            "Fruit position {:?} is out of bounds",
            fruit
        );
    }

    #[test]
    fn test_get_snake_head_pos_returns_correct_point() {
        let game = Snakegame::new();

        // The initial snake is hardcoded in Snakegame::new()
        // Last point should be the head
        let expected_head = Point { x: 9, y: 9 };

        let actual_head = game.get_snake_head_pos();

        assert_eq!(
            actual_head,
            expected_head,
            "Expected head at {:?}, got {:?}",
            expected_head,
            actual_head
        );
    }

    fn point(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn create_game(snake: Vec<Point>, direction: Direction, apple: Point, steps: usize) -> Snakegame {
        Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: steps,
            total_steps: 0,
            direction,
            score: 0,
            snake,
            apple_position: apple,
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        }
    }

    #[test]
    fn valid_movement_without_eating_apple() {
        let mut game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: STEPS_UNTIL_DEATH,
            total_steps: 0,
            direction: Direction::North,
            score: 0,
            snake: vec![Point { x: 5, y: 5 }],
            apple_position: Point { x: 0, y: 0 }, // not at next pos
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        game.move_snake(Direction::East);

        assert_eq!(game.get_snake_head_pos(), Point { x: 6, y: 5 });
        assert!(game.alive);
        assert_eq!(game.snake.len(), 1, "Snake should not grow if no apple was eaten");
    }

    #[test]
    fn valid_movement_snake_grows_when_eating_apple() {
        let mut game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: STEPS_UNTIL_DEATH,
            total_steps: 0,
            direction: Direction::East,
            score: 0,
            snake: vec![Point { x: 5, y: 5 }],
            apple_position: Point { x: 6, y: 5 }, // directly in path
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        game.move_snake(Direction::East);

        assert_eq!(game.get_snake_head_pos(), Point { x: 6, y: 5 });
        assert!(game.alive);
        assert_eq!(game.apples_eaten, 1);
        assert!(game.score > 0);
        assert_eq!(game.snake.len(), 2, "Snake should grow to size 2 after eating apple");
    }

    #[test]
    fn invalid_direction_reversal_kills_snake() {
        let mut game = create_game(vec![point(5, 5)], Direction::North, point(0, 0), STEPS_UNTIL_DEATH);
        game.move_snake(Direction::South);
        assert!(!game.alive, "Snake should die when reversing direction");
    }

    #[test]
    fn steps_until_death_kills_snake() {
        let mut game = create_game(vec![point(5, 5)], Direction::East, point(0, 0), 1);
        game.move_snake(Direction::East);
        assert!(!game.alive, "Snake should die when out of steps");
    }

    #[test]
    fn wall_collision_kills_snake() {
        // Point at top wall, moving north will .saturating_sub(1) to 0, which is same as head
        let mut game = create_game(vec![point(5, 0)], Direction::North, point(0, 0), STEPS_UNTIL_DEATH);
        game.move_snake(Direction::North);
        assert!(!game.alive, "Snake should die if it doesn't move (hits wall/self)");
    }

    #[test]
    fn snake_dies_on_self_collision() {
        let snake = vec![
            point(2, 2), point(3, 2), point(4, 2),
            point(4, 3), point(3, 3), point(2, 3),
        ];
        let mut game = create_game(snake, Direction::East, point(0, 0), 10);
        game.move_snake(Direction::North); // (2,3) → (2,2), hits body
        assert!(!game.alive, "Snake should die colliding into itself");
    }

        #[test]
    fn test_fruit_east_west_distance_left_of_head() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::East,
            score: 0,
            snake: vec![Point { x: 10, y: 5 }],
            apple_position: Point { x: 4, y: 5 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        let dist = game.get_fruit_east_west_distance();
        let expected = (10.0 - 4.0) / (BOARD_SIZE as f64 - 1.0);
        assert!((dist - expected).abs() < 1e-6);
    }

    #[test]
    fn test_fruit_east_west_distance_right_of_head() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::East,
            score: 0,
            snake: vec![Point { x: 4, y: 5 }],
            apple_position: Point { x: 10, y: 5 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        let dist = game.get_fruit_east_west_distance();
        let expected = (4.0 - 10.0) / (BOARD_SIZE as f64 - 1.0);
        assert!((dist - expected).abs() < 1e-6);
    }

    #[test]
    fn test_distance_fruit_infront_one_step() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::East,
            score: 0,
            snake: vec![Point { x: 3, y: 5 }],
            apple_position: Point { x: 4, y: 5 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        assert_eq!(game.distance_fruit_infront(), 1.0);
    }

    #[test]
    fn test_distance_fruit_infront_four_steps() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::North,
            score: 0,
            snake: vec![Point { x: 4, y: 6 }],
            apple_position: Point { x: 4, y: 2 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        assert_eq!(game.distance_fruit_infront(), 0.4);
    }

    #[test]
    fn test_distance_fruit_infront_not_in_path() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::West,
            score: 0,
            snake: vec![Point { x: 10, y: 10 }],
            apple_position: Point { x: 3, y: 9 }, // not in direct path
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        assert_eq!(game.distance_fruit_infront(), -1.0);
    }

    #[test]
    fn test_snake_immediately_north() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::North,
            score: 0,
            snake: vec![
                Point { x: 5, y: 4 }, // ← directly north
                Point { x: 5, y: 5 }, // head
            ],
            apple_position: Point { x: 0, y: 0 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        let score = game.have_snake_in_direction(Direction::North);
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_snake_three_steps_east() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::East,
            score: 0,
            snake: vec![
                Point { x: 8, y: 5 }, // 3 steps east
                Point { x: 5, y: 5 }, // head
            ],
            apple_position: Point { x: 0, y: 0 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        let score = game.have_snake_in_direction(Direction::East);
        assert_eq!(score, 0.5);
    }

    #[test]
    fn test_snake_four_steps_south() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::South,
            score: 0,
            snake: vec![
                Point { x: 10, y: 14 }, // 4 steps south
                Point { x: 10, y: 10 }, // head
            ],
            apple_position: Point { x: 0, y: 0 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        let score = game.have_snake_in_direction(Direction::South);
        assert_eq!(score, 0.1);
    }

    #[test]
    fn test_no_snake_in_direction() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::West,
            score: 0,
            snake: vec![
                Point { x: 2, y: 5 }, // far away
                Point { x: 10, y: 10 }, // head
            ],
            apple_position: Point { x: 0, y: 0 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        let score = game.have_snake_in_direction(Direction::West);
        assert_eq!(score, -1.0);
    }

    #[test]
    fn test_snake_two_steps_west() {
        let game = Snakegame {
            apples_eaten: 0,
            alive: true,
            steps_until_death: 10,
            total_steps: 0,
            direction: Direction::West,
            score: 0,
            snake: vec![
                Point { x: 3, y: 5 }, // 2 steps west
                Point { x: 5, y: 5 }, // head
            ],
            apple_position: Point { x: 0, y: 0 },
            killed_by_hunger: false,
            killed_by_myself: false,
            killed_by_wall: false
        };

        let score = game.have_snake_in_direction(Direction::West);
        assert_eq!(score, 0.8);
    }
}
