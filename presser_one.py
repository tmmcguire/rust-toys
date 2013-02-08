from bisect import bisect_left
from itertools import combinations
from time import time

def loadvars():
    f = open('anadict.txt','r')
    anadict = f.read().split('\n')
    f.close()
    return anadict

def findwords(rack, anadict):
    rack = ''.join(sorted(rack))
    foundwords = []
    for i in xrange(2,len(rack)+1):
        for comb in combinations(rack,i):
            ana = ''.join(comb)
            j = bisect_left(anadict, ana)
            if j == len(anadict):
                continue
            words = anadict[j].split()
            if words[0] == ana:
                foundwords.extend(words[1:])
    return foundwords

if __name__ == "__main__":
    import sys
    rack = sys.argv[1].strip()
    anadict = loadvars()
    foundwords = set(findwords(rack, anadict))
    print(len(foundwords))
