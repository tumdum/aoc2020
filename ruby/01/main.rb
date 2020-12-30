num = STDIN.readlines.map! { |e| e.strip.to_i }
h = num.product(num).to_h { |e| [e[0]+e[1], e]}
p h[2020].reduce(:*)
a = num.detect { |n| h[2020-n] }
p h[2020-a].reduce(a) {|a,b| a * b}