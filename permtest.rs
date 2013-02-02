use io::*;
use vec::*;

pure fn each_permutation<T: Copy>(v: &[T], put: fn(ts: &[T]) -> bool) {
    let ln = len(v);
    if ln <= 1 {
        put(v);
    } else {
        // This does not seem like the most efficient implementation.  You
        // could make far fewer copies if you put your mind to it.
        let mut i = 0u;
        while i < ln {
            let elt = v[i];
            let mut rest = slice(v, 0u, i);
            unsafe {
                rest.push_all(const_view(v, i+1u, ln));
                for each_permutation(rest) |permutation| {
                    if !put(append(~[elt], permutation)) {
                        return;
                    }
                }
            }
            i += 1u;
        }
    }
}

fn reverse_part<T>(vector : &[mut T], x : uint, y : uint) {
    let mut i = x;
    let mut j = y;
    while i < j {
        vec::swap(vector, i, j);
        i += 1;
        j -= 1;
    }
}

pure fn each_permutation2<T>(values : &v/[T], fun : &fn(perm : &[&v/T]) -> bool) {
    let length = values.len();
    let mut permutation = vec::from_fn(length, |i| &values[i]);
    if length <= 1 {
        fun(permutation);
        return;
    }
    let mut indices = vec::from_fn(length, |i| i);
    loop outer: {
        if !fun(permutation) { break; }
        // find largest k such that indices[k] < indices[k+1]
        // if no such k exists, all permutations have been generated
        let mut k = length - 2;
        while indices[k] >= indices[k+1] {
            if k == 0 && indices[0] > indices[1] { break outer; }
            k -= 1;
        }
        // find largest l such that indices[k] < indices[l]
        // k+1 is guaranteed to be such
        let mut l = length - 1;
        while indices[k] >= indices[l] {
            l -= 1;
        }
        // swap indices[k] and indices[l]; sort indices[k+1..]
        unsafe {
            vec::swap(indices, k, l);
            reverse_part(indices, k+1, length-1);
        }
        // fixup permutation based on indices
        for uint::range(k, length) |i| {
            permutation[i] = &values[indices[i]];
        }
    }
}


fn main() {
    for uint::range(0,10) |r| {
        let vector = vec::from_fn(r,|i| i);
        let mut count = 0;
        for each_permutation2(vector) |c| {
//            println(fmt!("%?", c));
            count += 1;
        }
        println(fmt!("%d permutations", count));
        let mut count = 0;
        for each_permutation(vector) |c| {
//            println(fmt!("%?", c));
            count += 1;
        }
        println(fmt!("%d permutations", count));
    }
    for each_permutation(["a","b","c","d"]) |c| {
        println(fmt!("%?", c));
    }
}
