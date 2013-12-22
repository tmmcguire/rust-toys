Rust-toys
=========

Fun and exciting explorations of [Rust](http://www.rust-lang.org) and some alternatives.

* **mk_anadict.rs** Part of a translation of [Creating and Optimizing a Letterpress Cheating Program in Python](http://www.jeffknupp.com/blog/2013/01/04/creating-and-optimizing-a-letterpress-cheating-program-in-python/).

* **mk_anadict_traits.rs** Version of mk_anadict.rs making use of Rust traits.

* **anagrams-hashmap.rs** A hashmap-based version of the anagram finder.

* **anagrams-vectors.rs** Old-school, binary search version of the anagram finder.

* **anagrams-djbhashmap.rs** A version of the anagram finder using a simpler hashmap implementation.

* **anagrams-hashmap-mmap.rs** Using a custom C inteface to `mmap` to read files.

* **anagrams-hashmap-wide.rs** Using multiple threads, each having part of the dictionary.

* **anagrams-vectors-wide.rs** Using multiple threads, each having part of the dictionary, using binary search.

* **anagrams-vectors-tasks.rs** Multiple tasks with all of the dictionaries, each getting some of the keys to search.

Supporting files
----------------

* **combinations.rs** A replacement for Python's `combinations` function from `itertools`, in a general combinations and permutations module.

* **permtest.rs** Fidding with `each_permutation`.

* **bisect.rs** A replacement for Python's `bisect` module: binary search algorithms.

* **djbhash.rs** Hashmap based on the DJB hash function and Python's dictionary implementation.

* **mmap.rs** Thin shim to call mmap for reading files.


Python alternatives
-------------------

Taken directly from Jeff Knupp.

* **alternatives/mk_anadict.py**
* **alternatives/presser_one.py**
* **alternatives/presser_three.py**
* **alternatives/presser_two.py**

C alternatives
--------------

Quick and dirty implementations in C.

* **alternatives/anagrams-hash.c**
* **alternatives/anagrams-vectors.c**

[Nimrod](http://nimrod-lang.org/) alternatives
----------------------------------------------

* **alternatives/mk_anadict.nim** A first Nimrod program!
* **alternatives/nimrod_anagrams.nim** A second Nimrod program!


Other programs
--------------

* **complex.rs** Operator overloading example.
* **hashing-performance.rs** Quick and dirty hash function benchmark.
