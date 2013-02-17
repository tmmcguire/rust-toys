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

const int bpi = sizeof(int) * 8;

struct v_line {
  char *line;                           // beginning of the line
  int   len;                            // length of first word on line
};

struct dictionary {
  struct v_line *lines;                 // lines in the dictionary file
  int            count;                 // number of lines
};

int
bisect_left_lines(struct v_line *lines, char *letters, int llength, int lo, int hi) {
  while (lo < hi) {
    int mid = (lo + hi) / 2;
    int cmp = memcmp(lines[mid].line, letters, min(lines[mid].len, llength));
    if (cmp < 0 || (cmp == 0 && lines[mid].len < llength)) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return lo;
}

void
combinations(struct dictionary *dict, char *letters, int llen, int *set) {
  int i, r;
  for (r = 2; r < llen; ++r) {
    int max_indices0 = llen - r;
    int *indices = alloca(sizeof(int) * r);
    for (i = 0; i < r; ++i) { indices[i] = i; }
    char *combination = alloca(sizeof(char) * r);
    memcpy(combination, letters, r);
    while (1) {
      // handle this combination
      int j = bisect_left_lines(dict->lines, combination, r, 0, dict->count);
      if (j < dict->count
          && dict->lines[j].len == r
          && memcmp(dict->lines[j].line, combination, r) == 0) {
        set[j / bpi] |= 1 << (j % bpi);
      }
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
cmpch(const void* a, const void* b) {
  char *ach = (char*) a;
  char *bch = (char*) b;
  if (*ach < *bch) { return -1; }
  else if (*bch < *ach) { return 1; }
  else { return 0; }
}

int
main(int argc, char *argv[]) {
  long i, j;

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
  dict.count = 0;
  for (i = 0; i < stat_buf.st_size; ++i) {
    if (buffer[i] == '\n') { dict.count++; }
  }
  dict.lines = alloca(dict.count * sizeof(struct v_line));
  i = 0;
  j = 0;
  while (i < stat_buf.st_size) {
    dict.lines[j].line = &buffer[i];
    dict.lines[j].len = index(&buffer[i], ' ') - &buffer[i];
    ++j;
    while (buffer[i] != '\n') { ++i; }
    ++i;
  }

  // create output set
  int *set = alloca( 1 + ((dict.count / bpi) * sizeof(int)) );
  for (i = 0; i < dict.count; i += bpi) { set[i/bpi] = 0; }

  // compute set
  int argv1_len = strlen(argv[1]);
  qsort(argv[1], argv1_len, sizeof(char), cmpch);
  combinations(&dict, argv[1], argv1_len, set);

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
