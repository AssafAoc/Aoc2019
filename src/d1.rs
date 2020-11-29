fn calc_fuel(mass: u32) -> Option<u32> {
    ((mass as f32 / 3.0) as u32).checked_sub(2)
}

fn calc_fuel_fuel(fuel: u32) -> u32 {
    let mut total_fuel = fuel;
    let mut current_fuel = fuel;
    while let Some(more_fuel) = calc_fuel(current_fuel) {
        total_fuel += more_fuel;
        current_fuel = more_fuel;
    }
    total_fuel
}

#[allow(dead_code)]
pub fn run() {
    let input = super::get_input(1, "");

    let fuel_mass = input.iter()
        .map(|line| line.parse::<u32>().unwrap())
        .map(calc_fuel).map(|f| f.unwrap())
        .sum::<u32>();

    let fuel_total_mass = input.iter()
        .map(|line| line.parse::<u32>().unwrap())
        .map(calc_fuel)
        .map(|f| f.unwrap())
        .map(calc_fuel_fuel)
        .sum::<u32>();

    println!("a: {}", fuel_mass);
    println!("b: {}", fuel_total_mass);
}
