require 'set'
input = STDIN.readlines
  .map {|l| l.strip}
  .chunk_while {|l| !l.empty? }
  .map {|l| l.reject {|s| s.empty?}.map {|s| s.chars} }
p input.map{|s| s.flatten.to_set.length }.sum
p input.map{|s| s.map{|s| s.to_set}.reduce{|acc, s| acc & s}}.map{|s| s.length}.sum
