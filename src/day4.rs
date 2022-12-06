use std::fs::File;
use std::io::Read;

use scan_fmt::scan_fmt;

pub fn intervals_contain(row: &((i32, i32), (i32, i32))) -> bool {
    return (row.0.0 >= row.1.0 && row.0.1 <= row.1.1) ||
        (row.1.0 >= row.0.0 && row.1.1 <= row.0.1);
}

pub fn intervals_intersect(row: &((i32, i32), (i32, i32))) -> bool {
    return (row.0.0 <= row.1.1 && row.0.1 >= row.1.0) ||
           (row.1.0 <= row.0.1 && row.1.1 >= row.0.0);
}

pub fn run_day4() {
    println!("Starting day 4!");

    let mut f = File::open("data/day4.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let rows = s.split("\n").map(
        |row|  
            if let Ok((x1,y1,x2,y2)) = scan_fmt!(row, "{d}-{d},{d}-{d}", i32, i32, i32, i32) {
                ((x1, y1), (x2, y2))
            } else {
                panic!("Unable to parse line: {}", row);
            }).collect::<Vec<_>>();

    let part_1 = rows.iter().filter(|r| intervals_contain(r)).count();

    println!("Part 1 total: {}", part_1);

    let part_2 = rows.iter().filter(|r| intervals_intersect(r)).count();

    println!("Part 2 total: {}", part_2);

}

mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(intervals_intersect(&((5,7), (7, 9))), true);
        assert_eq!(intervals_intersect(&((2,8), (3, 7))), true);
        assert_eq!(intervals_intersect(&((6,6), (4, 6))), true);
        assert_eq!(intervals_intersect(&((2,6), (4, 8))), true);
    }
}