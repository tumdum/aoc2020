Rule=Struct.new(:min, :max, :char) do 
  def valid?(s)
    hist = s.chars.group_by {|v| v}.map {|e| [e[0], e[1].length] }.to_h
    hist.default = 0
    min <= hist[char] and max >= hist[char]
  end

  def valid2?(s)
    (s[min-1] == char) ^ (s[max-1] == char)
  end
end

def parse(line)
  m=/(.*)-(.*) (.): (.*)/.match(line.strip)
  r=Rule.new(m[1].to_i, m[2].to_i, m[3])
  [r, m[4]]
end

input=STDIN.readlines.map {|l| parse(l) }
p input.count {|e| e[0].valid?(e[1]) }
p input.count {|e| e[0].valid2?(e[1]) }
