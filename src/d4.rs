use std::num::NonZeroU32;

struct DigitIterator {
    number: u32,
    power: u32,
    at_digit: u32,
}

impl DigitIterator {
    fn new(number: u32/*NonZeroU32*/) -> Self {
        let power = (number as f32).log10().ceil() as u32;

        DigitIterator { number, power, at_digit: 0 }
    }
}

impl Iterator for DigitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_digit == self.power {
            None
        } else {
            let digit = self.number / (10u32.pow(self.power - 1 - self.at_digit)) % 10;
            self.at_digit += 1;
            Some(digit as u8)
        }
    }
}

pub fn run() {
    let range = 136760u32..595730;

    let mut good_passwords = 0;

    for i in range {
        let mut it = DigitIterator::new(i).peekable();

        let mut always_increase = true;
        let mut double_found = false;
        'inner: while let Some(d) = it.next() {
            if let Some(next_d) = it.peek() {
                always_increase = always_increase && d <= *next_d;
                double_found = double_found || d == *next_d
            } else {
                break 'inner
            }
        }
        if always_increase && double_found {
            good_passwords += 1;
        }
    }

    println!("a: {}", good_passwords);
}
