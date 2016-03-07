use "collections"

class Combination[T]
  let values: Array[T] val
  let r: USize
  var done: Bool
  let max_indices0: USize
  let indices: Array[USize val] ref
  let combination: Array[val->T!]

  new create(values': Array[T] val, r': USize) =>
    values = values'
    r = r'
    let length = values'.size()
    done = (r == 0) or (r > length)
    max_indices0 = length - r
    indices = Array[USize](r)
    combination = Array[val->T!](r)
    if not done then
      var i: USize = 0
      while i < r do
        indices.push(i)
        try combination.push(values'(i)) end
        i = i + 1
      end
    end

  fun has_next(): Bool =>
    not done

  fun ref next(): Array[val->T!] ? =>
    let combination' = combination.slice()
    var i = r - 1
    indices(i) = indices(i) + 1
    while (i > 0) and (indices(i) > (max_indices0 + i)) do
      i = i - 1
      indices(i) = indices(i) + 1
    end
    if indices(0) <= max_indices0 then
      combination(i) = values( indices(i) )
      i = i + 1
      while i < r do
        indices(i) = indices(i-1) + 1
        combination(i) = values( indices(i) )
        i = i + 1
      end
    else
      done = true
    end
    combination'

trait Fn[T]
  fun ref apply(combination: Array[val->T!])?

class EachCombination

  fun apply[T](values: Array[T] val, r: USize, fn: Fn[T] ref) ? =>
    let length = values.size()
    if (r == 0) or (r > length) then
      return
    end
    let max_indices0 = length - r
    var indices: Array[USize] = Array[USize](r)
    for i in Range(0, r) do
      indices.push(i)
    end
    var combination: Array[val->T!] = Array[val->T!](r)
    for i in Range(0, r) do
      combination.push( values(i) )
    end
    while true do
      fn(combination)
      // Increment the indices
      var i = r - 1
      indices(i) = indices(i) + 1
      while (i > 0) and (indices(i) > (max_indices0 + i)) do
        // indices(i) is now too large; decrement i, increment the new
        // indices(i), and we'll fix up the following indices later
        i = i - 1
        indices(i) = indices(i) + 1
      end
      if indices(0) > max_indices0 then
        // can't fix up done
        break
      end
      // fix up the indices and combination from i to r-1
      combination(i) = values( indices(i) )
      for j in Range(i+1, r) do
        indices(j) = indices(j-1) + 1
        combination(j) = values( indices(j) )
      end
    end
