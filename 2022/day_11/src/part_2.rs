use crate::*;

pub fn p2(input: &str) -> usize {
    let mut game = mnk::game(input);
    let mut inspected: Vec<usize> = vec![0; game.len()];

    // modulo aretch matic magicccc
    // ref: https://www.youtube.com/watch?v=F4MCuPZDKog
    let mut modulo: usize = 1;
    for g in &game {
        modulo *= g.test.devide_by;
    }

    for _ in 0..10000 {
        for i in 0..game.len() {
            for j in 0..game[i].items.len() {
                inspected[i] += 1;

                let mut curr = game[i].items[j];
                match game[i].op.operatin {
                    Operator::Plus => curr += game[i].op.number.unwrap_or(curr),
                    Operator::Multiply => curr *= game[i].op.number.unwrap_or(curr),
                    Operator::None => panic!("p2: operation can't be null"),
                };

                curr %= modulo;

                let mut throw_to = game[i].test.if_true;
                if curr % game[i].test.devide_by != 0 {
                    throw_to = game[i].test.if_false;
                }
                game[throw_to as usize].items.push(curr);
            }
            // removing the items
            game[i].items = vec![];
        }
    }

    inspected.sort();
    inspected.last().unwrap() * inspected[inspected.len() - 2]
}

mod tests {
    #[test]
    fn test_p1() {
        use super::*;
        use crate::_TEST_STR;

        assert_eq!(p2(_TEST_STR), 2713310158);
    }
}
