#[derive(Debug, Copy, Clone)]
pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl FizzBuzz {
    pub fn from_number(n: u32) -> FizzBuzz {
        use FizzBuzz::*;
        match (n % 3 == 0, n % 5 == 0) {
            (true, false) => Fizz,
            (false, true) => Buzz,
            (true, true) => FizzBuzz,
            _ => Number(n),
        }
    }
}

fn main() {
    (1..100)
        .map(FizzBuzz::from_number)
        .for_each(|f| println!("{:?}", f));
}
