use std::collections::HashSet;
use std::io::BufRead;

fn decode_row(input: &str) -> usize {
    decode(input, 0, 127)
}

fn decode_col(input: &str) -> usize {
    decode(input, 0, 7)
}

fn decode(input: &str, l: usize, h: usize) -> usize {
    fn step((l, h): (usize, usize), c: char) -> (usize, usize) {
        match c {
            'F' | 'L' => (l, l + (h - l) / 2),
            _ => (l + (h - l) / 2 + 1, h),
        }
    }
    input.chars().fold((l, h), step).0
}

fn decode_ticket(input: &String) -> (usize, usize) {
    (decode_row(&input[..7]), decode_col(&input[7..]))
}

fn ticket_id((a, b): (usize, usize)) -> usize {
    a * 8 + b
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let t = input
        .iter()
        .map(decode_ticket)
        .map(ticket_id)
        .collect::<HashSet<_>>();
    let max = *dbg!(t.iter().max()).unwrap();
    let my_ticket = |i: &usize| t.contains(&(i - 1)) && t.contains(&(i + 1)) && !t.contains(&i);
    dbg!((1..=max).find(my_ticket));
}

#[test]
fn decode_test() {
    assert_eq!(44, decode_row("FBFBBFF"));
    assert_eq!(70, decode_row("BFFFBBF"));
    assert_eq!(14, decode_row("FFFBBBF"));
    assert_eq!(102, decode_row("BBFFBBF"));

    assert_eq!(5, decode_col("RLR"));
    assert_eq!(7, decode_col("RRR"));
    assert_eq!(4, decode_col("RLL"));

    assert_eq!((44, 5), decode_ticket(&"FBFBBFFRLR".to_owned()));
}
