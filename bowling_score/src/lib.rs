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
        if self.complete {
            return Err(Error::GameComplete)
        }

        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft)
        }

        match pins {
            // Fill Balls______________________________
                p if self.frame == 9 => {
                if self.extra == 0 {
                    self.score[self.frame].1 += p;
                    
                    if self.bonus > 0 {
                        while self.bonus > 2 {
                            self.score[self.frame].1 += p;
                            self.bonus -= 1;
                        }

                        self.score[self.frame].1 += p;
                        self.bonus -= 1;
                    };

                    self.complete = true;
                } else {
                    self.score[self.frame].1 += p;
                    
                    if self.bonus > 0 {
                        while self.bonus > 2 {
                            self.score[self.frame].1 += p;
                            self.bonus -= 1;
                        }
                        
                        self.score[self.frame].1 += p;
                        self.bonus -= 1;
                    };
                    
                    self.extra -= 1;
                }
            }

            // Strike_____________________________________________________________
            10 if self.roll == 1 => {
                self.score[self.frame].0 += pins;

                if self.bonus > 0 {
                    while self.bonus > 2 {
                        self.score[self.frame].0 += pins;
                        self.bonus -= 1;
                    }

                    self.score[self.frame].0 += pins;
                    self.bonus -= 1;
                };

                self.bonus += 2;
                self.frame += 1;

                if self.frame == 9 {
                    self.extra = 2
                };
            }

            // Not Strike___________________________________________
             p @ 0..=9 if self.roll == 1 => {
                self.score[self.frame].0 += p;

                if self.bonus > 0 {
                    while self.bonus > 2 {
                        self.score[self.frame].0 += p;
                        self.bonus -= 1;
                    }

                    self.score[self.frame].0 += p;
                    self.bonus -= 1;
                };

                self.pins -= p;
                self.roll += 1;
            }

            // Spare or Open________________________________________________
            p @ 0..=10 if self.roll == 2 => {
                self.score[self.frame].1 += p;

                if self.bonus > 0 {
                    while self.bonus > 2 {
                        self.score[self.frame].1 += p;
                        self.bonus -= 1;
                    }

                    self.score[self.frame].1 += p;
                    self.bonus -= 1;
                };

                if self.pins == p {
                    self.bonus += 1
                };

                if self.pins == p && self.frame == 9 {
                    self.extra = 1
                };

                self.pins = 10;
                self.roll -= 1;
                self.frame += 1;
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
    fn score_roll_1(&mut self, p: u16,) {
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
