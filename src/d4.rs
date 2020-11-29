use std::num::NonZeroU32;

struct DigitIterator {
    number: u32,
    power: u32,
    at_digit: u32,
}

impl DigitIterator {
    fn new(number: NonZeroU32) -> Self {
        let power = (number.get() as f32).log10().ceil() as u32;

        DigitIterator { number: number.get(), power, at_digit: 0 }
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

#[allow(dead_code)]
pub fn run() {
    fn always_increase(digits: &[u8]) -> bool {
        digits.windows(2).fold(true, |is_increasing, pair| is_increasing && pair[0] <= pair[1])
    }
    fn double_found(digits: &[u8]) -> bool {
        digits.windows(2).fold(false, |has_double, pair| has_double || pair[0] == pair[1])
    }
    fn double_only(digits: &[u8]) -> bool {
        digits.windows(4).fold(false, |double_only, quad| double_only || (quad[1] == quad[2] && quad[1] != quad[0] && quad[2] != quad[3]))
            || (digits[0] == digits[1] && digits[1] != digits[2])
            || (digits[digits.len() - 1] == digits[digits.len() - 2] && digits[digits.len() - 2] != digits[digits.len() - 3])
    }

    let range = 136760u32..595730;

    let mut good_passwords_a = 0;
    let mut good_passwords_b = 0;

    for i in range {
        let digits: Vec<u8> = DigitIterator::new(NonZeroU32::new(i).unwrap()).collect::<Vec<_>>();

        if always_increase(&digits) && double_found(&digits) {
            good_passwords_a += 1;
            if double_only(&digits) {
                good_passwords_b += 1;
            }
        }
    }

    println!("a: {}", good_passwords_a);
    println!("a: {}", good_passwords_b);
}
