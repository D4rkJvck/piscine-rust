pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nbr_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nbr_games: u16) -> Box<Self> {
        let p1 = (String::from(p1_name), 0);
        let p2 = (String::from(p2_name), 0);

        Box::new(GameSession {
            id,
            p1,
            p2,
            nbr_games,
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        match (
            (self.p1.0.to_string(), self.p1.1),
            (self.p2.0.to_string(), self.p2.1),
        ) {
            ((p1_name, p1_score), (_, p2_score)) if p1_score > p2_score => (p1_name, p1_score),
            ((_, p1_score), (p2_name, p2_score)) if p1_score < p2_score => (p2_name, p2_score),
            _ => (String::from("Same score! tied"), self.p1.1),
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        if self.nbr_games == 0 {
            return ()
        }

        if user_name == self.p1.0 {
            self.p1.1 += 1
        } else if user_name == self.p2.0 {
            self.p2.1 += 1
        } else {
            ()
        }

        self.nbr_games -= 1;
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
