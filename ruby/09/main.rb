def valid(l)
  l.last if !l[..-2].product(l[..-2]).find{|a,b| a!=b and a+b==l.last}
end
input = STDIN.readlines.map(&:strip).map(&:to_i).to_a
part_a = pp input.each_cons(25+1).find{|l| valid(l)}.last
l=2
a=[]
(2..).each do |l|
  if a=input.each_cons(l).find {|l| l.sum == part_a } then
    break
  end
end
p a.minmax.sum
