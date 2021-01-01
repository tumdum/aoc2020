require 'set'

def select(current, selected, max)
  loop do
    current -= 1
    if current < 1
      current = max
    end
    if !selected.include?(current)
      return current
    end
  end
end

def take(m, n)
  ret=[]
  current=1
  (1..n).each do
    n = m[current]
    ret << n
    current = n
  end
  ret
end

def solve(input, moves)
  max = input.max
  input << input[0]
  current = input[0]
  input = input.each_cons(2).to_h


  (0..(moves-1)).each do |i|
    a = input[current]
    b = input[a]
    c = input[b]
    d = input[c]
    dest = select(current, [a,b,c], max)
    next_dest = input[dest]
    input[dest] = a
    input[c] = next_dest
    input[current]=d
    current=d
  end

  input
end

moves=10000000
total=1000000
input = "219748365".chars.map(&:to_i)

puts take(solve(input.dup, 100), 8).join("")
puts take(solve(input + (input.max+1..total).to_a, moves), 2).reduce(:*)

