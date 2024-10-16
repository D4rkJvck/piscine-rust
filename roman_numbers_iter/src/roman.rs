pub use roman_numbers::RomanDigit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        let mut shadow = value;

        let mut table = match value {
            0 => vec![RomanDigit::from(0)],
            _ => Vec::new(),
        };

        while shadow > 0 {
            let round = match shadow {
                900..=999 | 400..=499 => 100,
                90..=99 | 40..=49 => 10,
                9 | 4 => 1,
                _ => 0,
            };

            if round != 0 {
                table.push(RomanDigit::from(round));
                shadow += round;
            }

            table.push(RomanDigit::from(shadow));

            shadow -= match shadow {
                1000.. => 1000,
                500..=999 => 500,
                100..=499 => 100,
                50..=99 => 50,
                10..=49 => 10,
                5..=9 => 5,
                ..=4 => 1,
            };
        }

        Self(table)
    }
}
