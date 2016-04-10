use "debug"
use "collections"

// http://codecapsule.com/2013/11/11/robin-hood-hashing/
// http://codecapsule.com/2013/11/17/robin-hood-hashing-backward-shift-deletion/
// http://www.sebastiansylvan.com/post/robin-hood-hashing-should-be-your-default-hash-table-implementation/
// http://www.sebastiansylvan.com/post/more-on-robin-hood-hashing-2/
// http://www.pvk.ca/Blog/more_numerical_experiments_in_hashing.html

// -------------------------------------

class Util
  fun div(n: USize, d: USize): USize =>
    (n / d) + (if (n % d) == 0 then 0 else 1 end)

class RHMap[K,V,H: HashFunction[K] val]
  let _lf_numerator: USize = 9
  let _lf_denominator: USize = 10
  var _size: USize = 0
  var _valid: Array[U64]
  var _hashes: Array[USize]
  var _keys: Array[(K | None)]
  var _values: Array[(V | None)]

  new create(prealloc: USize = 6) =>
    let len = ((prealloc * _lf_denominator) / _lf_numerator).ponyint_next_pow2().max(8)
    _valid = Array[U64].init(0, Util.div(len, 64))
    _keys = Array[(K|None)].init(None, len)
    _values = Array[(V|None)].init(None, len)
    _hashes = Array[USize].init(0, len)

  fun size(): USize =>
    _size

  fun count(): USize ? =>
    var c: USize = 0
    for i in Range(0, _keys.size()) do
      if _is_valid(i) then
        c = c + 1
      end
    end
    c

  fun space(): USize =>
    (_keys.size() * _lf_numerator) / _lf_denominator

  fun contains(k: box->K!): Bool =>
    (_, let found: Bool) = _search(k)
    found

  fun apply(k: box->K!): this->V ? =>
    (let i: USize, let found: Bool) = _search(k)
    if found then
      return _values(i) as this->V
    else
      error
    end

  fun ref update(key: K, value: V): (V^ | None) =>
    let hash = H.hash(key).usize()
    try _update(consume key, consume value, hash) end

  fun ref insert(key: K, value: V): V ? =>
    let k = key
    this(consume key) = consume value
    this(k)

  fun ref remove(key: box->K!): (K^, V^) ? =>
    let mask = _keys.size() - 1
    (let i: USize, let found: Bool) = _search(key)
    if found then
      let stop = _stop(i)
      let key' = (_keys(i) = None) as K^
      let value' = (_values(i) = None) as V^
      var j = i
      var k = (j + 1) and mask
      while k != stop do
        _keys(j) = _keys(k) = None
        _values(j) = _values(k) = None
        _hashes(j) = _hashes(k)
        j = k
        k = (k + 1) and mask
      end
      _toggle_valid(j)
      _size = _size - 1
      (consume key', consume value')
    else
      error
    end

  fun dib(current: USize, initial: USize): USize =>
    if current >= initial then
      current - initial
    else
      (_keys.size() + current) - initial
    end

  fun _is_valid(i: USize): Bool ? =>
    let one: U64 = 1
    (_valid(i / 64) and (one << (i.u64() % 64))) > 0

  fun ref _toggle_valid(i: USize) ? =>
    let one: U64 = 1
    _valid(i / 64) = _valid(i / 64) xor (one << (i.u64() % 64))

  fun _search(key: box->K!): (USize, Bool) =>
    let mask = _keys.size() - 1
    let hash = H.hash(key).usize()
    let bucket = hash and mask
    var i = bucket
    var found = false
    try
      repeat
        if not _is_valid(i) then
          break
        elseif dib(i, _hashes(i) and mask) < dib(i, bucket) then
          break
        elseif (_hashes(i) == hash) and (H.eq(key, _keys(i) as K)) then
          found = true
          break
        end
        i = (i + 1) and mask
      until i == bucket end
    end
    (i, found)

  fun ref _update(key: K, value: V, hash: USize): (V^ | None) ? =>
    let mask = _keys.size() - 1
    var k = consume key
    var v = consume value
    var hash' = hash
    var bucket = hash' and mask
    var i = bucket
    repeat
      if not _is_valid(i) then
        _toggle_valid(i)
        _keys(i) = consume k
        _values(i) = consume v
        _hashes(i) = hash'
        _size = _size + 1
        if _size > space() then
          _resize()
        end
        return None
      else
        if (_hashes(i) == hash') and H.eq(_keys(i) as K, k) then
          _keys(i) = consume k
          let value' = _values(i) = consume v
          _hashes(i) = hash'
          return value' as V^
        elseif dib(i, _hashes(i) and mask) < dib(i, bucket) then
          k = (_keys(i) = consume k) as K^
          v = (_values(i) = consume v) as V^
          hash' = _hashes(i) = hash'
          bucket = hash' and mask
        end
      end
      i = (i + 1) and mask
    until i == bucket end
    None

  fun ref _resize() =>
    try
      var new_map = RHMap[K,V,H].create(_size * 2)
      for i in Range(0, _keys.size()) do
        if _is_valid(i) then
          let k = (_keys(i) = None) as K^
          let v = (_values(i) = None) as V^
          new_map._update(consume k, consume v, _hashes(i))
        end
      end
      _size = new_map._size = _size
      _valid = new_map._valid = _valid
      _keys = new_map._keys = _keys
      _values = new_map._values = _values
      _hashes = new_map._hashes = _hashes
    end

  fun _stop(initial: USize): USize ? =>
    let mask = _keys.size() - 1
    var i = (initial + 1) and mask
    repeat
      if not _is_valid(i) then
        return i
      elseif dib(i, _hashes(i) and mask) == 0 then
        return i
      end
      i = (i + 1) and mask
    until i == initial end
    error
