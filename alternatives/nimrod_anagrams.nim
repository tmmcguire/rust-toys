import algorithm
import strutils
import tables
import sets
import os
import sequtils
import nimprof

iterator eachCombination[T](values : openarray[T], r : int) : seq[T] =
    let length = values.len
    # Little inconvenient that there is no way to break out here
    if r > 0 and r <= length:
      var max_indices0 = length - r
      # var indices : seq[int] = newSeq[int](r)
      # -> Error: type mismatch: got (seq[typedesc[int]]) but expected 'seq[int]'
      var indices : seq[int]
      var combination : seq[T]
      newSeq(indices, r)
      newSeq(combination, r)
      for i in 0..r-1:
        indices[i] = i
        combination[i] = values[i]
      while true:
        yield(combination)
        # increment the indices
        var i = r - 1
        indices[i] += 1
        while i > 0 and indices[i] > max_indices0 + i:
          # indices[i] now too large; decrement i, increment indices[i]
          # and we'll fix up the following indices later
          i -= 1
          indices[i] += 1
        # Can't fix up 'done'
        if indices[0] > max_indices0: break
        # Fix up the indices and the combination from i to r-1
        combination[i] = values[indices[i]]
        for i in i+1 .. r-1:
          indices[i] = indices[i-1] + 1
          combination[i] = values[indices[i]]

iterator allCombinations[T](values : openarray[T]) : seq[T] =
  for length in 2..values.len:
    for combo in eachCombination(values, length):
      yield(combo)

proc stringToSeq(str : string) : seq[char] =
  return toSeq(str.items)

proc seqToString(seq : openarray[char]) : string =
  result = newString(seq.len)
  for i,ch in seq: result[i] = ch
  return result

proc loadDictionary() : TTable[seq[char],seq[string]] =
  var result = initTable[seq[char],seq[string]]()
  for line in "anadict.txt".lines:
    var words = line.split
    result[stringToSeq(words[0])] = words[1..words.len - 1]
  return result

when isMainModule:
  if os.paramCount() == 1:
    var board = stringToSeq(os.paramStr(1))
    board.sort(system.cmp[char])
    var dictionary = loadDictionary()
    var result = initSet[string]()
    for combo in allCombinations(board):
      if dictionary.hasKey(combo):
        for word in dictionary[combo]:
          discard result.containsOrIncl(word)
    echo(result.len)

# asdwtribnowplfglewhqagnbe = 7440
