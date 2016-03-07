use "collections"
use "debug"

type ArrayHashMap is RHMap[Array[U32],Array[String],ArrayHashFcn[U32] val]

primitive ArrayHashFcn[T: (Hashable #read & Equatable[T] #read)] is HashFunction[Array[T]]
  fun hash(x: box->Array[T]!): U64 =>
    var h: U64 = 0
    for v in x.values() do
      h = h xor v.hash()
    end
    Debug(["h", h.string()])
    h

  fun eq(x: box->Array[T]!, y: box->Array[T]!): Bool =>
    try
      if x.size() == y.size() then
        var i: USize = 0
        while i < x.size() do
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
