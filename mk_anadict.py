import collections

with open('/usr/share/dict/words') as file_handle:
    words = collections.defaultdict(list)
    letters = set('abcdefghijklmnopqrstuvwxyz\n')
    for word in file_handle:
        if len(set(word) - letters) == 0 and len(word) > 2 and len(word) < 20:
            word = word.strip()
            key = ''.join(sorted(word))
            words[key].append(word)

anagram_dictionary = sorted([' '.join([key] + value) for key, value in words.items()])
with open('anadict.txt', 'w') as file_handle:
    file_handle.write('\n'.join(anagram_dictionary))
