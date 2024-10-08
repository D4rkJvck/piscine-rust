pub mod boss;
pub mod member;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: boss::Boss,
    pub members: Vec<member::Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members
            .push(member::Member::new(name, member::Role::Associate, age));
    }

    pub fn attack(&mut self, target: &mut Self) {
        let atk = power_combat_score(self);
        let def = power_combat_score(&target);

        if atk > def {
            end_of_fight(self, target);
        } else {
            end_of_fight(target, self);
        }
    }

    pub fn steal(&mut self, target: &mut Self, money: u32) {
        if money < target.wealth {
            self.wealth += money;
            target.wealth -= money;
        } else {
            self.wealth += target.wealth;
            target.wealth = 0;
        }
    }

    pub fn conquer_city(&mut self, mobs: Vec<Self>, city_name: String, value: u8) {
        let taken = mobs
            .iter()
            .flat_map(|mob| mob.cities.iter())
            .any(|c| c.0 == city_name);

        if !taken {
            self.cities.push((city_name, value));
        }
    }
}

//_____________________________________________________________________________________
//

fn power_combat_score(mob: &Mob) -> u32 {
    use member::Role::*;

    mob.members
        .iter()
        .map(|m| match m.role {
            Associate => 1,
            Soldier => 2,
            Caporegime => 3,
            Underboss => 4,
        })
        .sum::<u32>()
}

//---------------------------------------------------

fn end_of_fight(winner: &mut Mob, loser: &mut Mob) {
    loser.members.pop();

    if loser.members.is_empty() {
        winner.cities.extend(loser.cities.clone());
        loser.cities.clear();

        winner.steal(loser, loser.wealth);
    }
}
