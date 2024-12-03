use std::io::{stdin, BufRead};

struct PDA {
    current: usize,
    start: usize,
    end: usize,
    stack: Vec<i32>,
    transitions: Vec<Box<dyn Fn(&mut Vec<i32>, u8) -> usize>>,
    accept: Box<dyn Fn(&mut Vec<i32>) -> i32>,
}

impl PDA {
    fn transition(&mut self, input: u8) -> Option<i32> {
        if self.current >= self.transitions.len() {
            panic!("undefined state");
        }
        let next = (self.transitions[self.current])(&mut self.stack, input);
        if next == self.end {
            self.current = self.start;
            return Some((self.accept)(&mut self.stack));
        }
        self.current = next;
        None
    }
}

fn main() {
    let input: String = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut parser = PDA {
        current: 0,
        start: 0,
        end: 6,
        stack: vec![],
        transitions: vec![
            Box::new(|stack: &mut Vec<i32>, input: u8| match input {
                b'm' => match stack.last() {
                    Some(-1) => 0,
                    _ => 1,
                },
                b'd' => 7,
                _ => 0,
            }),
            Box::new(|_, input: u8| match input {
                b'u' => 2,
                _ => 0,
            }),
            Box::new(|_, input: u8| match input {
                b'l' => 3,
                _ => 0,
            }),
            Box::new(|_, input: u8| match input {
                b'(' => 4,
                _ => 0,
            }),
            Box::new(|stack: &mut Vec<i32>, input: u8| match input {
                b'0'..=b'9' => {
                    let mut val = stack.pop().unwrap_or(0);
                    val *= 10;
                    val += (input - b'0') as i32;
                    stack.push(val);
                    return 4;
                }
                b',' => {
                    stack.push(0);
                    return 5;
                }
                _ => {
                    stack.clear();
                    0
                }
            }),
            Box::new(|stack: &mut Vec<i32>, input: u8| match input {
                b'0'..=b'9' => {
                    let mut val = stack.pop().unwrap_or(0);
                    val *= 10;
                    val += (input - b'0') as i32;
                    stack.push(val);
                    5
                }
                b')' => 6,
                _ => {
                    stack.clear();
                    0
                }
            }),
            Box::new(|_, _| 0),
            Box::new(|_, input: u8| match input {
                b'o' => 8,
                _ => 0,
            }),
            Box::new(|_, input: u8| match input {
                b'(' => 9,
                b'n' => 10,
                _ => 0,
            }),
            Box::new(|stack: &mut Vec<i32>, input: u8| match input {
                b')' => {
                    stack.clear();
                    0
                }
                _ => 0,
            }),
            Box::new(|_, input: u8| match input {
                b'\'' => 11,
                _ => 0,
            }),
            Box::new(|_, input: u8| match input {
                b't' => 12,
                _ => 0,
            }),
            Box::new(|_, input: u8| match input {
                b'(' => 13,
                _ => 0,
            }),
            Box::new(|stack: &mut Vec<i32>, input: u8| match input {
                b')' => {
                    stack.push(-1);
                    0
                }
                _ => 0,
            }),
        ],
        accept: Box::new(|stack: &mut Vec<i32>| {
            let v1 = stack.pop().expect("expected first value");
            let v2 = stack.pop().expect("expected second value");
            return v1 * v2;
        }),
    };

    let mut ans = 0;
    for byte in input.as_bytes() {
        match parser.transition(*byte) {
            Some(v) => {
                ans += v;
            }
            _ => {}
        };
    }
    println!("{}", ans);
}
