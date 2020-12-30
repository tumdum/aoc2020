require 'set'

Contents=Struct.new(:count, :name)

def parse(s)
  if s == "no other bags" then 
    []
  else 
    m=/([0-9]*) (.*) bag/.match(s)
    Contents.new(m[1].to_i, m[2])
  end
end

def inside(name, all)
  all[name].map{|n| n.count * inside(n.name, all)}.sum + 1
end

input=STDIN.readlines.map {|l| /(.*) bags? contain (.*)./.match(l.strip) }
  .map {|m| [m[1], m[2].split(", ").flat_map{|s| parse(s)}]}
  .to_h
input.default = []
invert=Hash.new {|h,k| h[k]=[]}
input.each do |k,v|
  v.each do |val|
    invert[val.name] << k
  end
end
seen=Set.new(["shiny gold"])
todo=Set.new(["shiny gold"])
while !todo.empty? do
  e=todo.to_a.sample
  todo.delete(e)
  invert[e].each do |n|
    if !seen.include?(n) then
      todo << n
      seen << n
    end
  end
end
pp seen.length-1
pp inside("shiny gold", input)-1
