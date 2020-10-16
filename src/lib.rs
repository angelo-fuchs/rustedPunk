// Name: Erwin Müller
// Role: Corporate Age: 23
// Att:3/3
// Bew: 4/4
// Coo: 3/3
// Emp: 3/3
// Glk: 3/3
// Int: 10/10
// Kpb: 7/7 Ref: 6/6 Tch: 10/10
// Ressourc:  5 = 15   | Schaden: OOOO.0000.0000.0000.0000.0000.0000.0000.0000
// HumPerce:  2 =  5  | KO       220X1X4362X23032X334X0105XX2X610XX7XXXX8XXXX9
// 0GFight :  1 =  7      | DEATH                   X336110X6210XX3XX0X4332X5XXX06
// Composit:  1 = 11

pub struct Character {
    pub name: String,
    pub role: String,
    pub age: usize,
    pub att: (usize, usize),
    pub bew: (usize, usize),
    pub coo: (usize, usize),
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
\tcoo: {}/{}\n\
\tbew: {}/{}\n\
}}",
            self.name,
            self.role,
            self.age,
            self.att.0,
            self.att.1,
            self.bew.0,
            self.bew.1,
            self.coo.0,
            self.coo.1,
        );
    }
}
