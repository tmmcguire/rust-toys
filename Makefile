ALT    = alternatives
INPUT  = asdwtribnowplfglewhqagnbe
CC     = gcc -O3

PROGS  = anagrams-hashmap-wide anagrams-hashmap anagrams-vectors-tasks anagrams-vectors-wide \
         anagrams-vectors mk_anadict mk_anadict_traits $(ALT)/anagrams-hash $(ALT)/anagrams-vectors

PYTHON = $(ALT)/mk_anadict.py $(ALT)/presser_one.py $(ALT)/presser_two.py $(ALT)/presser_three.py

all : $(PROGS)

results : $(PROGS) $(PYTHON)
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
	rm -f $(PROGS) results

% : %.rs
	rustc -L . -O $<
