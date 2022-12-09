use std::io::{self, Write};
#[derive(Debug)]
struct Tree {
    height: usize,
    visible: bool,
    score: usize,
}

fn main() {
    let mut trees = Vec::<Vec<Tree>>::new();
    let mut h = 0;
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.len() == 1 {
            break;
        }
        let mut treerow: Vec<Tree> = input
            .split_at(input.len() - 1)
            .0
            .chars()
            .map(|x| Tree {
                height: x.to_digit(10).unwrap() as usize,
                visible: false,
                score: 0,
            })
            .collect();
        trees.push(Vec::<Tree>::new());
        trees[h].append(&mut treerow);
        h += 1;
    }
    let w = trees[0].len();
    let mut max_score = 0;
    //calc(&mut trees, (0, 1), (0, 0));
    let mut num = 0;
    println!();
    for y in 0..h {
        for x in 0..w {
            trees[y][x].visible = check(&trees, (x, y), (x, h))
                || check(&trees, (x, y), (x, 0))
                || check(&trees, (x, y), (0, y))
                || check(&trees, (x, y), (w, y));
            trees[y][x].score = calc(&trees, (x, y), (x, h))
                * calc(&trees, (x, y), (x, 0))
                * calc(&trees, (x, y), (0, y))
                * calc(&trees, (x, y), (w, y));
            max_score = std::cmp::max(max_score, trees[x][y].score);
            if trees[y][x].visible {
                num += 1;
                print!("{:2}|", trees[y][x].score);
            } else {
                print!("  |");
            }
        }
        print!("\n");
    }

    print!("Number {}, Max {}", num, max_score);
}

fn calc(trees: &Vec<Vec<Tree>>, cell: (usize, usize), to: (usize, usize)) -> usize {
    let w = trees[0].len();
    let h = trees.len();
    let d = ((to.0 as i32 - cell.0 as i32), (to.1 as i32 - cell.1 as i32));
    let dir = (
        if d.0 != 0 { d.0 / d.0.abs() } else { 0 },
        if d.1 != 0 { d.1 / d.1.abs() } else { 0 },
    );
    let mut scr = 0;
    let (mut x, mut y) = (cell.0, cell.1);
    while y < h {
        let max = trees[cell.1][cell.0].height;
        while x < w {
            let ht = &trees[y as usize][x as usize];
            if max <= ht.height && (x, y) != cell {
                return scr + 1;
            }
            if (x, y) != cell {
                scr += 1;
            }
            x = (x as i32 + dir.0) as usize;
            if dir.0 == 0 {
                break;
            }
        }
        y = (y as i32 + dir.1) as usize;
        if dir.1 == 0 {
            break;
        }
    }
    return scr as usize;
}

fn check(trees: &Vec<Vec<Tree>>, cell: (usize, usize), to: (usize, usize)) -> bool {
    let w = trees[0].len();
    let h = trees.len();
    let d = ((to.0 as i32 - cell.0 as i32), (to.1 as i32 - cell.1 as i32));
    let dir = (
        if d.0 != 0 { d.0 / d.0.abs() } else { 0 },
        if d.1 != 0 { d.1 / d.1.abs() } else { 0 },
    );
    let (mut x, mut y) = (cell.0, cell.1);
    while y < h {
        let max = trees[cell.1][cell.0].height;
        while x < w {
            let ht = &trees[y as usize][x as usize];
            if max <= ht.height && (x, y) != cell {
                return false;
            }
            x = (x as i32 + dir.0) as usize;
            if dir.0 == 0 {
                break;
            }
        }
        y = (y as i32 + dir.1) as usize;
        if dir.1 == 0 {
            break;
        }
    }
    return true;
}
fn calc_visible(trees: &mut Vec<Vec<Tree>>, dir: (i32, i32), start: (usize, usize)) {
    let w = trees[0].len() as i32;
    let h = trees.len() as i32;
    let (mut x, mut y) = (start.0 as i32, start.1 as i32);

    while 0 <= y && y < h {
        let mut max = -1;
        'inner: while 0 <= x && x < w {
            let ht = &mut trees[y as usize][x as usize];
            if max < ht.height as i32 {
                max = ht.height as i32;
                ht.visible = true;
            } else {
                break 'inner;
            }
            x = x + dir.0;
        }
        y = y + dir.1;
    }
}
