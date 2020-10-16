use rusted_punk::*;

fn main() {
    let cool_guy = Character {
        name: "Erwin Müller".to_string(),
        role: "Corporate".to_string(),
        age: 23,
        att: (3, 3),
        mov: (4, 4),
        coo: (1, 3),
        emp: (3, 3),
        luck: (3, 3),
        int: (10, 10),
        body: (7, 7),
        refl: (6, 6),
        tec: (10, 10),
    };

    cool_guy.print();
}
