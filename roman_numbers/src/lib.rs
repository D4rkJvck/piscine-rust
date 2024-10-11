//  ╔═══════════════════════════════════════════════════════════════════════════════╗
//  ║                          ● Library: Roman Numbers ●                           ║
//  ║                                                                               ║
//  ║                                • DESCRIPTION •                                ║
//  ║                                                                               ║
//  ╚═══════════════════════════════════════════════════════════════════════════════╝

use std::vec;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            v if v >= 1000 => Self::M,
            v if v >= 500 => Self::D,
            v if v >= 100 => Self::C,
            v if v >= 50 => Self::L,
            v if v >= 10 => Self::X,
            v if v >= 5 => Self::V,
            v if v >= 1 => Self::I,
            _ => Self::Nulla,
        }
    }
}

//________________________________________________________________
//

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
            shadow = match shadow {
                v if v / 900 > 0 || v / 400 > 0 => {
                    table.push(RomanDigit::from(100));
                    v + 100
                }
                v if v / 90 > 0 || v / 40 > 0 => {
                    table.push(RomanDigit::from(10));
                    v + 10
                }
                9 | 4 => {
                    table.push(RomanDigit::from(1));
                    shadow + 1
                }
                v => v,
            };

            table.push(RomanDigit::from(shadow));

            shadow = match shadow {
                v if v / 1000 > 0 => v - 1000,
                v if v / 500 > 0 => v - 500,
                v if v / 100 > 0 => v - 100,
                v if v / 50 > 0 => v - 50,
                v if v / 10 > 0 => v - 10,
                v if v / 5 > 0 => v - 5,
                v => v - 1,
            };
        }

        Self(table)
    }
}
