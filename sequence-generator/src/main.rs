trait Sequence {
    fn new() -> Self;

    fn n(&self) -> u128;
    fn generate(&mut self) -> Option<u128>;
    fn reset(&mut self);
}

struct Fibonacci {
    n1: u128,
    n2: u128,
    index: u128,
}

impl Sequence for Fibonacci {
    fn new() -> Self {
        Fibonacci { n1: 0, n2: 1, index: 0 }
    }

    fn n(&self) -> u128 {
        self.index
    }

    fn generate(&mut self) -> Option<u128> {
        let res = self.n1;
        (self.n1, self.n2) = (self.n2, self.n1 + self.n2);
        self.index += 1;
        Some(res)
    }

    fn reset(&mut self) {
        *self = Fibonacci::new();
    }
}

struct Factorial {
    current: u128,
    num: u128,
    index: u128,
}

impl Sequence for Factorial {
    fn new() -> Self {
        Factorial {current:1, num: 1, index: 0}
    }

    fn n(&self) -> u128 {
        self.index
    }

    fn generate(&mut self) -> Option<u128> {
        let res = self.current;
        self.num += 1;
        self.current = self.current * self.num;
        self.index += 1;
        Some(res)
    }

    fn reset(&mut self) {
        *self = Factorial::new();
    }
}

struct PowersOfTwo {
    index: u128,
}

impl Sequence for PowersOfTwo {
    fn new() -> Self {
        PowersOfTwo {index: 0}
    }

    fn n(&self) -> u128 {
        self.index
    }

    fn generate(&mut self) -> Option<u128> {
        let i = self.index as u32;
        self.index += 1;
        Some(2_i32.pow(i) as u128)
    }

    fn reset(&mut self) {
        *self = PowersOfTwo::new()
    }
}

struct TriangularNums {
    current: u128,
    num: u128,
    index: u128,
}

impl Sequence for TriangularNums {
    fn new() -> Self {
        TriangularNums {current: 1, num: 1, index: 0}
    }

    fn n(&self) -> u128 {
        self.index
    }

    fn generate(&mut self) -> Option<u128> {
        let res = self.current;
        self.index += 1;
        self.num += 1;
        self.current += self.num;
        Some(res)
    }

    fn reset(&mut self) {
        *self = TriangularNums::new()
    }
}

fn main() {
    println!("Hello, world!");

    let mut fibonacci = Fibonacci::new();
    let x = first_five(&mut fibonacci);
    println!("{} {} {} {} {}", x.0, x.1, x.2, x.3, x.4);

    let mut factorial = Factorial::new();
    let x = first_five(&mut factorial);
    println!("{} {} {} {} {}", x.0, x.1, x.2, x.3, x.4);

    let mut powers_of_two = PowersOfTwo::new();
    let x = first_five(&mut powers_of_two);
    println!("{} {} {} {} {}", x.0, x.1, x.2, x.3, x.4);

    let mut triangular_nums = TriangularNums::new();
    let x = first_five(&mut triangular_nums);
    println!("{} {} {} {} {}", x.0, x.1, x.2, x.3, x.4);
}

fn first_five<S>(s: &mut S) -> (u128, u128, u128, u128, u128)
    where
        S: Sequence
{
    if let (Some(first), Some(second), Some(third), Some(fourth), Some(fifth)) = (s.generate(), s.generate(), s.generate(), s.generate(), s.generate()) {
        (first, second, third, fourth, fifth)
    } else {
        panic!("All of the sequences should be able to produce five elements")
    }
}
