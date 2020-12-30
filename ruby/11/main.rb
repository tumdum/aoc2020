require 'matrix'

$adjecent=[Vector[1,0],Vector[-1,0],Vector[0,1],Vector[0,-1],Vector[1,1],Vector[1,-1],Vector[-1,-1],Vector[-1,1]]
map={}
STDIN.readlines.map(&:strip).each_with_index do |line,y|
  line.chars.each_with_index do |c,x|
    map[Vector[x,y]] = c
  end
end

def occupied(map, pos)
  $adjecent.filter_map{|d|map[pos+d]}.count{|c| c=='#'}
end

def occupied_in_dir(map, current, dir)
  loop do
    current=current+dir
    v=map[current]
    if v == nil || v == 'L'
      return false
    elsif v == '#'
      return true
    end
  end
end

def occupied_distant(map, pos)
  $adjecent.count{|d| occupied_in_dir(map, pos, d)}
end

def next_val(old, pos, val, threshold, count)
  if val == 'L' && count.call(old, pos) == 0
    '#'
  elsif val == '#' && count.call(old, pos) >= threshold
    'L'
  else
    val
  end
end

def round(old, threshold, count)
  old.map{|pos,val| [pos, next_val(old, pos, val, threshold, count)]}.to_h
end

def find_stable(start,threshold, count)
  old = start
  loop do
    new=round(old,threshold, count)
    if new == old
      return new
    end
    old=new
  end
end

p find_stable(map, 4, -> m, p { occupied(m, p)}).count{|_,v| v=='#'}
p find_stable(map, 5, -> m, p { occupied_distant(m, p)}).count{|_,v| v=='#'}
