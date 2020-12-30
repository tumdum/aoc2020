def find(start, step, ids)
  current = start
  loop do
    if ids.all?{|val, offset| (current + offset) % val == 0}
      return [current, ids.map{|val, offset| val}.reduce(1, :lcm)]
    end
    current += step
  end
end

start, timetable = STDIN.readlines.map(&:strip)
start=start.to_i
p timetable.split(",").map(&:to_i).reject{|e|e==0}.map{|e| [e, ((start/e)+1)*e-start]}.min_by {|_,v| v}.reduce(:*)
ids=timetable.split(",").map(&:to_i).each_with_index.reject{|v,_| v == 0}

offset=1
(0..ids.length-1).each {|n| start, offset = find(start, offset, ids[0..n])}
p start
