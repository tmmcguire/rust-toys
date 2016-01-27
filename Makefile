ALT    = alternatives
INPUT  = asdwtribnowplfglewhqagnbe
CC     = gcc -O3

RUST_P = anagrams-hashmap anagrams-vectors anagrams-djbhashmap		\
         mk_anadict mk_anadict_traits complex hashing-performance

# anagrams-hashmap-wide anagrams-vectors-tasks
# anagrams-vectors-wide anagrams-djbhash-tasks 
# anagrams-hashmap-mmap 

PROGS  = $(ALT)/anagrams-hash $(ALT)/anagrams-vectors \
         $(ALT)/mk_anadict $(ALT)/nimrod_anagrams \

PYTHON = $(ALT)/mk_anadict.py $(ALT)/presser_one.py $(ALT)/presser_two.py $(ALT)/presser_three.py

all : rustp $(PROGS)

rustp :
	(cd rust; cargo build --release)	

results : $(PROGS) $(PYTHON)
	echo > results
	for j in $(RUST_P); do \
	  echo $$j; \
	  echo +$$j >> results; \
          for k in 1 2 3; do \
	    time ./rust/target/release/$$j $(INPUT) >> results 2>&1; \
          done; \
	done
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
	(cd rust; cargo clean)
	rm -f $(PROGS) results lib*

% : %.nim
	/home/mcguire/soft/nimrod/nim-0.13.0/bin/nim compile -d:release $<
