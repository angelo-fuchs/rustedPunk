use rusted_punk::{Attribute, Character};

fn main() {
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
