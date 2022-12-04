use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];

    let string_in = fs::read_to_string(fname).unwrap();
    let string_split: Vec<&str> = string_in.split("\n").collect();

    let mut count = 0;
    let mut counts: Vec<u32> = Vec::new();

    for line in string_split.iter() {
        match line.parse::<u32>() {
            Ok(x)   => { count += x; },
            Err(_)  => {
                counts.push(count);
                count = 0;
            }
        }
    }

    counts.sort_by(|a, b| b.cmp(a));

    println!("Max is {}", counts[0]);
    let top_3 = counts[0..3].to_vec();
    println!("Top 3 combined are {}", top_3.iter().sum::<u32>());
}
