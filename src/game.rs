use colored::{ColoredString, Colorize};

pub struct Character {
    hp: u64,
    mana: u64,
    ad: u64,
    ap: u64,
    pr: u64,
    mr: u64,
    name: ColoredString,
    class: CLASSES,
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
    }
}

pub enum CLASSES {
    MAGE,
    OFFTANK,
    ARCHER,
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
    )
}

pub fn start() {
    let liw = build_mage();
    let myu = build_offtank();
    let misthy = build_archer();
    println!("Characters: {}, {}, {}", misthy.name, myu.name, liw.name)
}
