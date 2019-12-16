use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
enum Directions {
    R(usize),
    L(usize),
    D(usize),
    U(usize),
}

impl FromStr for Directions {
    type Err = String;
    
    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        use Directions::*;
        match input.chars().nth(0) {
            Some('R') => input[1..].parse().map(R).map_err(|e| format!("{}", e)),
            Some('D') => input[1..].parse().map(D).map_err(|e| format!("{}", e)),
            Some('U') => input[1..].parse().map(U).map_err(|e| format!("{}", e)),
            Some('L') => input[1..].parse().map(L).map_err(|e| format!("{}", e)),
            None => Err(format!("Cannot parse empty string")),
            _ => Err(format!("Cannot understand '{}'", input)),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Path(Vec<Directions>);

impl FromStr for Path {
    type Err = String;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        input.split(',')
            .map(|single| single.parse())
            .collect::<Result<Vec<_>,_>>()
            .map(Path)
    }

}

#[cfg(test)]
mod should {
    use super::*;
    use Directions::*;
    use rstest::rstest;

    #[rstest(
        input, expected,
        case("R75,D30,L12,U7", Path(vec![R(75),D(30),L(12),U(7)]))
    )]
    fn parse_input_string(input: &str, expected: Path) {
        assert_eq!(expected, input.parse::<Path>().unwrap());
    }

    #[rstest(
        input,
        case(""),
        case("Pippo"),
        case("U-10"),
        case("Uciao"),
        case("D10,U,L10"),
    )]
    #[should_panic]
    fn parse_invalid_input(input: &str) {
        input.parse::<Path>().unwrap();
    }
}