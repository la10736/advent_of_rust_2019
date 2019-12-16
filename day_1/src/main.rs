use std::{fs, env};

fn mass_to_fuel(mass: u32) -> u32 {
    (mass/3) - 2
}

fn fuel_for_ship(mass: u32) -> u32 {
    if mass <= 6 {
        return 0;
    }
    let m = mass_to_fuel(mass);
    m + fuel_for_ship(m)
}

pub fn main() {
    let path = env::args().nth(1).expect("Input file name");
    let total = env::args().nth(2).is_some();
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    
    let fuel: u32 = contents.lines()
            .map(|l| l.parse().expect(&format!("A valid unsigned integer: '{}'", l)))
            .map(|m| if total { fuel_for_ship(m) } else { mass_to_fuel(m) })
            .sum();
    println!("Total fuel = {}", fuel);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(mass, expected_fuel,
        case(12, 2),
        case(14, 2),
        case(1969, 654),
        case(100756, 33583),
    )]
    fn should_return_fuel_ammount(mass: u32, expected_fuel: u32) {
        assert_eq!(mass_to_fuel(mass), expected_fuel);
    }

    #[rstest(mass, expected_fuel,
        case(14, 2),
        case(1969, 966),
        case(100756, 50346),
    )]
    fn should_return_fuel_ammount_for_trip(mass: u32, expected_fuel: u32) {
        assert_eq!(fuel_for_ship(mass), expected_fuel);
    }
}
