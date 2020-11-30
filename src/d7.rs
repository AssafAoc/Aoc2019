use std::collections::VecDeque;

use permutohedron;

use crate::get_input;
use crate::intcode;
use permutohedron::LexicalPermutation;

fn a(ribbon: &Vec<i32>) -> i32 {
    let mut phase_settings = [0, 1, 2, 3, 4];
    let mut max_signal = 0;
    'lp: loop {
        let mut prev_output = 0;
        for phase in &phase_settings {
            let (_, output) = intcode::run(ribbon.clone(), Some(vec![*phase, prev_output]));
            if output.len() > 1 {
                panic!("wat?");
            }
            prev_output = output[0];
        }
        max_signal = max_signal.max(prev_output);
        if !phase_settings.next_permutation() {
            break 'lp;
        }
    };
    max_signal
}

fn b(ribbon: &Vec<i32>) -> i32 {
    let mut phase_settings = [5, 6, 7, 8, 9];
    let mut max_signal = 0;
    let (_, output) = intcode::run(ribbon.clone(), Some(vec![5, 0, 6, 7, 8, 9]));
    println!("{:?}", output);
    // 'lp: loop {
    //     let mut prev_output = 0;
    //     for phase in &phase_settings {
    //         let (_, output) = intcode::run(ribbon.clone(), Some(vec![*phase, prev_output]));
    //         if output.len() > 1 {
    //             panic!("wat?");
    //         }
    //         prev_output = output[0];
    //     }
    //     max_signal = max_signal.max(prev_output);
    //     if !phase_settings.next_permutation() {
    //         break 'lp;
    //     }
    // };
    max_signal
}

pub fn run() {
    let input = get_input(7, "");
    // let input = get_input(7, "tests");

    let base_ribbon = input[0].split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

    println!("a: {}", a(&base_ribbon));
    // println!("b: {}", b(&base_ribbon));
}