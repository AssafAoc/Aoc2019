use std::collections::HashMap;
use std::collections::HashSet;

fn a(input: &[String]) -> u32 {
    let mut visited = HashSet::new();
    let mut current = (0i32, 0i32);

    for mov in input[0].split(',') {
        let mut chars = mov.chars();
        let direction = chars.next().unwrap();
        let incrementor = match direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!(format!("unknown direction: {}", direction))
        };
        let size = chars.as_str().parse::<i32>().unwrap();
        for _ in 0..size {
            current = ((current.0 + incrementor.0), (current.1 + incrementor.1));
            visited.insert(current);
        }
    }

    let mut collisions = Vec::new();
    current = (0, 0);

    for mov in input[1].split(',') {
        let mut chars = mov.chars();
        let direction = chars.next().unwrap();
        let incrementor = match direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!(format!("unknown direction: {}", direction))
        };
        let size = chars.as_str().parse::<i32>().unwrap();
        for _ in 0..size {
            current = ((current.0 + incrementor.0), (current.1 + incrementor.1));
            if visited.contains(&current) {
                collisions.push(current);
            }
        }
    }

    collisions.sort_by_key(|(x, y)| x.abs() + y.abs());
    collisions[0].0.abs() as u32 + collisions[0].1.abs() as u32
}

fn b(input: &[String]) -> u32 {
    let mut visited = HashSet::new();
    let mut current = (0, 0);

    for mov in input[0].split(',') {
        let mut chars = mov.chars();
        let direction = chars.next().unwrap();
        let incrementor = match direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!(format!("unknown direction: {}", direction))
        };
        let size = chars.as_str().parse::<i32>().unwrap();
        for _ in 0..size {
            current = ((current.0 + incrementor.0), (current.1 + incrementor.1));
            visited.insert(current);
        }
    }

    let mut collisions = HashMap::new();
    current = (0, 0);
    let mut steps = 0u32;

    for mov in input[1].split(',') {
        let mut chars = mov.chars();
        let direction = chars.next().unwrap();
        let incrementor = match direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!(format!("unknown direction: {}", direction))
        };
        let size = chars.as_str().parse::<i32>().unwrap();
        for _ in 0..size {
            steps += 1;
            current = ((current.0 + incrementor.0), (current.1 + incrementor.1));
            if visited.contains(&current) {
                collisions.insert(current, steps);
            }
        }
    }

    let mut total_steps_to_collision = Vec::new();

    steps = 0;
    current = (0, 0);
    for mov in input[0].split(',') {
        let mut chars = mov.chars();
        let direction = chars.next().unwrap();
        let incrementor = match direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!(format!("unknown direction: {}", direction))
        };
        let size = chars.as_str().parse::<i32>().unwrap();
        for _ in 0..size {
            current = ((current.0 + incrementor.0), (current.1 + incrementor.1));
            steps += 1;
            if collisions.contains_key(&current) {
                total_steps_to_collision.push(collisions[&current] + steps)
            }
        }
    }

    *total_steps_to_collision.iter().min().unwrap()
}

#[allow(dead_code)]
pub fn run() {
    let input = super::get_input(3, "");

    println!("a: {}", a(&input));
    println!("b: {}", b(&input));
}
