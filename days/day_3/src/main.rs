struct InputState {
    input: String,
    idx: usize,
    enabled: bool,
}

impl Iterator for InputState {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let idx = self.idx;
            self.idx = idx + 1;

            if idx + 3 >= self.input.len() {
                break;
            }
            match (
                self.enabled,
                self.input.as_bytes()[idx],
                self.input.as_bytes()[idx + 1],
                self.input.as_bytes()[idx + 2],
            ) {
                (true, b'm', b'u', b'l') => {
                    if let Some((a, b)) = InputState::parse_parenthesis(&self.input[self.idx + 2..])
                    {
                        println!("Wawa: ({a}, {b})");
                        return Some(a * b);
                    }
                }
                (_, b'd', b'o', _) => {
                    if idx + 5 < self.input.len() && self.input[idx + 2..idx + 5] == *"n't" {
                        self.enabled = false;
                        self.idx += 2;
                        continue;
                    }
                    self.enabled = true;
                }
                _ => {}
            }
        }
        None
    }
}

impl InputState {
    fn new(input: String) -> Self {
        assert!(input.len() >= 3);

        Self {
            input,
            idx: 0,
            enabled: true,
        }
    }

    fn parse_parenthesis(input: &str) -> Option<(i32, i32)> {
        let mut input = input.chars();

        if !matches!(input.next(), Some(c) if c == '(' ) {
            return None;
        }

        let num1: i32;
        let num2: i32;

        let mut num = String::new();

        for c in &mut input {
            if c == ',' {
                break;
            }

            if !c.is_alphanumeric() {
                return None;
            }

            num.push(c);
        }

        if let Ok(n) = num.parse() {
            num1 = n;
        } else {
            return None;
        }
        num.clear();

        for c in input {
            if c == ')' {
                break;
            }

            if !c.is_alphanumeric() {
                return None;
            }

            num.push(c);
        }

        if let Ok(n) = num.parse() {
            num2 = n;
        } else {
            return None;
        }

        Some((num1, num2))
    }
}

fn main() {
    let input = common::read_input();

    let state = InputState::new(input);

    let mut sum = 0;

    for mul in state {
        sum += mul;
    }

    println!("Sum: {sum}");
}
