#include <alloca.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/types.h>

#define min(i,j) ((i) < (j) ? i : j)

const int bpi           = sizeof(int) * 8;
const int ht_loadfactor = 5;    /* load factor = .2 */

struct v_line {
  char        *line;            /* beginning of the line */
  int          len;             /* length of first word on line */
};

struct dictionary {
  struct v_line *lines;         /* lines in the dictionary file */
  int            count;         /* number of lines */
  int           *hashtbl;       /* hash table */
  int            htsize;        /* hash table size */
};

unsigned int
hash(char *s, unsigned int l) {
  unsigned int hash = 5381;
  int i;
  for (i = 0; i < l; ++i) {
    hash = hash * 33 ^ s[i];
  }
  return hash;
}

int
eq_dictline(struct v_line *line, char *letters, unsigned int len) {
  return line->len == len && memcmp(line->line, letters, len) == 0;
}

void
combinations(struct dictionary *dict, char *letters, int llen, int *set) {
  int i, r;
  for (r = 2; r <= llen; ++r) {
    int max_indices0 = llen - r;
    int *indices = alloca(sizeof(int) * r);
    for (i = 0; i < r; ++i) { indices[i] = i; }
    char *combination = alloca(sizeof(char) * r);
    memcpy(combination, letters, r);
    while (1) {
      // handle this combination
      unsigned int k = hash(combination, r) % dict->htsize;
      int j = dict->hashtbl[k];
      while (j >= 0 && !eq_dictline(&dict->lines[j], combination, r)) {
        k = (k + 1) % dict->htsize;
        j = dict->hashtbl[k];
      }
      if (j >= 0) { set[j / bpi] |= 1 << (j % bpi); }
      // get the next combination
      i = r - 1;
      indices[i] += 1;
      while (i > 0 && indices[i] > max_indices0 + i) {
        i -= 1;
        indices[i] += 1;
      }
      if (indices[0] > max_indices0) { break; }
      combination[i] = letters[indices[i]];
      for (i = i+1; i < r; ++i) {
        indices[i] = indices[i-1] + 1;
        combination[i] = letters[indices[i]];
      }
    }
  }
}

int
char_cmp(const void* a, const void* b) {
  char *ach = (char*) a;
  char *bch = (char*) b;
  if (*ach < *bch) { return -1; }
  else if (*bch < *ach) { return 1; }
  else { return 0; }
}

int
char_count(char ch, char *s, int l) {
  int i, count = 0;
  for (i = 0; i < l; ++i) {
    if (s[i] == ch) { ++count; }
  }
  return count;
}

int
main(int argc, char *argv[]) {
  long i, j, k;

  // open and map anadict-rust.txt
  int fd = open("anadict-rust.txt", O_RDONLY);
  struct stat stat_buf;
  if (fstat(fd, &stat_buf) != 0) { 
    fprintf(stderr, "Cannot stat file");
    exit(1);
  }
  char *buffer = mmap(0, stat_buf.st_size, PROT_READ, MAP_SHARED, fd, 0);

  // initialize dictionary
  struct dictionary dict;
  dict.count = char_count('\n', buffer, stat_buf.st_size);
  dict.htsize = dict.count * ht_loadfactor;
  dict.hashtbl = alloca(dict.htsize * sizeof(int));
  for (i = 0; i < dict.htsize; ++i) { dict.hashtbl[i] = -1; }
  dict.lines = alloca(dict.count * sizeof(struct v_line));
  i = 0;
  j = 0;
  while (i < stat_buf.st_size) {
    struct v_line *l = &dict.lines[j];
    l->line = &buffer[i];
    l->len = index(&buffer[i], ' ') - &buffer[i];
    k = hash(l->line, l->len) % dict.htsize;
    while (dict.hashtbl[k] >= 0) { k = (k + 1) % dict.htsize; };
    dict.hashtbl[k] = j;
    ++j;
    while (buffer[i] != '\n') { ++i; }
    ++i;
  }

  // create output set
  int *set = alloca( 1 + ((dict.count / bpi) * sizeof(int)) );
  for (i = 0; i < dict.count; i += bpi) { set[i/bpi] = 0; }

  // compute set
  qsort(argv[1], strlen(argv[1]), sizeof(char), char_cmp);
  combinations(&dict, argv[1], strlen(argv[1]), set);

  // print output
  j = 0;
  for (i = 0; i < dict.count; ++i) {
    if (set[i/bpi] & (1 << (i % bpi))) {
      char *ch = index(dict.lines[i].line, ' ') + 1;
      ++j;
      for (; *ch != '\n'; ++ch) {
        if (*ch == ' ') { ++j; }
        /* putchar((*ch != ' ') ? *ch : '\n'); */
      }
      /* putchar('\n'); */
    }
  }
  printf("%ld\n", j);

  munmap(buffer, stat_buf.st_size);
  close(fd);
}
