use crate::components::stats::Stats;

pub fn attack(attacker: &Stats, defender: &mut Stats) {
    let damage = (attacker.attack - defender.defense).max(0);
    defender.health = (defender.health - damage).max(0);
}