use colored::{ColoredString, Colorize};
use rand::Rng;

pub struct Character {
    hp: i64,
    mana: i64,
    ad: i64,
    ap: i64,
    pr: i64,
    mr: i64,
    name: ColoredString,
    class: CLASSES,
    healthy: HEALTHY,
}

impl Character {
    pub fn deal_damage(&mut self, damage: i64) {
        let pre_damage = damage - self.pr;
        let full_damage = if pre_damage < 0 { 0 } else { pre_damage };
        let final_damage = self.hp - full_damage;
        self.hp = final_damage;
        println!(
            "Damage dealt: {}; New HP: {}",
            full_damage.to_string().red(),
            final_damage.to_string().green()
        )
    }
}

pub enum CLASSES {
    MAGE,
    OFFTANK,
    ARCHER,
    UNKNOWN,
}

pub enum HEALTHY {
    ALIVE,
    DEAD,
}

pub fn build_offtank() -> Character {
    Character {
        hp: 500,
        mana: 500,
        ad: 10,
        ap: 20,
        pr: 50,
        mr: 50,
        name: "Misthy".to_string().purple(),
        class: CLASSES::OFFTANK,
        healthy: HEALTHY::ALIVE,
    }
}

pub fn build_archer() -> Character {
    Character {
        hp: 200,
        mana: 100,
        ad: 100,
        ap: 10,
        pr: 10,
        mr: 5,
        name: "Myumii".to_string().red(),
        class: CLASSES::ARCHER,
        healthy: HEALTHY::ALIVE,
    }
}

pub fn build_mage() -> Character {
    Character {
        hp: 250,
        mana: 1000,
        ad: 5,
        ap: 100,
        pr: 5,
        mr: 15,
        name: "Liwphael".to_string().yellow(),
        class: CLASSES::MAGE,
        healthy: HEALTHY::ALIVE,
    }
}

pub fn build_enemy() -> Character {
    let mut rng = rand::thread_rng();
    Character {
        hp: rng.gen_range(100..1000),
        mana: rng.gen_range(100..1000),
        ad: rng.gen_range(25..150),
        ap: rng.gen_range(50..200),
        pr: rng.gen_range(10..110),
        mr: rng.gen_range(15..130),
        name: "Random".to_string().yellow(),
        class: CLASSES::UNKNOWN,
        healthy: HEALTHY::ALIVE,
    }
}

pub fn start() {
    let liw = build_mage();
    let myu = build_offtank();
    let misthy = build_archer();
    let mut enemy = build_enemy();
    println!(
        "Characters party: {}, {}, {}",
        misthy.name, myu.name, liw.name
    );
    println!(
        "Enemy status = hp:{}, mana:{}, ad:{}, ap:{}, pr:{}, mr{}",
        enemy.hp, enemy.mana, enemy.ad, enemy.ap, enemy.pr, enemy.mr
    );
    for i in 0..5 {
        if is_dead(&myu) && is_dead(&misthy) && is_dead(&liw) {
            println!("Game Over");
            break;
        }
        if is_dead(&enemy) {
            println!("You won!");
            break;
        }
        enemy.deal_damage(100);
    }
}

fn is_dead(char: &Character) -> bool {
    matches!(char.healthy, HEALTHY::DEAD)
}
