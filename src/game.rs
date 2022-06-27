pub struct Character {
    hp: u64,
    mana: u64,
    ad: u64,
    ap: u64,
    pr: u64,
    mr: u64,
    name: String,
}

pub fn build_character(
    hp: u64,
    mana: u64,
    ad: u64,
    ap: u64,
    pr: u64,
    mr: u64,
    name: String,
) -> Character {
    Character {
        hp,
        mana,
        ad,
        ap,
        pr,
        mr,
        name,
    }
}

pub fn build_misthy() -> Character {
    build_character(500, 500, 10, 20, 50, 50, "Misthy".to_string())
}

pub fn build_myu() -> Character {
    build_character(200, 100, 100, 10, 10, 5, "Myumii".to_string())
}

pub fn build_liw() -> Character {
    build_character(250, 1000, 5, 100, 5, 15, "Liwphael".to_string())
}

pub fn start() {
    let liw = build_liw();
    let myu = build_misthy();
    let misthy = build_myu();
    println!("Characters: {}, {}, {}", misthy.name, myu.name, liw.name)
}
