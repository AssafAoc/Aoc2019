enum OpMode {
    Position,
    Immediate,
}

fn operate(ribbon: &mut [i32], position: usize, input: &mut impl Iterator<Item=i32>, output: &mut Vec<i32>) -> Option<usize> {
    fn get(ribbon_: &[i32], position_: usize, mode_: OpMode) -> i32 {
        match mode_ {
            OpMode::Position => ribbon_[ribbon_[position_] as usize],
            OpMode::Immediate => ribbon_[position_],
        }
    }

    let mode_op = ribbon[position];
    let op = mode_op % 100;
    let mode_1 = if mode_op / 100 % 10 == 0 { OpMode::Position } else { OpMode::Immediate };
    let mode_2 = if mode_op / 1_000 % 10 == 0 { OpMode::Position } else { OpMode::Immediate };
    let mode_3 = if mode_op / 10_000 % 10 == 0 { OpMode::Position } else { OpMode::Immediate };

    match op {
        1 => { // ADD
            let lhs = get(ribbon, position as usize + 1, mode_1);
            let rhs = get(ribbon, position as usize + 2, mode_2);
            if let OpMode::Immediate = mode_3 {
                panic!("set cannot be used with immediate");
            }
            ribbon[ribbon[position + 3] as usize] = lhs + rhs;
            Some(position + 4)
        }
        2 => { // MUL
            let lhs = get(ribbon, position as usize + 1, mode_1);
            let rhs = get(ribbon, position as usize + 2, mode_2);
            if let OpMode::Immediate = mode_3 {
                panic!("set cannot be used with immediate");
            }
            ribbon[ribbon[position + 3] as usize] = lhs * rhs;
            Some(position + 4)
        }
        3 => { // INPUT
            let i = input.next().expect("out of input");
            if let OpMode::Immediate = mode_1 {
                panic!("set cannot be used with immediate");
            }
            ribbon[ribbon[position + 1] as usize] = i;
            Some(position + 2)
        }
        4 => { // OUTPUT
            let o = get(ribbon, position+1, mode_1);
            output.push(o);
            Some(position + 2)
        },
        5 => { // JUMP TRUE
            let b = get(ribbon, position + 1, mode_1);
            if b != 0 {
                let new_pos = get(ribbon, position + 2, mode_2);
                Some(new_pos as usize)
            } else {
                Some(position + 3)
            }
        },
        6 => { // JUMP FALSE
            let b = get(ribbon, position + 1, mode_1);
            if b == 0 {
                let new_pos = get(ribbon, position + 2, mode_2);
                Some(new_pos as usize)
            } else {
                Some(position + 3)
            }
        },
        7 => { // LESS THAN
            let lhs = get(ribbon, position + 1, mode_1);
            let rhs = get(ribbon, position + 2, mode_2);
            if lhs < rhs {
                ribbon[ribbon[position + 3] as usize] = 1
            } else {
                ribbon[ribbon[position + 3] as usize] = 0
            }
            Some(position + 4)
        },
        8 => { // EQUAL
            let lhs = get(ribbon, position + 1, mode_1);
            let rhs = get(ribbon, position + 2, mode_2);
            if lhs == rhs {
                ribbon[ribbon[position + 3] as usize] = 1
            } else {
                ribbon[ribbon[position + 3] as usize] = 0
            }
            Some(position + 4)
        },
        99 => None,
        _ => panic!("nope")
    }
}

pub type RibbonAndOutput = (Vec<i32>, Vec<i32>);

pub fn run(mut ribbon: Vec<i32>, input: Option<Vec<i32>>) -> RibbonAndOutput {
    let mut position = 0;

    let mut output = Vec::new();
    let input = input.unwrap_or(Vec::new());

    let mut it = input.into_iter();

    while let Some(new_position) = operate(&mut ribbon, position, &mut it, &mut output) {
        position = new_position;
    }
    (ribbon, output)
}
