#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    score: Vec<(u16, u16)>,
    frame: usize,
    pins: u16,
    roll: usize,
    complete: bool,
    bonus: u16,
    extra: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            score: vec![(0, 0); 10],
            frame: 0,
            pins: 10,
            roll: 1,
            complete: false,
            bonus: 0,
            extra: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match (pins, self) {
            (_, Self { complete: true, .. }) => return Err(Error::GameComplete),
            (p, Self { pins, .. }) if p > *pins => return Err(Error::NotEnoughPinsLeft),

            // Fill Balls______________________________
            (
                p,
                Self {
                    extra,
                    frame,
                    score,
                    bonus,
                    complete,
                    ..
                },
            ) if *frame == 9 => {
                if *extra == 0 {
                    score[*frame].1 += p;

                    if *bonus > 0 {
                        while *bonus > 2 {
                            score[*frame].1 += p;
                            *bonus -= 1;
                        }

                        score[*frame].1 += p;
                        *bonus -= 1;
                    };

                    *complete = true;
                } else {
                    score[*frame].1 += p;

                    if *bonus > 0 {
                        while *bonus > 2 {
                            score[*frame].1 += p;
                            *bonus -= 1;
                        }

                        score[*frame].1 += p;
                        *bonus -= 1;
                    };

                    *extra -= 1;
                }
            }

            // Strike_____________________________________________________________
            (
                10,
                Self {
                    frame,
                    roll: 1,
                    bonus,
                    score,
                    extra,
                    ..
                },
            ) => {
                score[*frame].0 += pins;

                if *bonus > 0 {
                    while *bonus > 2 {
                        score[*frame].0 += pins;
                        *bonus -= 1;
                    }

                    score[*frame].0 += pins;
                    *bonus -= 1;
                };

                *bonus += 2;
                *frame += 1;

                if *frame == 9 {
                    *extra = 2
                };
            }

            // Not Strike___________________________________________
            (
                p @ 0..=9,
                Self {
                    frame,
                    roll,
                    pins,
                    score,
                    bonus,
                    ..
                },
            ) if *roll == 1 => {
                score[*frame].0 += p;

                if *bonus > 0 {
                    while *bonus > 2 {
                        score[*frame].0 += p;
                        *bonus -= 1;
                    }

                    score[*frame].0 += p;
                    *bonus -= 1;
                };

                *pins -= p;
                *roll += 1;
            }

            // Spare or Open________________________________________________
            (
                p @ 0..=10,
                Self {
                    frame,
                    roll,
                    score,
                    pins,
                    bonus,
                    extra,
                    ..
                },
            ) if *roll == 2 => {
                score[*frame].1 += p;

                if *bonus > 0 {
                    while *bonus > 2 {
                        score[*frame].1 += p;
                        *bonus -= 1;
                    }

                    score[*frame].1 += p;
                    *bonus -= 1;
                };

                if *pins == p {
                    *bonus += 1
                };

                if *pins == p && *frame == 9 {
                    *extra = 1
                };

                *pins = 10;
                *roll -= 1;
                *frame += 1;
            }

            _ => {}
        };

        Ok(())
    }

    //----------------------------------------------------------------
    
    pub fn score(&mut self) -> Option<u16> {
        if !self.complete {
            return None;
        };
        
        Some(self.score.iter().map(|f| f.0 + f.1).sum())
    }
    
    //----------------------------------------

    #[allow(unused)]
    fn frame_score(&mut self, p: u16) {
        self.score[self.frame].1 += p;

        if self.bonus > 0 {
            while self.bonus > 2 {
                self.score[self.frame].1 += p;
                self.bonus -= 1;
            }

            self.score[self.frame].1 += p;
            self.bonus -= 1;
        };
    }
}
