use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Move {
    R(usize),
    L(usize),
    D(usize),
    U(usize),
}

impl FromStr for Move {
    type Err = String;
    
    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        use Move::*;
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
pub struct Moves(Vec<Move>);

impl FromStr for Moves {
    type Err = String;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        input.split(',')
            .map(|single| single.parse())
            .collect::<Result<Vec<_>,_>>()
            .map(Moves)
    }

}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
struct Position(isize, isize);

impl Position {
    fn apply(self, m: Move) -> Self {
        use Move::*;
        match m {
            R(l) => Self(self.0 + l as isize, self.1),
            L(l) => Self(self.0 - l as isize, self.1),
            U(l) => Self(self.0, self.1 + l as isize),
            D(l) => Self(self.0, self.1 - l as isize),
        }
    }

    fn manathan_distance(self, other: Self) -> usize {
        (self.0 - other.0).abs() as usize + (self.1 - other.1).abs() as usize
    } 

}

type Segment = (Position, Move);
type PathInner = Vec<(Position, Move)>;

#[derive(Debug, PartialEq, Default)]
pub struct Path(Vec<(Position, Move)>);

impl From<PathInner> for Path {
    fn from(inner: PathInner) -> Self { Self(inner) }
}

impl Path {
    fn from_moves(moves: Moves, start: Position) -> Self {
        moves.0.into_iter()
            .scan(start, |position, m| {
                let p = *position;
                *position = p.apply(m);
                
                Some((p, m))
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<Moves> for Path {
    fn from(moves: Moves) -> Self { 
        Self::from_moves(moves, Default::default())
     }
}

fn inside(segment: &Segment, p: Position) -> bool {
    let start = segment.0;
    let end = start.apply(segment.1);

    use Move::*;
 
    match segment.1 {
        R(_) => p.0>=start.0 && p.0<=end.0 && p.1 == start.1 && p.1 == end.1,
        L(_) => p.0<=start.0 && p.0>=end.0 && p.1 == start.1 && p.1 == end.1,
        U(_) => p.0==start.0 && p.0<=end.0 && p.1 >= start.1 && p.1 <= end.1,
        D(_) => p.0==start.0 && p.0<=end.0 && p.1 <= start.1 && p.1 >= end.1,
    }
}

fn cross(a: &Segment, b: &Segment) -> Option<Position> {
    use Move::*;

    let candidate= match a.1 {
        R(_) | L(_) => Position((b.0).0, (a.0).1),
        D(_) | U(_) => Position((a.0).0, (b.0).1),
    };

    if inside(a, candidate) && inside(b, candidate) {
        Some(candidate)
    } else {
        None
    }
}

pub fn minimum_cross_distance(first: Moves, second: Moves) -> Option<usize> {
    let first = Path::from(first);
    let second = Path::from(second);
    let mut distances = Vec::new();

    for segment1 in &first.0[1..] {
        for segment2 in &second.0[1..] {
            if let Some(p) = cross(segment1, segment2) {
                let distance = dbg!(dbg!(p).manathan_distance(Default::default()));
                distances.push(distance)

            }
        }
    }

    distances.into_iter().min()
}

#[cfg(test)]
mod should {
    use super::*;
    use Move::*;
    use rstest::rstest;

    #[rstest(
        input, expected,
        case("R75,D30,L12,U7", Moves(vec![R(75),D(30),L(12),U(7)]))
    )]
    fn parse_input_string(input: &str, expected: Moves) {
        assert_eq!(expected, input.parse::<Moves>().unwrap());
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
        input.parse::<Moves>().unwrap();
    }

    #[rstest(
        first, second, expected,
        case("R8,U5,L5,D3", "U7,R6,D4,L4", 6),
        case("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83", 159),
        case("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 135),
    )]
    fn return_minimum_distance(first: &str, second: &str, expected: usize) {
        let distance = minimum_cross_distance(first.parse().unwrap(), second.parse().unwrap()).unwrap();

        assert_eq!(distance, expected);
    }
}