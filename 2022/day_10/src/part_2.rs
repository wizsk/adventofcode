pub fn p2(input: &str) -> u8 {
    let mut x: i32 = 1;
    let mut cy: Vec<i32> = Vec::new();
    for l in input.trim().lines() {
        if l == "noop" {
            cy.push(x);
        } else {
            cy.push(x);
            cy.push(x);
            x += l
                .split(" ")
                .collect::<Vec<_>>()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
    }

    for i in (0..cy.len()).step_by(40) {
        for j in 0..40 {
            let idk = cy[i + j] - j as i32;
            if idk.abs() <= 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    return 0;
}

