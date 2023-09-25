use crate::*;

pub fn game(input: &str) -> Vec<Monkey> {
    let mut game: Vec<Monkey> = Vec::new();

    for (i, round) in input.trim().split("\n\n").enumerate() {
        let mut mnky = Monkey::new(i);
        let mut gm = round.trim().lines();
        // skip monky idx
        gm.next().unwrap();

        // starting items
        mnky.items = gm
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(",")
            .map(|itm| itm.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        // operatins
        let op = gm.next().unwrap().trim();
        let operator = op
            .split("=")
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap();

        match operator {
            "+" => mnky.op.operatin = Operator::Plus,
            "*" => mnky.op.operatin = Operator::Multiply,
            _ => panic!("unknown '{operator}' operator"),
        }

        // op.split(" ").last().unwrap();
        mnky.op.number = match op.split_whitespace().last().unwrap().parse::<usize>() {
            Ok(val) => Some(val),
            Err(_) => None,
        };

        // test
        mnky.test.devide_by = gm
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        mnky.test.if_true = gm
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        mnky.test.if_false = gm
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        game.push(mnky);
    }

    game
}

// mod tests {
//     #[test]
//     fn test_p1() {
//         use super::*;
//         use crate::_TEST_STR;
//
//         p1(_TEST_STR);
//         assert!(false);
//     }
// }
