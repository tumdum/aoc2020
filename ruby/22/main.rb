require 'set'

def score(a, b) = (a + b).reverse.each_with_index.map{|v, i| v * (i+1)}.sum

def round(p1, p2)
  p1c = p1.delete_at(0)
  p2c = p2.delete_at(0)
  if p1c > p2c
    p1 << p1c
    p1 << p2c
  else
    p2 << p2c
    p2 << p1c
  end
end

def play(p1, p2)
  while !p1.empty? && !p2.empty?
    round(p1, p2)
  end
  score(p1, p2)
end
p1, p2 = STDIN.readlines.map(&:strip).chunk_while{|l| !l.empty?}.map{|cards| cards[1..].reject(&:empty?).map(&:to_i).to_a }

p play(p1.dup, p2.dup)

def play_rec(p1, p2)
  seen = Set.new
  while !p1.empty? && !p2.empty?
    if seen.include?([p1, p2])
      return [1, score(p1, p2)]
    end
    seen.add([p1.dup, p2.dup])
    round_rec(p1, p2)
  end
  [if !p1.empty? then 1 else 2 end, score(p1, p2)]
end

def round_rec(p1, p2)
  p1c = p1.delete_at(0)
  p2c = p2.delete_at(0)
  winner = 2
  if p1c <= p1.length && p2c <= p2.length
    winner, _ = play_rec(p1[..(p1c-1)], p2[..(p2c-1)])
  elsif p1c > p2c
    winner = 1
  end
  if winner == 1
    p1 << p1c
    p1 << p2c
  else
    p2 << p2c
    p2 << p1c
  end
end

p play_rec(p1.dup, p2.dup)[1]
