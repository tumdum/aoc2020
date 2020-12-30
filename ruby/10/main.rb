input = STDIN.readlines.map(&:to_i).to_a
input.sort!
input.prepend 0
input << (input.max + 3)

p input.each_cons(2).map {|a,b| b-a}.group_by {|v| v}.map{|k,v| v.length }.reduce(:*)

connects_to = input.map {|to| [to, input.filter{|from| from<to and to<=(from+3)}]}.to_h
paths_to = Hash[0, 1]
input[1..].each do |e|
  paths_to[e] = connects_to[e].map{|from| paths_to[from]}.sum
end
p paths_to[input.max]
