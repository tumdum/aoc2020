require 'set'
Pos=Struct.new(:col, :row) do
  def neighbour(dir)
    case dir
      in :w 
      Pos.new(col-2, row)
      in :e 
      Pos.new(col+2, row)
      in :se 
      Pos.new(col+1, row+1)
      in :nw 
      Pos.new(col-1, row-1)
      in :sw 
      Pos.new(col-1, row+1)
      in :ne 
      Pos.new(col+1, row-1)
    end
  end

  def all_neighbours() = [:w, :e, :se, :nw, :sw, :ne].map{|d| self.neighbour(d)}

  def move_by(dirs) = dirs.reduce(self){|pos, dir| pos.neighbour(dir)}
end

def parse(s)
  ret = []
  while !s.empty?
    if s.start_with?("e")
      ret << :e
      s=s[1..]
    elsif s.start_with?("se")
      ret << :se
      s=s[2..]
    elsif s.start_with?("sw")
      ret << :sw
      s=s[2..]
    elsif s.start_with?("w")
      ret << :w
      s=s[1..]
    elsif s.start_with?("nw")
      ret << :nw
      s=s[2..]
    elsif s.start_with?("ne")
      ret << :ne
      s=s[2..]
    end
  end
  ret
end

def should_become_black(old, pos)
  black_neighbours = pos.all_neighbours().count{|n| old.include?(n)}
  is_black = old.include?(pos)
  if is_black
    if black_neighbours == 0 || black_neighbours > 2
      false
    else
      true
    end
  else
    black_neighbours == 2
  end
end

def next_day(old) = old.flat_map{|p| [p] + p.all_neighbours()}.filter{|p| should_become_black(old, p)}.to_set

black = STDIN.readlines.map(&:strip).map{|l| Pos.new(0,0).move_by(parse(l))}.group_by{|v| v}.map{|k,v| [k, v.length]}.filter_map{|k,v| k if v%2==1}.to_set

p black.length
p (1..100).reduce(black.dup) {|old, _| next_day(old)}.length
