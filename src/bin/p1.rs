use std::io;

fn main() {
    let n: u32;
    let divisors: Vec<u32>;
    loop {
        println!("Input maximum number: ");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read line");
        n = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    loop {
        println!("Enter divisors: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        divisors = match input
            .trim()
            .split_whitespace()
            .into_iter()
            .map(|i| i.parse::<u32>())
            .collect() {
                Ok(values) => values,
                Err(_) => continue,
            };
        break;
    }
    println!("The sum of multiples of {divisors:?} under {n} is: {}", sum_of_multiples(n, &divisors));
}

fn sum_of_multiples(max_number: u32, divisors: &[u32]) -> u32 {
    let mut result: u32 = 0;
    for n in 0..max_number {
        for d in divisors.iter() {
            if n % d == 0 {
                result += n;
                break;
            }
        }
    }
    result
}
