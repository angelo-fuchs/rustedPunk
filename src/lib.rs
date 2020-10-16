// Name: Erwin Müller
// Role: Corporate Age: 23
// Att:3/3
// mov: 4/4
// Coo: 3/3
// Emp: 3/3
// luck: 3/3
// Int: 10/10
// body: 7/7 Ref: 6/6 Tch: 10/10
// Ressourc:  5 = 15   | Schaden: OOOO.0000.0000.0000.0000.0000.0000.0000.0000
// HumPerce:  2 =  5  | KO       220X1X4362X23032X334X0105XX2X610XX7XXXX8XXXX9
// 0GFight :  1 =  7      | DEATH                   X336110X6210XX3XX0X4332X5XXX06
// Composit:  1 = 11

pub struct Character {
    pub name: String,
    pub role: String,
    pub age: usize,
    pub att: (usize, usize),
    pub mov: (usize, usize),
    pub coo: (usize, usize),

    pub emp: (usize, usize),
    pub luck: (usize, usize),
    pub int: (usize, usize),
    pub body: (usize, usize),
    pub refl: (usize, usize),
    pub tec: (usize, usize),
}

impl Character {
    pub fn print(self) {
        println!(
            "\
Character {{ \n\
\tname: {}\n\
\trole: {}\n\
\tage: {}\n\
\tatt: {}/{}\n\
\tmov: {}/{}\n\
\tcoo: {}/{}\n\
\temp: {}/{}\n\
\tluck: {}/{}\n\
\tint: {}/{}\n\
\tbody: {}/{}\n\
\tref: {}/{}\n\
\ttec: {}/{}\n\
}}",
            self.name,
            self.role,
            self.age,
            self.att.0,
            self.att.1,
            self.mov.0,
            self.mov.1,
            self.coo.0,
            self.coo.1,
            self.emp.0,
            self.emp.1,
            self.luck.0,
            self.luck.1,
            self.int.0,
            self.int.1,
            self.body.0,
            self.body.1,
            self.refl.0,
            self.refl.1,
            self.tec.0,
            self.tec.1,
        );
    }
}
