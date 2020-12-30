require 'set'

$M  = 0b00000000_00000000_00000000_00001111_11111111_11111111_11111111_11111111

def parse_mask(m) = m.chars.each_with_index.reject{|c,_|c=="X"}.map{|v,i|[35-i, v.to_i]}.to_h

def set_bit(v, bit) = v | 1 << bit

def unset_bit(v, bit) = v & ($M & ~(1 << bit))

def write(mem, addr, mask, value) = mem[addr] = apply(mask, value)

def write2(mem, addr, mask, value) = all_addresse(addr, mask).each { |addr| mem[addr] = value }

def all_addresse(addr, mask)
  mask.each {|bit, b| addr = set_bit(addr, bit) if b == 1 }
  ((0..35).to_set - mask.keys).reduce([addr]) { |acc, n| acc.flat_map {|v| [set_bit(v, n), unset_bit(v, n)]} }
end

def apply(mask, value)
  mask.each do |bit, b|
    if b == 1
      value = set_bit(value, bit)
    else
      value = unset_bit(value, bit)
    end
  end
  value
end

input = STDIN.read.scan(/(.*) = ([a-zA-Z0-9]*)/)
  .map {|l| if l[0] == "mask" then [l[0], parse_mask(l[1])] else [l[0].scan(/mem\[(.*)\]/)[0][0].to_i, l[1].to_i] end }

mem = {}
mask = 0

input.each do |type, value|
  if type=="mask"
    mask=value
  else
    write(mem, type, mask, value)
  end
end

pp mem.values.sum

mem = {}
mask = 0

input.each do |type, value|
  if type=="mask"
    mask=value
  else
    write2(mem, type, mask, value)
  end
end

pp mem.values.sum
