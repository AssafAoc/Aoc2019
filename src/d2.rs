fn change_and_run(ribbon: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut ribbon_run = ribbon.clone();
    ribbon_run[1] = noun;
    ribbon_run[2] = verb;
    let (new_ribbon, _) = crate::intcode::run(ribbon_run, None);
    new_ribbon[0]
}

fn b(ribbon: &Vec<i32>) -> Option<(i32, i32)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if change_and_run(&ribbon, noun, verb) == 19690720 {
                return Some((noun, verb))
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn run() {
    let input = super::get_input(2, "");

    let ribbon = input[0].split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();

    println!("a: {}", change_and_run(&ribbon, 12, 2));

    let (noun, verb) = b(&ribbon).unwrap();
    println!("b: {}", 100 * noun + verb);
}
