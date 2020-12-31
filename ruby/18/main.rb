def eval_op(a, op, b) = if op == "+" then a.to_i + b.to_i else a.to_i * b.to_i end

def eval(input)
  stack=[[]]
  while !input.empty?
    v=input.delete_at 0
    case v
    in "("
      stack << []
    in ")"
      tmp=stack.pop.pop
      stack.last << tmp
    else
      stack.last << v
    end
    if stack.last.length >= 3
      a, op, b = stack.last.pop(3)
      stack.last << eval_op(a, op, b)
    end
  end
  stack.pop.pop
end

def eval_add_first(l)
  loop do
    i = l.index("+")
    break if !i
    b = l.delete_at(i+1)
    op = l.delete_at(i)
    a = l.delete_at(i-1)
    l.insert(i-1, a.to_i + b.to_i)
  end
  l.reject {|e| e =="*"}.map(&:to_i).reduce(:*)
end

def eval2(input)
  stack=[[]]
  while !input.empty?
    v=input.delete_at 0
    case v
    in "("
      stack << []
    in ")"
      tmp=eval_add_first(stack.pop)
      stack.last << tmp
    else
      stack.last << v
    end
  end
  eval_add_first(stack.pop)
end

input=STDIN.readlines.map(&:strip).map{|l|l.gsub("(", " ( ").gsub(")", " ) ").split(" ")}
p input.map{|l| eval(l.dup)}.sum
p input.map{|l| eval2(l.dup)}.sum
