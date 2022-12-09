use std::fs::File;
use std::io::Read;

fn is_tree_visible(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let size = forest.len();

    if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
        return true;
    }

    let row = forest.get(y).expect("Tree must be in a row");
    let height = row.get(x).expect("Tree must be in a row");

    // Scan right
    let mut right_visible = true;
    for j in x + 1..size {
        if row.get(j).expect("") >= height {
            right_visible = false;
            break;
        }
    }

    if right_visible {
        return true;
    }

    // Scan left
    let mut left_visible = true;
    for j in 0..x {
        if row.get(j).expect("") >= height {
            left_visible = false;
            break;
        }
    }

    if left_visible {
        return true;
    }

    // Scan up
    let mut up_visible = true;
    for i in 0..y  {
        let r = forest.get(i).expect("");

        if r.get(x).expect("") >= height {
            up_visible = false;
            break;
        }
    }

    if up_visible {
        return true;
    }

    let mut down_visible = true;
    for i in y + 1..size {
        let r = forest.get(i).expect("");

        if r.get(x).expect("") >= height {
            down_visible = false;
            break;
        }
    }

    if down_visible {
        return true;
    }

    return false;
}

fn compute_scenic_score(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let size = forest.len();

    let mut score = 1;

    let row = forest.get(y).expect("Tree must be in a row");
    let height = row.get(x).expect("Tree must be in a row");

    // Scan right
    let mut right_range = 0;

    for j in x + 1..size {
        right_range += 1;

        if row.get(j).expect("") >= height {
            break;
        }
    }

    score *= right_range;
    //println!("Right visibility: {}", right_range);

    // Scan left
    let mut left_range = 0;

    for j in (0..x).rev() {
        left_range += 1;

        if row.get(j).expect("") >= height {
            break;
        }
    }

    score *= left_range;
    //println!("Left visibility: {}", left_range);

    // Scan up
    let mut up_range = 0;
    for i in (0..y).rev()  {
        up_range += 1;

        let r = forest.get(i).expect("");

        if r.get(x).expect("") >= height {
            break;
        }
    }

    score *= up_range;
    //println!("Up visibility: {}", up_range);

    let mut down_range = 0;
    for i in y + 1..size {
        down_range += 1;

        let r = forest.get(i).expect("");

        if r.get(x).expect("") >= height {
            break;
        }
    }

    score *= down_range;

    return score;
}

pub fn run_day8() {
    println!("Starting day 8!");

    let mut f = File::open("data/day8.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let lines = s.split("\n").collect::<Vec<_>>();

    let size = lines.first().expect("At least one line").len();

    let mut forest: Vec<Vec<u32>> = Vec::new();

    for line in lines.iter() {
        let mut row: Vec<u32> = Vec::new();

        for tree in line.chars() {
            row.push(tree.to_digit(10).expect("Unable to parse tree"));            
        }

        forest.push(row);
    }

    let mut visible_trees = 0;

    for x in 0..size {
        for y in 0..size {
           if is_tree_visible(&forest, x, y) {
            visible_trees += 1;
           }
        }
    }

    println!("Visible trees: {}", visible_trees);

    let mut max_visibility = 0;
    for x in 0..size {
        for y in 0..size {
           let v = compute_scenic_score(&forest, x, y);

           if v > max_visibility {
            max_visibility = v;
           }
        }
    }

    println!("Max scenic score: {}", max_visibility);

}

mod tests {
    use super::*;

    #[test]
    fn test_is_visible() {
        let forest = vec!(
            vec!(3, 0, 3, 7, 3),
            vec!(2, 5, 5, 1, 2),
            vec!(6, 5, 3, 3, 2),
            vec!(3, 3, 5, 4, 9),
            vec!(3, 5, 3, 9, 0)
        );

        assert_eq!(super::is_tree_visible(&forest, 1, 1), true);
        assert_eq!(super::is_tree_visible(&forest, 2, 1), true);
        assert_eq!(super::is_tree_visible(&forest, 3, 1), false);
    }

    #[test]
    fn test_scenic_score() {
        let forest = vec!(
            vec!(3, 0, 3, 7, 3),
            vec!(2, 5, 5, 1, 2),
            vec!(6, 5, 3, 3, 2),
            vec!(3, 3, 5, 4, 9),
            vec!(3, 5, 3, 9, 0)
        );

        assert_eq!(super::compute_scenic_score(&forest, 2, 1), 4);
        assert_eq!(super::compute_scenic_score(&forest, 2, 3), 8);
    }
}