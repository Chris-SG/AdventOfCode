pub fn solve_1() {
    let input = parse_input(include_str!("input"));
    let result = calculate_delta(&input);
    println!("Input from file has vertical of {} and horizontal of {}! Total of {}", result.vertical, result.horizontal, result.vertical * result.horizontal);
}

pub fn solve_2() {
    let input = parse_input(include_str!("input"));
    let result = calculate_delta_by_aim(&input);
    println!("Input from file has vertical of {} and horizontal of {}! Total of {}", result.vertical, result.horizontal, result.vertical * result.horizontal);
}

fn calculate_delta(movement: &Vec<Movement>) -> Position {
    let mut position = Position {
        horizontal: 0,
        vertical: 0
    };
    movement
        .iter()
        .for_each(|m| match m.direction {
            Directions::DOWN => position.vertical += m.distance,
            Directions::UP => position.vertical -= m.distance,
            Directions::FORWARD => position.horizontal += m.distance,
            Directions::BACKWARD => position.horizontal -= m.distance,
            Directions::NONE => println!("Direction of {} with no movement!", m.distance),
        });
    position
}

fn calculate_delta_by_aim(movement: &Vec<Movement>) -> Position {
    let mut aim = 0;
    let mut position = Position {
        horizontal: 0,
        vertical: 0
    };
    movement
        .iter()
        .for_each(|m| match m.direction {
            Directions::DOWN => aim += m.distance,
            Directions::UP => aim -= m.distance,
            Directions::FORWARD => {
                position.horizontal += m.distance;
                position.vertical += m.distance * aim;
            },
            Directions::BACKWARD => {
                position.horizontal -= m.distance;
                position.vertical -= m.distance * aim;
            },
            Directions::NONE => println!("Direction of {} with no movement!", m.distance),
        });
    position
}

#[derive(Debug, PartialEq)]
enum Directions {
    NONE,
    UP,
    DOWN,
    FORWARD,
    BACKWARD,
}

#[derive(Debug)]
struct Movement {
    direction: Directions,
    distance: i64,
}

impl PartialEq<Self> for Movement {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance && self.direction == other.direction
    }
}

#[derive(Debug)]
struct Position {
    horizontal: i64,
    vertical: i64,
}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.horizontal == other.horizontal && self.vertical == other.vertical
    }
}

fn parse_input(data: &str) -> Vec<Movement> {
    data
        .split("\n")
        .map(|s| create_movement(s))
        .collect()
}

fn create_movement(data: &str) -> Movement {
    let mut data_iter = data.split_whitespace();

    let direction = match data_iter.next().as_deref() {
        Some(dir) =>
            match dir.to_uppercase().as_str() {
                    "UP" => Directions::UP,
                    "DOWN" => Directions::DOWN,
                    "FORWARD" => Directions::FORWARD,
                    "BACKWARD" => Directions::BACKWARD,
                    _ => Directions::NONE
            }
        None => Directions::NONE
    };

    let distance = match data_iter.next() {
        None => 0,
        Some(s) => match s.parse::<i64>() {
            Ok(a) => a,
            Err(_) => 0,
        }
    };

    Movement {
        direction,
        distance,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::{calculate_delta_by_aim, calculate_delta, create_movement, Directions, Movement, parse_input, Position};

    #[rstest]
    #[case("forward 0", Directions::FORWARD)]
    #[case("BACKWARD 0", Directions::BACKWARD)]
    #[case("Up 0", Directions::UP)]
    #[case("dowN 0", Directions::DOWN)]
    #[case("failcase 0", Directions::NONE)]
    fn it_converts_input_into_direction(#[case] input: &str, #[case] expected: Directions) {
        let result = create_movement(input);
        assert_eq!(result.direction, expected);
    }

    #[test]
    fn it_adds_distance_to_movement() {
        let input = "FORWARD 20";
        let result = create_movement(input);
        assert_eq!(result.distance, 20);
    }

    #[test]
    fn it_collects_input_into_vec() {
        let input = "FORWARD 20\nBACKWARD 20";
        let result = parse_input(input);
        let expected = vec![
            Movement {
                direction: Directions::FORWARD,
                distance: 20
            },
            Movement {
                direction: Directions::BACKWARD,
                distance: 20
            },
        ];
        assert_eq!(&result, &expected);
    }

    #[test]
    fn it_calculates_the_distance() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let movements = parse_input(input);
        let result = calculate_delta(&movements);
        let expected_result = Position {
            horizontal: 15,
            vertical: 10,
        };

        assert_eq!(result, expected_result);
    }


    #[test]
    fn it_calculates_the_aim() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let movements = parse_input(input);
        let result = calculate_delta_by_aim(&movements);
        let expected_result = Position {
            horizontal: 15,
            vertical: 60,
        };

        assert_eq!(result, expected_result);
    }


}
