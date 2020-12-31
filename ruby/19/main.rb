def parse(s)
  id, rest = s.split(": ")
  if m=rest.match(/"(.*)"/)
    return [id.to_i, [:lit, m[1]]]
  elsif m=rest.match(/(.*) \| (.*)/)
    return [id.to_i, [:alt, m[1].split(" ").map(&:to_i), m[2].split(" ").map(&:to_i)]]
  else
    return [id.to_i, [:seq] + rest.split(" ").map(&:to_i)]
  end
end

def match_seq(rule_ids, input, all_rules) = rule_ids.reduce([input]) {|acc, rule_id| acc.flat_map {|tail| match(rule_id, tail.dup, all_rules)}}

def match(rule_id, input, all_rules)
  r=all_rules[rule_id]
  case r[0]
    in :lit
    if input.start_with?(r[1])
      [input[r[1].length..].dup]
    else
      []
    end
    in :alt
      a = match_seq(r[1], input.dup, all_rules)
      b = match_seq(r[2], input.dup, all_rules)
      a + b
    in :seq
      match_seq(r[1..], input.dup, all_rules)
  end
end

def matches?(input, all_rules) = match(0, input, all_rules).any?(&:empty?)

rules, messages = STDIN.readlines
  .map(&:strip)
  .chunk_while {|l| !l.empty? }
  .to_a
rules=rules.reject(&:empty?).map{|l| parse(l)}.to_h

pp messages.count{|l| matches?(l, rules)}

rules[8] = [:alt , [42], [42,8]]
rules[11] = [:alt, [42,31], [42,11,31]]
pp messages.count{|l| matches?(l, rules)}
