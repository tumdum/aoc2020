require 'set'
def find_uniq(all)
  if r=all.find {|k,v| v.length == 1}
    [r[0],r[1].to_a.pop]
  end
end
input = STDIN.read.scan(/(.*) \(contains (.*)\)/).map{|l,r| [l.split(" "), r.split(", ")]}
allergens_to_ingredients = {}
input.each do |ingredients, allergens|
  allergens.each do |allergen|
    if allergens_to_ingredients.has_key?(allergen)
      allergens_to_ingredients[allergen] &= ingredients
    else
      allergens_to_ingredients[allergen] = ingredients.to_set
    end
  end
end
all =input.flat_map{|ingredients,_| ingredients}.to_set
candidates =allergens_to_ingredients.flat_map{|_,allergens| allergens.to_a}.to_set
safe=all-candidates
p input.map{|ing,_| (safe & ing).length}.sum
mapped=[]
loop do
  if r = find_uniq(allergens_to_ingredients)
    allergen, ingredient = r
    allergens_to_ingredients.delete(allergen)
    mapped << [allergen, ingredient]
    allergens_to_ingredients.each do |_,v|
      v.delete(ingredient)
    end
  else
    break
  end
end
pp mapped.sort_by{|al,ing|al}.map{|_,ing| ing}.join(",")
