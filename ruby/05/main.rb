require 'set'

def walk(chars, min, max)
  chars.chars.each do |c|
    if c == 'F' or c == 'L' then
      max = (min + max)/2
    else
      min = (min + max)/2 + 1
    end
  end
  min
end

def parse(s)
  walk(s[..-4], 0, 127) * 8 + walk(s[7..], 0, 7)
end

seats = STDIN.readlines.map {|l| parse(l.strip)}.to_set
min, max = seats.minmax
p max
p (min+1..max-1).find {|n| !seats.member? n and seats.member? n-1 and seats.member? n+1 }
