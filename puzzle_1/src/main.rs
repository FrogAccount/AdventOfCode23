use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let cooler_input: Vec<&str> = input.split('\n').collect();

    let mut sum: i32 = 0;
    for line in cooler_input {
        println!("{}", line);
        for c1 in line.chars() {
            if c1.is_numeric() {
                for c2 in line.chars().rev() {
                    if c2.is_numeric() {
                        sum += format!("{c1}{c2}").parse::<i32>().unwrap();
                        break;
                    }
                }
                break;
            }
        }
    }
    print!("Sum of all calibration values: {}", sum)
}
