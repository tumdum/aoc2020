require 'set'

Pos=Struct.new(:x, :y)

def pos(x, y)
  Pos.new(x, y)
end

trees=Set.new([])
$height=0
$width=0
STDIN.readlines.map {|l| l.strip }.each_with_index do |line, y| 
  $height = [$height, y+1].max
  line.chars.each_with_index do |char, x|
    $width = [$width, x+1].max
    trees.add(pos(x, y)) if char == '#'
  end
end

def visit(trees, slope)
  x = slope.x
  y = slope.y
  seen = 0
  while y < $height do
    tmp = pos(x, y)
    seen += 1 if trees.include?(tmp)
    x = (x + slope.x) % $width
    y += slope.y
  end
  seen
end

p visit(trees, pos(3, 1))
p [pos(1, 1), pos(3, 1), pos(5, 1), pos(7, 1), pos(1, 2)].map {|slope| visit(trees, slope)}.reduce(:*)
