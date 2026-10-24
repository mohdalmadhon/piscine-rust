pub mod boss;
pub mod member;

pub use boss::*;
pub use member::*;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_string(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    fn power(&self) -> u32 {
        self.members.values().map(|m| m.role.power()).sum()
    }

    fn remove_youngest(&mut self) {
        if let Some(min_age) = self.members.values().map(|m| m.age).min() {
            self.members.retain(|_, m| m.age != min_age);
        }
    }

    pub fn attack(&mut self, other: &mut Mob) {
        // draw -> the attacker (self) loses
        let (winner, loser) = if self.power() > other.power() {
            (self, other)
        } else {
            (other, self)
        };

        loser.remove_youngest();

        if loser.members.is_empty() {
            winner.cities.extend(loser.cities.drain());
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u64) {
        let stolen = value.min(target.wealth);
        target.wealth -= stolen;
        self.wealth += stolen;
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], city: String) {
        if mobs.iter().all(|mob| !mob.cities.contains(&city)) {
            self.cities.insert(city);
        }
    }
}
