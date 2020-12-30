input = STDIN.readlines
  .map {|l| l.strip}
  .chunk_while {|l| !l.empty? }
  .map{|e| e.join(" ").strip.split(" ").map {|l| m=/(.*):(.*)/.match(l); [m[1], m[2]]}.to_h }

valid={
  "byr"=> Proc.new {|n| (1920..2002).include?(n.to_i) },
  "iyr"=> Proc.new {|n| (2010..2020).include?(n.to_i) },
  "eyr"=> Proc.new {|n| (2020..2030).include?(n.to_i) },
  "hgt"=> Proc.new {|n| if n.end_with?("cm") then (150..193).include?(n.to_i) else (59..76).include?(n.to_i) end },
  "hcl"=> Proc.new {|n| /^#[0-9a-f]{6}$/.match? n },
  "ecl"=> Proc.new {|n| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].include? n },
  "pid"=> Proc.new {|n| /^[0-9]{9}$/.match? n}
}

p input.count {|h| valid.all? {|key,_| h.has_key?(key)}}
p input.count {|h| valid.all? {|key,val| val.(h.fetch(key,"")) } }
