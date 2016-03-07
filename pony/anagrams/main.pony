use "collections"
use "debug"
use "files"

actor Main
  let empty: Array[String] val = recover Array[String] end
  let set: Set[String] ref = Set[String]

  new create(env: Env) =>
    let letters =
      try
        get_letters( env.args(1) )
      else
        env.out.print("Usage: anagrams letters")
        return
      end
    let dictionary =
      try
        read_dictionary(env, "anadict.txt")
      else
        env.out.print("cannot read anadict.txt")
        return
      end
    env.out.print(dictionary.size().string())
    for i in Range(0, letters.size() + 1) do

      // for combo in Combination[U32](letters, i) do
      //   if dictionary.has_key(combo) then
      //     try
      //       let words = dictionary(combo)
      //       for word in words.values() do
      //         set.set(word)
      //       end
      //     end
      //   end
      // end

      try
        EachCombination[U32](letters, i,
          object is Fn[U32]
            let dictionary': ArrayHashMap val = dictionary
            var set': Set[String] ref = set
            fun ref apply(combo: Array[U32]) =>
              if dictionary'.has_key(combo) then
                try
                  let words = dictionary'(combo)
                  for word in words.values() do
                    set'.set(word)
                  end
                end
              end
          end
        )
      end

    end
    env.out.print(set.size().string())

  fun get_letters(letters: String): Array[U32] val =>
    recover
      let letters' = Array[U32](letters.size())
      for rune in letters.runes() do
        letters'.push(rune)
      end
      Lib.sort[U32](letters')
      letters'
    end

  fun read_dictionary(env: Env, path: String): ArrayHashMap val ? =>
    recover
      let d = ArrayHashMap()
      let caps: FileCaps val = recover val FileCaps.set(FileRead).set(FileStat) end
      var file: (None val | File ref) = None
      try
        file = File.open( FilePath(env.root, path, caps) )
        if (file as File).errno() isnt FileOK then
          error
        end
        for line in (file as File).lines() do
          let line' = line.split()
          let k = Array[U32]( line'(0).size() ).concat( line'(0).runes() )
          let l = (consume line').slice(1)
          d(k) = l
        end
        d
      else
        error
      then
        if file isnt None then (file as File).dispose() end
      end
    end
