class Lib2
  """
  A bag of sorting functionality, 2nd edition: using a comparison function.
  """

  fun sort[T: Comparable[T] val](a: Array[box->T] ref) =>
    """
    Sort the array a using the natural comparison between elements.
    """
    sort_with[T](lambda(l: box->T!, r: box->T!): Compare => l.compare(r) end, a)

  fun sort_with[T](cmp: {(box->T!, box->T!): Compare} val, a: Array[box->T] ref) =>
    """
    Sort the array a using a comparison function cmp,
    which takes two elements of a and returns Less | Equal| Greater.
    """
    try _quicksort[T](cmp, a, 0, a.size() - 1) end

  fun _quicksort[T](cmp: {(box->T!, box->T!): Compare} val, a: Array[box->T] ref, left: USize val, right: USize val) ? =>
    """
    Recursive Quicksort implementation using a median-of-three pivot value and
    the Bentley-McIlroy three-way partitioning function. (For arrays with large
    numbers of duplicate keys, three-way partitioning *may* reduce the running
    time of the sort to linear.)

    For reference, see *Algorithms* by Robert Sedgewick and Kevin Wayne or the
    following:

    * http://algs4.cs.princeton.edu/23quicksort/

    * http://algs4.cs.princeton.edu/lectures/23Quicksort.pdf

    * https://www.cs.princeton.edu/~rs/talks/QuicksortIsOptimal.pdf

    * http://algs4.cs.princeton.edu/23quicksort/QuickX.java.html (An optimized
    version using B-M partitioning, Tukey's ninther (pivoting on a
    median-of-nine), and a cutoff to insertion sort.)
    """
    let pivot = _find_pivot[T](cmp, a, left, right)
    (let i, let j) = _bentley_mcilroy[T](cmp, a, pivot, left, right)
    if left < j then
      _quicksort[T](cmp, a, left, j)
    end
    if i < right then
      _quicksort[T](cmp, a, i, right)
    end

  fun _find_pivot[T](cmp: {(box->T!, box->T!): Compare} val, ary: Array[box->T] box, left: USize val, right: USize val): box->T ? =>
    """
    Return the median-of-three values in the array: the first element, the
    midpoint, and the last element. Ideally, this avoids the O(n^2) behavior of
    simple Quicksort.
    """
    let mid = (left + right) / 2
    let a = ary(left)
    let b = ary(right)
    let median = _max[box->T](cmp, _min[box->T](cmp, a, b), _min[box->T](cmp, ary(mid), _max[box->T](cmp, a, b)))
    median

  fun _min[U](cmp: {(U!,U!): Compare} val, a: U, b: U): U =>
    match cmp(a, b)
    | Less => a
    | Greater => b
    else
      b
    end

  fun _max[U](cmp: {(U!,U!): Compare} val, a: U, b: U): U =>
    match cmp(a, b)
    | Less => b
    | Greater => a
    else
      a
    end

  fun _bentley_mcilroy[T](cmp: {(box->T!,box->T!): Compare} val, a: Array[box->T] ref, pivot: box->T, left: USize val, right: USize val): (USize val, USize val) ? =>
    """
    Rearrange the elements of the array a so that all of the elements between
    left and j are less than the pivot value, all of the elements between j and
    i are equal to the pivot, and all of the elements between i and right are
    greater than the pivot. Separating out the equal elements allows Quicksort
    to avoid recursing on them.

    For more information:

    * http://algs4.cs.princeton.edu/lectures/23DemoPartitioning.pdf
    """
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
