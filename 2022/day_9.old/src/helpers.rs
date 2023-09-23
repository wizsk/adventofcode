// x, y co ordinates
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub head: Point,
    pub tail: Point,
}

impl Position {
    pub fn new() -> Position {
        Position {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        }
    }

    // this is for the part 1
    pub fn calculate(&mut self) {
        let dx = self.head.x - self.tail.x;
        let dy = self.head.y - self.tail.y;
        // same row || same cullmn and next to eatch other
        // println!("calc: {:?}", self);
        if dx == 0 && dy == 0
            || self.head.x == self.tail.x && (dy == 1 || dy == -1)
            || self.head.y == self.tail.y && (dx == 1 || dx == -1)
        {
            // println!("pasei ache");
            return;
        }

        if self.head.x == self.tail.x {
            if dy > 1 {
                self.tail.y += 1;
            } else {
                self.tail.y -= 1;
            }
        } else if self.head.y == self.tail.y {
            if dx > 1 {
                self.tail.x += 1;
            } else {
                self.tail.x -= 1;
            }
        } else if self.head.x == self.tail.x + 1 && self.head.y == self.tail.y + 2
            || self.head.x == self.tail.x + 2 && self.head.y == self.tail.y + 1
        {
            self.tail.x += 1;
            self.tail.y += 1;
        } else if self.head.x == self.tail.x + 1 && self.head.y == self.tail.y - 2
            || self.head.x == self.tail.x + 2 && self.head.y == self.tail.y - 1
        {
            self.tail.x += 1;
            self.tail.y -= 1;
        } else if self.head.x == self.tail.x - 1 && self.head.y == self.tail.y - 2
            || self.head.x == self.tail.x - 2 && self.head.y == self.tail.y - 1
        {
            self.tail.x -= 1;
            self.tail.y -= 1;
        } else if self.head.x == self.tail.x - 1 && self.head.y == self.tail.y + 2
            || self.head.x == self.tail.x - 2 && self.head.y == self.tail.y + 1
        {
            self.tail.x -= 1;
            self.tail.y += 1;
        }
        // println!("cccc: {:?}\n\t{dx} {dy}", self);
    }
}

pub fn calc_point(c: &Point) {
    let p: Vec<Point> = vec![
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 1, y: -1 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: -1 },
        Point { x: -1, y: 0 },
        Point { x: -1, y: 1 },
    ];
}
