require 'matrix'

$dirs={:N=>Vector[0,1], :S => Vector[0,-1], :E => Vector[1,0], :W => Vector[-1,0]}
$left={:N=>:W, :W=>:S, :S=>:E, :E=>:N}

ShipWaypoint=Struct.new(:pos, :waypoint) do
  def apply(op)
    case op[0]
    when :F
      ShipWaypoint.new(pos+waypoint*op[1], waypoint)
    when :N, :E, :W, :S
      ShipWaypoint.new(pos, waypoint+$dirs[op[0]]*op[1])
    when :L, :R
      ShipWaypoint.new(pos, rotate_vec(waypoint, op[0], op[1]))
    end
  end
end

Ship=Struct.new(:pos, :dir) do
  def apply(op)
    case op[0]
    when :F
      Ship.new(pos+$dirs[dir]*op[1], dir)
    when :N, :E, :W, :S
      Ship.new(pos+$dirs[op[0]]*op[1], dir)
    when :L, :R
      Ship.new(pos, rotate(dir, op[0], op[1]))
    end
  end
end

def rotate_vec_counter(v)
  Vector[-v[1], v[0]]
end

def rotate_vec(v, dir, deg)
  if deg == 0
    return v
  end
  if dir == :L
    rotate_vec(rotate_vec_counter(v), dir, deg-90)
  else
    rotate_vec(v, :L, 360-deg)
  end
end

def rotate_left(dir, times)
  (1..times).each do
    dir = $left[dir]
  end
  dir
end

def rotate(dir, side, deg)
  times=deg/90
  if side == :L
    rotate_left(dir, times)
  else
    rotate_left(dir, 4-times)
  end
end

def apply_all(ops, start)
  ret=start.clone
  ops.each do |op|
    new=ret.apply(op)
    ret = new
  end
  ret
end

input=STDIN.readlines.map(&:strip).map{|l| [l[0].to_sym, l[1..].to_i]}
start=Ship.new(Vector[0,0], :E)
p apply_all(input, start).pos.map(&:abs).sum
start=ShipWaypoint.new(Vector[0,0], Vector[10, 1])
p apply_all(input, start).pos.map(&:abs).sum
