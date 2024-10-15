use chrono::Duration;
use colored::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    Top,
    Center,
    Bottom,
}

// impl Display for Position {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Position::Top => write!(f, "Top"),
//             Position::Center => write!(f, "Center"),
//             Position::Bottom => write!(f, "Bottom"),
//         }
//     }
// }

//__________________________________________
//

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Self::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: format!("{}", text),
            },
            Self::Registration(duration) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!("You have {} left before the registration ends", duration),
            },
            Self::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: format!("{}", text),
            },
            Self::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: format!("Enjoy your holiday"),
            },
        }
    }
}

//_______________________________________________________________________
//

#[derive(Debug, PartialEq, Eq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

impl Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b) = self.color;
        
        write!(
            f,
            "({:?}, {}, {})",
            self.position, self.size, self.content.truecolor(r, g, b)
        )
    }
}
