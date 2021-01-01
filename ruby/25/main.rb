def handshake(subject, loop_size) = (1..loop_size).reduce(1) {|n| (n * subject) % 20201227 }

def find_loop_size(subject, pub_key)
  n = 1
  (1..).find {|_| true if pub_key == n = (n * subject) % 20201227 }
end

def solve(pub_a, pub_b) = handshake(pub_b, find_loop_size(7, pub_a))

p solve(8421034, 15993936)
