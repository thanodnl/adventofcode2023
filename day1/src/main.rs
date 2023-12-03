fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let input = std::fs::read_to_string(&args[1]).unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let code = parse_line(line);
        sum += code;
    }

    println!("Sum: {}", sum);
}

fn parse_line(line: &str) -> i32 {
    let mut first = None;
    let mut last = None;

    for c in line.chars() {
        if !c.is_numeric() {
            continue;
        }

        if first.is_none() {
            first = Some(c);
        }
        last = Some(c);
    }

    let code = first.expect("No first digit").to_string()
        + last.expect("No last digit").to_string().as_str();

    return code.parse::<i32>().unwrap();
}
