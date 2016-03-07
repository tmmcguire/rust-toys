
class Lib

  fun sort[T: Comparable[T] val](a: Array[T] ref) =>
    try _quicksort[T](a, 0, a.size() - 1) end

  fun _quicksort
      [T: Comparable[T] val]
      (a: Array[T] ref, left: USize val, right: USize val) ? =>
    let pivot = _find_pivot[T](a, left, right)
    (let i, let j) = _bentley_mcilroy[T](a, pivot, left, right)
    if left < j then
      _quicksort[T](a, left, j)
    end
    if i < right then
      _quicksort[T](a, i, right)
    end

  fun _min[T: Comparable[T] val](a: T, b: T): T =>
    if a < b then a else b end

  fun _max[T: Comparable[T] val](a: T, b: T): T =>
    if a < b then b else a end

  fun _find_pivot[T: Comparable[T] val](ary: Array[T] box, left: USize val, right: USize val): T ? =>
    let mid = (left + right) / 2
    let a = ary(left)
    let b = ary(right)
    let median = _max[T](_min[T](a,b), _min[T](ary(mid), _max[T](a,b)))
    median

  fun _bentley_mcilroy[T: Comparable[T] val](a: Array[T] ref, pivot: T, left: USize val, right: USize val): (USize val, USize val) ? =>
    var p = left
    var i = left
    var j = right
    var q = right
    while i < j do
      while (i < j) and (a(i) <= pivot) do
        if a(i) == pivot then
          a(i) = a(p) = a(i)
          p = p + 1
        end
        i = i + 1
      end
      while (i < j) and (pivot <= a(j)) do
        if a(j) == pivot then
          a(j) = a(q) = a(j)
          q = q - 1
        end
        j = j - 1
      end
      if i < j then
        a(i) = a(j) = a(i)
      end
    end
    // (i == j) and (All x : l <= x < p : a(x) == pivot)
    // and (All x : p <= x < i : a(x) < pivot)
    // and symmetrically for j/q
    if (a(i) <= pivot) and (i < right) then
      i = i + 1
    end
    if (pivot <= a(j)) and (left < j) then
      j = j - 1
    end
    if p <= j then
      while left < p do
        p = p - 1
        a(p) = a(j) = a(p)
        j = j - 1
      end
    else
      j = left
    end
    if i <= q then
      while q < right do
        q = q + 1
        a(q) = a(i) = a(q)
        i = i + 1
      end
    else
      i = right
    end
    (i,j)
