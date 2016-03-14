class Lib2

  fun sort[T: Comparable[T] val](a: Array[box->T] ref) =>
    sort_with[T](lambda(l: box->T!, r: box->T!): Compare => l.compare(r) end, a)

  fun sort_with[T](cmp: {(box->T!, box->T!): Compare} val, a: Array[box->T] ref) =>
    try _quicksort_with[T](cmp, a, 0, a.size() - 1) end

  fun _quicksort_with[T](cmp: {(box->T!, box->T!): Compare} val, a: Array[box->T] ref, left: USize val, right: USize val) ? =>
    let pivot = _find_pivot_with[T](cmp, a, left, right)
    (let i, let j) = _bentley_mcilroy_with[T](cmp, a, pivot, left, right)
    if left < j then
      _quicksort_with[T](cmp, a, left, j)
    end
    if i < right then
      _quicksort_with[T](cmp, a, i, right)
    end

  fun _find_pivot_with[T](cmp: {(box->T!, box->T!): Compare} val, ary: Array[box->T] box, left: USize val, right: USize val): box->T ? =>
    let mid = (left + right) / 2
    let a = ary(left)
    let b = ary(right)
    let median = _maxw[box->T](cmp, _minw[box->T](cmp, a, b), _minw[box->T](cmp, ary(mid), _maxw[box->T](cmp, a, b)))
    median

  fun _minw[U](cmp: {(U!,U!): Compare} val, a: U, b: U): U =>
    match cmp(a, b)
    | Less => a
    | Greater => b
    else
      b
    end

  fun _maxw[U](cmp: {(U!,U!): Compare} val, a: U, b: U): U =>
    match cmp(a, b)
    | Less => b
    | Greater => a
    else
      a
    end

  fun _bentley_mcilroy_with[T](cmp: {(box->T!,box->T!): Compare} val, a: Array[box->T] ref, pivot: box->T, left: USize val, right: USize val): (USize val, USize val) ? =>
    var p = left
    var i = left
    var j = right
    var q = right
    while i < j do
      var i' = cmp(a(i), pivot)
      while (i < j) and ((i' is Less) or (i' is Equal)) do
        if i' is Equal then
          a(i) = a(p) = a(i)
          p = p + 1
        end
        i = i + 1
        i' = cmp(a(i), pivot)
      end
      var j' = cmp(pivot, a(j))
      while (i < j) and ((j' is Less) or (j' is Equal)) do
        if j' is Equal then
          a(j) = a(q) = a(j)
          q = q - 1
        end
        j = j - 1
        j' = cmp(pivot, a(j))
      end
      if i < j then
        a(i) = a(j) = a(i)
      end
    end
    // (i == j) and (All x : l <= x < p : a(x) == pivot)
    // and (All x : p <= x < i : a(x) < pivot)
    // and symmetrically for j/q
    let i' = cmp(a(i), pivot)
    if ((i' is Less) or (i' is Equal)) and (i < right) then
      i = i + 1
    end
    let j' = cmp(pivot, a(j))
    if ((j' is Less) or (j' is Equal)) and (left < j) then
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
