import streams
import strutils
import algorithm
import tables
import sequtils

# From Tutorial II
template withFile(f: expr, filename: string, mode: TFileMode, body: stmt): stmt {.immediate.} =
  block:
    let fn = filename
    var f: TFile
    if open(f, fn, mode):
      try:
        body
      finally:
        close(f)
    else:
      quit("cannot open: " & fn)

proc sorted(s : string) : string =
  var seq = toSeq(s.items)
  seq.sort(system.cmp[char])
  var newStr = newString(s.len)
  for i,ch in seq.pairs: newStr[i] = ch
  newStr

var dictionary = initTable[string,seq[string]]()

withFile(words, "/usr/share/dict/words", fmRead):
  while not words.EndOfFile:
    let line = words.readLine
    if line.len >= 2 and line.len < 19 and line.allCharsInSet({'a'..'z'}):
      let sline = sorted(line)
      if dictionary.hasKey(sline):
        dictionary.mget(sline).add(line)
      else:
        dictionary[sline] = @[line]

var keys = toSeq(dictionary.keys)
keys.sort(system.cmp[string])

with_file(dict, "anadict.txt", fmWrite):
  for key in keys:
    dict.writeln(key, " ", dictionary[key].join(" "))
