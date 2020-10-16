use rusted_punk::*;

fn main() {
    let cool_guy = Character {
        name: "Erwin Müller".to_string(),
        role: "Corporate".to_string(),
        age: 23,
        att: (3, 3),
        bew: (4, 4),
        coo: (3, 3),
    };

    cool_guy.print();
}
