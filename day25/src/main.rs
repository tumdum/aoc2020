fn handshake(subject: u64, loop_size: usize) -> u64 {
    let mut n = 1;
    for _ in 0..loop_size {
        n = (n * subject) % 20201227;
    }
    n
}

fn find_loop_size(subject: u64, pub_key: u64) -> usize {
    let mut n = 1;
    for loop_size in 1.. {
        n = (n * subject) % 20201227;
        if n == pub_key {
            return loop_size
        }
    }
    unreachable!()
}

fn solve(pub_a: u64, pub_b: u64) -> u64 {
    let loop_a = find_loop_size(7, pub_a);
    let loop_b = find_loop_size(7, pub_b);

    let priv_a = handshake(pub_b, loop_a);
    let priv_b = handshake(pub_a, loop_b);
    assert_eq!(priv_a, priv_b);
    priv_a
}

fn main() {
    dbg!(solve(8421034, 15993936));
}

#[test]
fn handshake_test() {
    assert_eq!(5764801, handshake(7, 8));
    assert_eq!(17807724, handshake(7, 11));

    assert_eq!(14897079, handshake(5764801,11));
    assert_eq!(14897079, handshake(17807724, 8));

    assert_eq!(find_loop_size(7, 5764801), 8);
    assert_eq!(find_loop_size(7, 17807724), 11);

    assert_eq!(solve(5764801,17807724), 14897079);
    assert_eq!(solve(17807724,5764801), 14897079);
}
