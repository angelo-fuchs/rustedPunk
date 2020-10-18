use rusted_punk::{Attribute, Character, Skill};

fn main() {
    //character_test();
    //skill_test();
}

fn character_test() {
    let cool_guy = Character {
        name: "Erwin Müller".to_string(),
        role: "Corporate".to_string(),
        age: 23,
        att: Attribute::new(3, 3),
        mov: Attribute::new(4, 4),
        coo: Attribute::new(1, 3),
        emp: Attribute::new(3, 3),
        luck: Attribute::new(3, 3),
        int: Attribute::new(10, 10),
        body: Attribute::new(7, 7),
        refl: Attribute::new(6, 6),
        tec: Attribute::new(10, 10),
    };
    cool_guy.print();
}

fn skill_test() {
    let skill = Skill::new("Schleichen".to_string(), 7, 2, 3);
    skill.print()
}
