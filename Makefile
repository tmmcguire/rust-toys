ALT    = alternatives
INPUT  = asdwtribnowplfglewhqagnbe
CC     = gcc -O3

LIBS   = bisect.rs combinations.rs mmap.rs djbhash.rs

RUST_P = anagrams-hashmap-wide anagrams-hashmap \
	 anagrams-vectors-tasks anagrams-vectors-wide \
         anagrams-vectors anagrams-hashmap-mmap \
	 anagrams-djbhashmap anagrams-djbhash-tasks \
         mk_anadict mk_anadict_traits \
         complex hashing-performance

PROGS  = $(RUST_P) \
         $(ALT)/anagrams-hash $(ALT)/anagrams-vectors \
         $(ALT)/mk_anadict $(ALT)/nimrod_anagrams \

PYTHON = $(ALT)/mk_anadict.py $(ALT)/presser_one.py $(ALT)/presser_two.py $(ALT)/presser_three.py

all : libs $(PROGS)

libs : $(LIBS)
	rustc -L. -O bisect.rs
	rustc -L. -O combinations.rs
	rustc -L. -O mmap.rs
	rustc -L. -O djbhash.rs
	touch libs

$(RUST_P) : libs

results : libs $(PROGS) $(PYTHON)
	echo > results
	for j in $(PROGS); do \
	  echo $$j; \
	  echo +$$j >> results; \
          for k in 1 2 3; do \
	    time ./$$j $(INPUT) >> results 2>&1; \
          done; \
	done
	for j in $(PYTHON); do \
	  echo $$j; \
	  echo +$$j >> results; \
          for k in 1 2 3; do \
	    time python ./$$j $(INPUT) >> results 2>&1; \
          done; \
	done

elapsed-times : results
	sed -n -e '/^+/p' -e '/elapsed/s/.* \([^ ]*\)elapsed.*/\1/p' <results > elapsed-times
	rm results

clean :
	rm -f $(PROGS) results lib*

% : %.rs
	rustc -L . -O $<

% : %.nim
	nimrod compile -d:release $<
