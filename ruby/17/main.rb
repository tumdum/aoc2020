require 'matrix'
require 'set'

$delta=[-1,0,1]

def neighbours3d(p) = $delta.product($delta, $delta, [0]).reject{|v| v.all?{|v| v==0}}.map{|d| p + Vector[*d]}
def neighbours4d(p) = $delta.product($delta, $delta, $delta).reject{|v| v.all?{|v| v==0}}.map{|d| p + Vector[*d]}
def active_neighbours(map, p, gen_n) = gen_n.call(p).count {|p| map[p]=='#'}
def all_candidates(map, gen_n) = (map.keys() + map.keys().flat_map{|p| gen_n.call(p)}).to_set

def should_be_active(map, p, gen_n)
  active=active_neighbours(map, p, gen_n)
  (active == 3) || (map[p] == '#' && active == 2)
end

def gen_new_state(map,times,gen_n)
  (1..times).each {
    map=all_candidates(map, gen_n).filter_map{|p| [p, '#'] if should_be_active(map, p, gen_n)}.to_h
    map.default='.'
  }
  map
end

input=STDIN.readlines.map(&:strip).each_with_index.flat_map{ |l, y|
  l.chars.each_with_index.map{ |c,x|
    [Vector[x, y, 0, 0], c]
  }
}.to_h
input.default = '.'

pp gen_new_state(input,6, -> v { neighbours3d(v) }).length
pp gen_new_state(input,6, -> v { neighbours4d(v) }).length
