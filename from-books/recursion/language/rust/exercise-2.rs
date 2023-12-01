enum NestedNumber {
    Num(i32),
    Nested(Vec<NestedNumber>),
}

fn print_numbers(data: &NestedNumber) {
    match data {
        NestedNumber::Num(x) => println!("{}", x),
        NestedNumber::Nested(vec) => {
            for item in vec {
                print_numbers(item);
            }
        }
    }
}

fn main() {
    let mixed = NestedNumber::Nested(vec![
        NestedNumber::Num(1),
        NestedNumber::Num(2),
        NestedNumber::Num(3),
        NestedNumber::Nested(vec![
            NestedNumber::Num(4),
            NestedNumber::Num(5),
            NestedNumber::Num(6),
        ]),
        NestedNumber::Num(7),
        NestedNumber::Num(8),
        NestedNumber::Nested(vec![
            NestedNumber::Num(9),
            NestedNumber::Nested(vec![NestedNumber::Num(10), NestedNumber::Num(11)]),
        ]),
    ]);

    print_numbers(&mixed);
}
