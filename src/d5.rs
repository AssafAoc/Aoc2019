#[allow(dead_code)]
pub fn run() {
    let input = super::get_input(5, "");
    // let input = super::get_input(5, "tests");

    let ribbon = input[0].split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let (_, output) = crate::intcode::run(ribbon, Some(vec![1]));
    println!("a: {:?}", output);

    let ribbon = input[0].split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let (_, output) = crate::intcode::run(ribbon, Some(vec![5]));
    println!("b: {:?}", output);
}
