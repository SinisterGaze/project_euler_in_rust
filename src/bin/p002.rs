struct Fibonacci {
    curr: i32,
    next: i32,
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

impl Fibonacci {
    fn new(f0: i32, f1: i32) -> Fibonacci {
        Fibonacci { curr: f0, next: f1 }
    }
}

fn main() {
    let result: i32 = Fibonacci::new(1, 2)
        .take_while(|&val| val < 4_000_000)
        .filter(|&val| val & 1 == 0)
        .sum();
    println!("{result}");
}
