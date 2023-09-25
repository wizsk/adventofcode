#[allow(unused)]
pub mod mnk;
pub mod part_1;
pub mod part_2;

#[derive(Debug)]
enum Operator {
    None,
    Plus,
    Multiply,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Monkey {
    idx: usize,
    op: Operation,
    items: Vec<usize>,
    test: Test,
}

#[derive(Debug)]
struct Operation {
    operatin: Operator,
    number: Option<usize>,
}

#[derive(Debug)]
struct Test {
    devide_by: usize,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn new(idx: usize) -> Monkey {
        Monkey {
            idx,
            items: vec![],
            op: Operation {
                operatin: Operator::None,
                number: None,
            },
            test: Test {
                devide_by: 0,
                if_true: 0,
                if_false: 0,
            },
        }
    }
}

const _TEST_STR: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
