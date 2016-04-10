use "collections"

primitive HashSeq[T: (Hashable #read & Equatable[T] val)] is HashFunction[Seq[T] box]
  """
  Hash and equality functions for arbitrary hashable and equatable collections.
  """
  fun hash(x: Seq[T] box): U64 =>
    var h = U64(1)
    var i = USize(0)
    while i < x.size() do
      h = (31 * h) + try x.apply(i).hash() else 1 end
      i = i + 1
    end
    h

  fun eq(x: Seq[T] box, y: Seq[T] box): Bool =>
    try
      if x.size() == y.size() then
        let sz = x.size()
        var i = USize(0)
        while i < sz do
          if x(i) != y(i) then
            return false
          end
          i = i + 1
        end
        return true
      else
        return false
      end
    else
      return false
    end
