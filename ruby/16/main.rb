require 'set'

Typ=Struct.new(:name, :a, :b) do
  def matches(v) = a.member?(v) || b.member?(v)
end
def typ(v) = Typ.new(v[0], (v[1].to_i..v[2].to_i), (v[3].to_i..v[4].to_i))
def any(classes, v) = classes.any? {|c| c.matches(v)}
def valid_classes_for(v, classes) = classes.filter_map {|c| c.name if c.matches(v) }.to_set

classes, your, other = STDIN.readlines
  .map {|l| l.strip}
  .chunk_while {|l| !l.empty? }.to_a
classes = classes.reject(&:empty?).map {|c| typ(c.scan(/(.*): (.*)-(.*) or (.*)-(.*)/)[0])}
other=other[1..].map{|l| l.split(",").map(&:to_i)}.to_a
pp other.flat_map{|t| t.reject{|v| any(classes, v)}}.sum
valid=other.reject {|t| t.any?{|v| !any(classes, v)}}
your = your[1].split(",").map(&:to_i)
valid << your
candidates = valid.map{|t| t.map{|v| valid_classes_for(v, classes)}}
candidates = (0..classes.length-1).map {|i| [i, candidates.map{|c|c[i]}.reduce(:&)] }.sort_by {|_,e| e.length }

known = {}
while !candidates.empty?
  name=candidates[0][1].to_a.sample
  id=candidates[0][0]
  candidates.delete_at(0)
  candidates.each {|_, s| s.delete(name) }
  known[id]=name
end
pp known.filter_map {|id,name| your[id] if name.start_with?("departure")}.reduce(:*)
