require 'set'

Op=Struct.new(:name, :value) do
  def corrupted?()
    name == :nop or name == :jmp
  end

  def fix!()
    case name
      in :nop
        self.name = :jmp
      in :jmp
        self.name = :nop
    end
  end
end
def parse(s)
  m=s.split(" ")
  Op.new(m[0].to_sym, m[1].to_i)
end

class Machine
  attr_accessor :acc, :pc

  def initialize()
    @acc = 0
    @pc = 0
    @seen = Set.new
  end

  def run_one(instr)
    if @seen.include?(@pc) then
      return :loop
    end
    if @pc < 0 or @pc >= instr.length then
      return :out
    end
    @seen.add(@pc)
    case instr[@pc]
    in {name: :nop, value: _}
      @pc += 1
    in {name: :acc, value: v}
      @acc += v
      @pc += 1
    in {name: :jmp, value: v}
      @pc += v
    end
  end

  def run(instr)
    loop do
      res=run_one(instr) 
      if res == :loop or res == :out then
        return res
      end
    end
  end
end
input = STDIN.readlines.map(&:strip).map{|l| parse(l)}
m=Machine.new
m.run(input)
pp m.acc

all=input.each_with_index.filter_map {|op, i| i if op.corrupted? }.map {|i| c=Marshal.load(Marshal.dump(input)); c[i].fix!; c}
pp all.filter_map{|i| m=Machine.new; r=m.run(i); m.acc if r==:out}[0]
