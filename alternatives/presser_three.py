from itertools import combinations
import collections

def load_anagrams():
    anagrams = collections.defaultdict(list)
    with open('anadict.txt', 'r') as file_handle:
        for line in file_handle:
            words = line.split()
            anagrams[tuple(words[0])] = words[1:]
    return anagrams

def find_words(board, anagrams, max_length=25):
    board = ''.join(sorted(board))
    target_words = []
    for word_length in range(2, len(board) + 1):
        for combination in combinations(board, word_length):
            if combination in anagrams:
                target_words += anagrams[combination]
    return target_words

if __name__ == "__main__":
    import sys
    if len(sys.argv) == 2:
        rack = sys.argv[1].strip()
    else:
        exit()
    anagrams = load_anagrams()
    target_words = set(find_words(rack, anagrams))
    print(len(target_words))
