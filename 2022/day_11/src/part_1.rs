use crate::*;

pub fn p1(input: &str) -> usize {
    let mut game = mnk::game(input);
    let mut inspected: Vec<usize> = vec![0; game.len()];

    for _ in 0..20 {
        for i in 0..game.len() {
            for j in 0..game[i].items.len() {
                inspected[i] += 1;
                let mut curr = game[i].items[j];
                match game[i].op.operatin {
                    Operator::Plus => curr += game[i].op.number.unwrap_or(curr),
                    Operator::Multiply => curr *= game[i].op.number.unwrap_or(curr),
                    Operator::None => panic!("p1: operation can't be null"),
                };

                curr /= 3;
                let mut through_to = game[i].test.if_true;
                if curr % game[i].test.devide_by != 0 {
                    through_to = game[i].test.if_false;
                }
                game[through_to as usize].items.push(curr);
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

        assert_eq!(p1(_TEST_STR), 10605);
    }
}
