#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

//________________________________________________________________
//

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

impl Member {
    pub fn new(n: &str, role: Role, age: u8) -> Self {
        let name = n.to_owned();
        Member { name, role, age }
    }

    pub fn get_promotion(&mut self) {
        use Role::*;

        self.role = match self.role {
            Associate => Soldier,
            Soldier => Caporegime,
            Caporegime => Underboss,
            Underboss => Underboss,
        }
    }
}
