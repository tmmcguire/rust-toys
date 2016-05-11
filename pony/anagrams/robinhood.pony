use "collections"

// http://codecapsule.com/2013/11/11/robin-hood-hashing/
// http://codecapsule.com/2013/11/17/robin-hood-hashing-backward-shift-deletion/
// http://www.sebastiansylvan.com/post/robin-hood-hashing-should-be-your-default-hash-table-implementation/
// http://www.sebastiansylvan.com/post/more-on-robin-hood-hashing-2/
// http://www.pvk.ca/Blog/more_numerical_experiments_in_hashing.html

// -------------------------------------

class _Util
  fun div_ceil(n: USize, d: USize): USize =>
    (n / d) + (if (n % d) == 0 then 0 else 1 end)

class RHMap[K,V,H: HashFunction[K] val]
  // Current maximum load factor before resize: 3/4
  let _lf_numerator: USize
  let _lf_denominator: USize
  var _size: USize = 0              // number of elements in map
  var _valid: Array[U64]            // bitmap of valid entries
  var _hashes: Array[USize]         // pre-computed hash values for entries
  var _pairs: Array[((K,V) | None)] // map slots

  new create(prealloc: USize = 6, load_factor_numerator: USize = 4,
    load_factor_denominator: USize = 5)
  =>
    _lf_numerator = load_factor_numerator
    _lf_denominator = load_factor_denominator
    let len = ((prealloc * _lf_denominator) / _lf_numerator).ponyint_next_pow2().max(8)
    _valid = Array[U64].init(0, _Util.div_ceil(len, 64))
    _hashes = Array[USize].init(0, len)
    _pairs = Array[((K,V) | None)].init(None, len)

  fun size(): USize =>
    _size

  fun space(): USize =>
    (_pairs.size() * _lf_numerator) / _lf_denominator

  fun contains(k: box->K!): Bool =>
    (_, let found: Bool) = _search(k)
    found

  fun apply(k: box->K!): this->V ? =>
    (let i: USize, let found: Bool) = _search(k)
    if found then
      return (_pairs(i) as (K,V))._2 as this->V
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
    (let i: USize, let found: Bool) = _search(key)
    if found then
      let mask = _pairs.size() - 1
      (let key', let value') = (_pairs(i) = None) as (K^, V^)
      var j = i
      var k = (j + 1) and mask
      let stop = _stop((i + 1) and mask)
      while k != stop do
        _pairs(j) = _pairs(k) = None
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

  fun index(i: USize): (this->K, this->V) ? =>
    if _is_valid(i) then
      _pairs(i) as (this->K, this->V)
    else
      error
    end

  fun keys(): RHKeys[K, V, H, this->RHMap[K, V, H]]^ =>
    """
    Returns an iterator over the keys of the map.

    This iterator will be invalid if the map is modified.
    """
    RHKeys[K, V, H, this->RHMap[K, V, H]](this)

  fun dib(current: USize, initial: USize): USize =>
    if current >= initial then
      current - initial
    else
      (_pairs.size() + current) - initial
    end

  fun _is_valid(i: USize): Bool ? =>
    let one: U64 = 1
    (_valid(i / 64) and (one << (i.u64() % 64))) > 0

  fun ref _toggle_valid(i: USize) ? =>
    let one: U64 = 1
    _valid(i / 64) = _valid(i / 64) xor (one << (i.u64() % 64))

  fun _search(key: box->K!): (USize, Bool) =>
    let mask = _pairs.size() - 1
    let hash = H.hash(key).usize()
    let bucket = hash and mask
    var i = bucket
    var found = false
    try
      repeat
        if not _is_valid(i) then
          break
        elseif dib(i, _hashes(i) and mask) < dib(i, bucket) then
          // This branch could be last, but putting it here is safe because the
          // DIB of this element for will be equal to the hypothetical DIB if
          // this is the target. Making the test here should be (a bit) faster
          // since we don't have to look at _keys.
          break
        elseif _hashes(i) == hash then
          let pair = _pairs(i) as (this->K, this->V)
          if H.eq(key, pair._1) then
            found = true
            break
          end
        end
        i = (i + 1) and mask
      until i == bucket end
    end
    (i, found)

  fun ref _update(key: K, value: V, hash: USize): (V^ | None) ? =>
    let mask = _pairs.size() - 1
    var pair: (K, V) = (consume key, consume value)
    var cur_hash = hash
    var bucket = cur_hash and mask
    var i = bucket
    repeat
      if not _is_valid(i) then
        _toggle_valid(i)
        _pairs(i) = consume pair
        _hashes(i) = cur_hash
        _size = _size + 1
        if _size > space() then
          _resize()
        end
        return None
      end
      if _hashes(i) == cur_hash then
        if H.eq((_pairs(i) as (K, V))._1, pair._1) then
          pair = (_pairs(i) = consume pair) as (K^, V^)
          _hashes(i) = cur_hash
          return pair._2 as V^
        end
      end
      if dib(i, _hashes(i) and mask) < dib(i, bucket) then
        pair = (_pairs(i) = consume pair) as (K^, V^)
        cur_hash = _hashes(i) = cur_hash
        bucket = cur_hash and mask
      end
      i = (i + 1) and mask
    until i == bucket end
    None

  fun ref _resize() =>
    try
      // Phase 1: create a new, bigger map and insert all the elements from this
      // map.
      var new_map = RHMap[K,V,H].create( (_size - 1) * 2 )
      for i in Range(0, _pairs.size()) do
        if _is_valid(i) then
          (let k, let v) = (_pairs(i) = None) as (K^, V^)
          new_map._update(consume k, consume v, _hashes(i))
        end
      end
      // Phase 2: transplant the new map's guts into this map.
      _size = new_map._size = _size
      _valid = new_map._valid = _valid
      _pairs = new_map._pairs = _pairs
      _hashes = new_map._hashes = _hashes
    end

  fun _stop(initial: USize): USize ? =>
    let mask = _pairs.size() - 1
    var i = initial
    repeat
      if not _is_valid(i) then
        return i
      elseif dib(i, _hashes(i) and mask) == 0 then
        return i
      end
      i = (i + 1) and mask
    until i == initial end
    error

class RHKeys[K, V, H: HashFunction[K] val, M: RHMap[K,V,H] #read] is
  Iterator[M->K]
  """
  An iterator over the keys of the map.
  """
  let _map: M
  var _i: USize = 0
  var _count: USize = 0

  new create(map: M) =>
    _map = map

  fun has_next(): Bool =>
    _count < _map.size()

  fun ref next(): M->K ? =>
    if _count >= _map.size() then
      error
    end
    var i = _i
    while not _map._is_valid(i) do
      i = i + 1
    end
    _i = i + 1
    _count = _count + 1
    _map.index(i) as (M->K, _)
