fn operate(ribbon: &mut [i32], position: usize) -> Option<usize> {
    match ribbon[position] {
        1 => {
            let lhs = ribbon[ribbon[position + 1] as usize];
            let rhs = ribbon[ribbon[position + 2] as usize];
            ribbon[ribbon[position + 3] as usize] = lhs + rhs;
            Some(position + 4)
        }
        2 => {
            let lhs = ribbon[ribbon[position + 1] as usize];
            let rhs = ribbon[ribbon[position + 2] as usize];
            ribbon[ribbon[position + 3] as usize] = lhs * rhs;
            Some(position + 4)
        }
        99 => None,
        _ => panic!("nope")
    }
}

pub fn run(mut ribbon: Vec<i32>) -> Vec<i32> {
    let mut position = 0;
    while let Some(new_position) = operate(&mut ribbon, position) {
        position = new_position;
    }
    ribbon
}
