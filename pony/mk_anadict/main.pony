use "collections"
use "debug"
use "files"

interface box FoldFn[T,U]
  fun apply(t: T!, u: U!): U!

class Lib

  fun fold[T,U](iter: Iterator[T] ref, fn: FoldFn[T,U] box, u: U!): U! =>
    for t in iter do
      fn(t, u)
    end
    u

  fun sort[T: (Comparable[T] val & Stringable)](a: Array[T] ref) =>
    try _quicksort[T](a, 0, a.size() - 1) end

  fun _quicksort
      [T: (Comparable[T] val & Stringable)]
      (a: Array[T] ref, left: USize val, right: USize val) ? =>
    let pivot = _find_pivot[T](a, left, right)
    // (let i, let j) = _partition[T](a, pivot, left, right)
    // (let i, let j) = _dijkstra[T](a, pivot, left, right)
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

  fun _partition[T: Comparable[T] val](a: Array[T] ref, pivot: T, left: USize val, right: USize val): (USize val, USize val) ? =>
    var i = left
    var j = right
    while i < j do
      while (i < right) and (a(i) < pivot) do
        i = i + 1
      end
      while (left < j) and (a(j) > pivot) do
        j = j - 1
      end
      if i <= j then
        if i < j then
          a(i) = a(j) = a(i)
        end
        if i < right then
          i = i + 1
        end
        if left < j then
          j = j - 1
        end
      end
    end
    (i,j)

  fun _dijkstra[T: Comparable[T] val](a: Array[T] ref, pivot: T, left: USize val, right: USize val): (USize val, USize val) ? =>
    var lt = left
    var i = left
    var gt = right
    while i <= gt do
      if (i <= right) and (a(i) < pivot) then
        a(i) = a(lt) = a(i)
        lt = lt + 1
        i = i + 1
      elseif (i <= right) and (a(i) > pivot) then
        a(i) = a(gt) = a(i)
        gt = gt - 1
      else
        i = i + 1
      end
    end
    (right.min(gt + 1), left.max(lt - 1))

  fun d[T: Stringable val](a: Array[T] box, lo: USize val, hi: USize val) ? =>
    var i = lo
    while i <= hi do
      Debug([" ", i, a(i)])
      i = i + 1
    end

  fun _bentley_mcilroy[T: (Comparable[T] val & Stringable)](a: Array[T] ref, pivot: T, left: USize val, right: USize val): (USize val, USize val) ? =>
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

type Dictionary is Map[String,Array[String]]

actor Main
  let _env: Env

  new create(env: Env) =>
    _env = env
    if env.args.size() < 3 then
      env.out.print("Usage: mk_anadict words anadict")
      return
    end
    let words: String val = try env.args(1) else "/usr/share/dict/words" end
    let anadict: String val = try env.args(2) else "anadict.txt" end

    let m: Dictionary box = try
      _build(words)
    else
      env.err.write("cannot read words file: ")
      env.err.print(words)
      return
    end
    try
      _write(anadict, m)
    else
      env.err.write("cannot write anadict file: ")
      env.err.print(anadict)
      return
    end
    // let s = "aardvark"
    // let s' = Array[U32](s.size()).concat( s.runes() )
    // try Lib.d[U32](s', 0, s'.size() - 1) end
    // Debug.out("")
    // Lib.sort[U32](s')
    // Debug.out("")
    // try Lib.d[U32](s', 0, s'.size() - 1) end

  fun _build(path: String): Dictionary? =>
    let caps: FileCaps val = recover val FileCaps.set(FileRead).set(FileStat) end
    var file: (None val | File ref) = None
    try
      file = File.open( FilePath(_env.root, path, caps) )
      let f: FoldFn[String,Dictionary] val =
        lambda(s: String val, n: Dictionary ref): Dictionary ref => DictHelper.to_map(s,n) end
      Lib.fold[String,Dictionary]((file as File).lines(), f,  Dictionary())
    else
      error
    then
      if file isnt None then (file as File).dispose() end
    end

  fun _write(path: String, m: Dictionary box) ? =>
    let caps: FileCaps val = recover val FileCaps.set(FileRead).set(FileCreate).set(FileTruncate).set(FileWrite) end
    var file: (None val | File ref) = None
    try
      file = File.create( FilePath(_env.root, path, caps) )
      if (file as File).errno() isnt FileOK then error end
      let file' = file as File
      let keys: Array[String] = Array[String].create().concat( m.keys() )
      Lib.sort[String](keys)
      for key in keys.values() do
        let values = try " ".join( m(key) ) else "" end
        if not ( file'.write(key) ) then
          error
        end
        if not ( file'.write(" ") ) then
          error
        end
        if not ( file'.print(values) ) then
          error
        end
      end
    else
      error
    then
      if file isnt None then (file as File).dispose() end
    end

class DictHelper

  fun to_map(s: String, n: Dictionary): Dictionary =>
    if (s.size() >= 2) and (s.size() < 19) and (is_lowercase_ascii(s)) then
      Debug(["s", s])
      let a = Array[U32](s.size())
      a.concat( s.runes() )
      Lib.sort[U32](a)
      var s' = recover val String end
      for ch in a.values() do
        s' = s' + String.from_utf32(ch)
      end
      n(s') = try n(s').push(s) else Array[String].init(s,1) end
    end
    n

  fun to_string(iter: Iterator[U32]): String =>
    var s = recover val String end
    for ch in iter do
      s = s + String.from_utf32(ch)
    end
    s

  fun is_lowercase_ascii(s: String): Bool =>
    var i: USize = 0
    while i < s.size() do
      try
        if (s(i) < 0x61) or (s(i) > 0x7a) then
          return false
        end
        i = i + 1
      else
        return false
      end
    end
    true
