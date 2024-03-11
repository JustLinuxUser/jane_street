type Matrix = [[u8; 6]; 6];

struct Coord {
    x: usize,
    y: usize,
}

struct ConstraintSet {
    rows: [[Option<u8>; 2]; 6],
    columns: [[Option<u8>; 2]; 6],
}

fn main() {
    let mut matrix: Matrix = [[0u8; 6]; 6];
    let cs = ConstraintSet {
        rows: [
            [Some(3), None],
            [None, Some(4)],
            [None, None],
            [Some(6), None],
            [None, Some(5)],
            [Some(3), Some(3)],
        ],
        columns: [
            [Some(4), None],
            [None, None],
            [Some(3), None],
            [Some(3), Some(4)],
            [None, None],
            [None, None],
        ],
    };
    solve(&mut matrix, &cs);
}

fn solve(matrix: &mut Matrix, cs: &ConstraintSet) {
    let mut curr_pos = 0;
    let mut max_pos = 0;
    let mut crd;
    loop {
        crd = get_next(curr_pos);
        if matrix[crd.y][crd.x] == 6 {
            matrix[crd.y][crd.x] = 0;
            if curr_pos == 0 {
                panic!("No solutions");
            }
            curr_pos -= 1;
        } else {
            while matrix[crd.y][crd.x] != 6 {
                matrix[crd.y][crd.x] += 1;
                if check_valid(&crd, &matrix, &cs) {
                    curr_pos += 1;
                    if curr_pos > max_pos {
                        max_pos = curr_pos;
                        println!("{}", max_pos);
                    }
                    break;
                }
            }
        }
        if curr_pos == 36 {
            for line in matrix.as_ref() {
                for e in line {
                    print!("{e} ");
                }
                println!()
            }
            curr_pos -= 1;
        }
    }
}
fn get_next(idx: u8) -> Coord {
    if idx > 35 {
        panic!("Index {idx} out of range of 6x6 matrix");
    }
    Coord {
        x: (idx % 6) as usize,
        y: (idx / 6) as usize,
    }
}

fn check_valid(c: &Coord, matrix: &Matrix, cs: &ConstraintSet) -> bool {
    if row_repetition(c, matrix) || col_repetition(c, matrix) {
        return false;
    }
    match cs.rows[c.y][0] {
        Some(crd) => {
            if !check_l2r(c, matrix, crd) {
                return false;
            }
        }
        _ => (),
    }
    match cs.rows[c.y][1] {
        Some(crd) => {
            if !check_r2l(c, matrix, crd) {
                return false;
            }
        }
        _ => (),
    }
    match cs.columns[c.x][0] {
        Some(crd) => {
            if !check_t2b(c, matrix, crd) {
                return false;
            }
        }
        _ => (),
    }
    match cs.columns[c.x][1] {
        Some(crd) => {
            if !check_b2t(c, matrix, crd) {
                return false;
            }
        }
        _ => (),
    }
    true
}

fn row_repetition(c: &Coord, matrix: &Matrix) -> bool {
    let mut encountered = [false; 7];
    for n in matrix[c.y as usize] {
        if n == 0 {
            continue;
        }
        if encountered[n as usize] {
            return true;
        }
        encountered[n as usize] = true;
    }
    false
}
fn col_repetition(c: &Coord, matrix: &Matrix) -> bool {
    let mut encountered = [false; 7];
    for n in matrix {
        let n = n[c.x as usize];
        if n == 0 {
            continue;
        }
        if encountered[n as usize] {
            return true;
        }
        encountered[n as usize] = true;
    }
    false
}
fn check_l2r(c: &Coord, matrix: &Matrix, constraint: u8) -> bool {
    let mut max = 0;
    let mut count = 0;
    for (_pos, n) in matrix[c.y as usize].iter().enumerate() {
        if *n == 0 {
            return true;
        }
        if *n > max {
            max = *n;
            count += 1;
        }
    }
    if count != constraint {
        false
    } else {
        true
    }
}
fn check_r2l(c: &Coord, matrix: &Matrix, constraint: u8) -> bool {
    let mut max = 0;
    let mut count = 0;
    for (_pos, n) in matrix[c.y as usize].iter().rev().enumerate() {
        if *n == 0 {
            return true;
        }
        if *n > max {
            max = *n;
            count += 1;
        }
    }
    if count != constraint {
        false
    } else {
        true
    }
}
fn check_t2b(c: &Coord, matrix: &Matrix, constraint: u8) -> bool {
    let mut max = 0;
    let mut count = 0;
    for (_pos, n) in matrix.iter().enumerate() {
        let n = n[c.x as usize];
        if n == 0 {
            return true;
        }
        if n > max {
            max = n;
            count += 1;
        }
    }
    if count != constraint {
        false
    } else {
        true
    }
}
fn check_b2t(c: &Coord, matrix: &Matrix, constraint: u8) -> bool {
    let mut max = 0;
    let mut count = 0;
    for (_pos, n) in matrix.iter().rev().enumerate() {
        let n = n[c.x as usize];
        if n == 0 {
            return true;
        }
        if n > max {
            max = n;
            count += 1;
        }
    }
    if count != constraint {
        false
    } else {
        true
    }
}
