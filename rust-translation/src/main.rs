use std::collections::{HashMap};

#[derive(Debug)]
struct context {
    cols:       Vec<char>,
    rows:       Vec<char>,
    squares:    Vec<String>,
    unitlist:   Vec<Vec<String>>,
    units:      HashMap<String, Vec<Vec<String>>>,
    peers:      HashMap<String, Vec<String>>
}

fn cross(A: &[char], B: &[char]) -> Vec<String> {
    let mut cross : Vec<String> = Vec::new();
    for a in A {
        for b in B {
            let mut sq = String::new();
            sq.push(*a);
            sq.push(*b);
            cross.push(sq)
        }
    }
    cross
}

fn grid_values(grid: &str, ctx: &context) -> HashMap<String, Vec<char>> {
    let mut grid_chars : Vec<Vec<char>> =
        grid.chars()
        .filter(|c| ctx.cols.contains(c) || ['0', '1'].contains(c))
        .map(|c| vec![c]).collect();
    assert_eq!(grid_chars.len(), 81);
    let grid_val : HashMap<String, Vec<char>> =
        ctx.squares.iter().cloned().zip(grid_chars.into_iter()).collect();
    grid_val
}

fn parse_grid(grid: &str, ctx: &context) -> Option<HashMap<String, Vec<char>>> {
    let mut values : HashMap<String, Vec<char>> = HashMap::new();
    //ctx.squares.iter().cloned().zip(ctx.cols.clone()).collect();
    for s in &ctx.squares {
        values.insert(s.clone(), ctx.cols.clone());
    }
    for (s, digits) in &grid_values(grid, ctx) {
        for d in digits {
            if ctx.cols.contains(d) && !assign(&mut values, s, d, ctx) {
                return None;
            }
        }
    }
    Some(values)
}

fn assign(values: &mut HashMap<String, Vec<char>>, s: &str, d: &char, ctx: &context) -> bool {
    let other_values : Vec<char> =
        values[s].iter().filter(|d2| d2 != d).cloned().collect();
    other_values.iter().all(|d2| eliminate(values, s, d2, ctx))
}