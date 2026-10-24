use mobs::*;
use std::collections::{HashMap, HashSet};

fn mob(name: &str, members: &[(&str, Role, u32)], cities: &[&str], wealth: u64) -> Mob {
    Mob {
        name: name.to_string(),
        boss: boss::Boss::new("Boss", 50),
        members: members
            .iter()
            .map(|&(n, role, age)| (n.to_string(), member::Member { role, age }))
            .collect::<HashMap<_, _>>(),
        cities: cities.iter().map(|c| c.to_string()).collect::<HashSet<_>>(),
        wealth,
    }
}

#[test]
fn boss_new() {
    assert_eq!(Boss::new("Louie", 36), Boss { name: "Louie".into(), age: 36 });
}

#[test]
fn recruit_is_associate() {
    let mut m = mob("A", &[], &[], 0);
    m.recruit(("Rob", 22));
    assert_eq!(m.members["Rob"], Member { role: Role::Associate, age: 22 });
}

#[test]
fn promotions() {
    let mut m = Member { role: Role::Associate, age: 30 };
    m.get_promotion();
    assert_eq!(m.role, Role::Soldier);
    m.get_promotion();
    assert_eq!(m.role, Role::Caporegime);
    m.get_promotion();
    assert_eq!(m.role, Role::Underboss);
}

#[test]
#[should_panic]
fn underboss_cannot_be_promoted() {
    Member { role: Role::Underboss, age: 30 }.get_promotion();
}

#[test]
fn attack_stronger_attacker_wins() {
    let mut a = mob("A", &[("x", Role::Underboss, 40)], &[], 0);
    let mut b = mob("B", &[("y", Role::Soldier, 20), ("z", Role::Associate, 30)], &[], 0);
    a.attack(&mut b);
    assert_eq!(a.members.len(), 1);
    assert!(b.members.contains_key("z") && !b.members.contains_key("y"));
}

#[test]
fn attack_draw_attacker_loses_all_youngest() {
    let mut a = mob("A", &[("x", Role::Soldier, 20), ("w", Role::Soldier, 20), ("v", Role::Associate, 25)], &[], 0);
    let mut b = mob("B", &[("y", Role::Underboss, 30), ("z", Role::Associate, 31)], &[], 0);
    a.attack(&mut b);
    assert_eq!(a.members.keys().collect::<Vec<_>>(), vec!["v"]);
    assert_eq!(b.members.len(), 2);
}

#[test]
fn attack_wipe_out_takes_cities_and_wealth() {
    let mut a = mob("A", &[("x", Role::Caporegime, 40)], &["Rome"], 100);
    let mut b = mob("B", &[("y", Role::Associate, 20)], &["Naples", "Milan"], 500);
    a.attack(&mut b);
    assert!(b.members.is_empty());
    assert_eq!(b.wealth, 0);
    assert!(b.cities.is_empty());
    assert_eq!(a.wealth, 600);
    assert_eq!(a.cities.len(), 3);
}

#[test]
fn steal_capped_by_target_wealth() {
    let mut a = mob("A", &[], &[], 10);
    let mut b = mob("B", &[], &[], 50);
    a.steal(&mut b, 20);
    assert_eq!((a.wealth, b.wealth), (30, 30));
    a.steal(&mut b, 1000);
    assert_eq!((a.wealth, b.wealth), (60, 0));
}

#[test]
fn conquer_city_only_if_free() {
    let mut a = mob("A", &[], &[], 0);
    let b = mob("B", &[], &["Rome"], 0);
    let c = mob("C", &[], &["Turin"], 0);
    a.conquer_city(&[&b, &c], "Rome".into());
    assert!(a.cities.is_empty());
    a.conquer_city(&[&b, &c], "Venice".into());
    assert!(a.cities.contains("Venice"));
}
