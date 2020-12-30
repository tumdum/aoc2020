def solve(spoken, max)
  seen = spoken.each_with_index.map {|v,i| [v, [i+1]]}.to_h
  start=spoken.length+1
  (start..max).each {|i|
    last=spoken.last
    prev_seen=seen[last]
    if prev_seen.length==1
      spoken << 0
      seen[0] << i
    else
      apart=prev_seen.last-prev_seen[-2]
      spoken << apart
      if seen.has_key?(apart)
        seen[apart] << i
      else 
        seen[apart] = [i]
      end
    end
  }
  spoken.last
end
p solve([18,8,0,5,4,1,20], 2020)
p solve([18,8,0,5,4,1,20], 30000000)
