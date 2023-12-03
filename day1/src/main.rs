fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let input = std::fs::read_to_string(&args[1]).unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let code = parse_line(line);
        println!("{}: {}", line, code);
        sum += code;
    }
    println!("Sum: {}", sum);
}

#[derive(Copy, Clone)]
struct Token(&'static str, i32);
static TOKENS: [Token; 18] = [
    // Token("0", 0),
    Token("1", 1),
    Token("2", 2),
    Token("3", 3),
    Token("4", 4),
    Token("5", 5),
    Token("6", 6),
    Token("7", 7),
    Token("8", 8),
    Token("9", 9),
    // Token("zero", 0),
    Token("one", 1),
    Token("two", 2),
    Token("three", 3),
    Token("four", 4),
    Token("five", 5),
    Token("six", 6),
    Token("seven", 7),
    Token("eight", 8),
    Token("nine", 9),
];

struct TokenReader {
    line: String,
    index: usize,
}

impl TokenReader {
    fn new(line: &str) -> TokenReader {
        TokenReader {
            line: line.to_string(),
            index: 0,
        }
    }

    fn token_at_index(&self, index: usize) -> Option<Token> {
        if let Some(substring) = self.line.get(index..) {
            for token in TOKENS {
                if substring.starts_with(token.0) {
                    return Some(token);
                }
            }
        }
        None
    }
}

impl Iterator for TokenReader {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.line.len() {
            if let Some(token) = self.token_at_index(self.index) {
                // we only increase by 1 since there could be overlap of letters
                // eg oneight -> one, eight
                self.index += 1;
                return Some(token);
            }
            self.index += 1;
        }
        None
    }
}

fn parse_line(line: &str) -> i32 {
    let mut first = None;
    let mut last = None;

    let reader = TokenReader::new(line);
    for token in reader {
        if first.is_none() {
            first = Some(token.1);
        }
        last = Some(token.1);
    }

    let code = first.expect("no first digit") * 10 + last.expect("no last digit");

    return code;
}
