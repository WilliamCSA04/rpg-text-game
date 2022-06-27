use colored::{ColoredString, Colorize};
use rand::Rng;

pub struct Character {
    hp: u64,
    mana: u64,
    ad: u64,
    ap: u64,
    pr: u64,
    mr: u64,
    name: ColoredString,
    class: CLASSES,
    healthy: HEALTHY,
}

pub fn build_character(
    hp: u64,
    mana: u64,
    ad: u64,
    ap: u64,
    pr: u64,
    mr: u64,
    name: ColoredString,
    class: CLASSES,
    healthy: HEALTHY,
) -> Character {
    Character {
        hp,
        mana,
        ad,
        ap,
        pr,
        mr,
        name,
        class,
        healthy,
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
    build_character(
        500,
        500,
        10,
        20,
        50,
        50,
        "Misthy".to_string().purple(),
        CLASSES::OFFTANK,
        HEALTHY::ALIVE,
    )
}

pub fn build_archer() -> Character {
    build_character(
        200,
        100,
        100,
        10,
        10,
        5,
        "Myumii".to_string().red(),
        CLASSES::ARCHER,
        HEALTHY::ALIVE,
    )
}

pub fn build_mage() -> Character {
    build_character(
        250,
        1000,
        5,
        100,
        5,
        15,
        "Liwphael".to_string().yellow(),
        CLASSES::MAGE,
        HEALTHY::ALIVE,
    )
}

pub fn build_enemy() -> Character {
    let mut rng = rand::thread_rng();
    build_character(
        rng.gen_range(100..1000),
        rng.gen_range(100..1000),
        rng.gen_range(25..150),
        rng.gen_range(50..200),
        rng.gen_range(10..110),
        rng.gen_range(15..130),
        "Random".to_string().yellow(),
        CLASSES::UNKNOWN,
        HEALTHY::ALIVE,
    )
}

pub fn start() {
    let liw = build_mage();
    let myu = build_offtank();
    let misthy = build_archer();
    let enemy = build_enemy();
    println!(
        "Characters party: {}, {}, {}",
        misthy.name, myu.name, liw.name
    );
    println!(
        "Enemy status = hp:{}, mana:{}, ad:{}, ap:{}, pr:{}, mr{}",
        enemy.hp, enemy.mana, enemy.ad, enemy.ap, enemy.pr, enemy.mr
    );
}
