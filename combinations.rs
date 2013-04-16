#[ link(name = "combinations", vers="1.0") ];
#[ crate_type = "lib" ];

#[warn(deprecated_mode)];
#[warn(deprecated_pattern)];
#[warn(vecs_implicitly_copyable)];
#[deny(non_camel_case_types)];

extern mod std;

/// Iterate over `r`-length subsequences of elements from `values`.
///
/// Combinations are emitted in lexicographic sort order. So, if the
/// input iterable is sorted, the combination tuples will be produced
/// in sorted order.
///
/// Elements are treated as unique based on their position, not on
/// their value. So if the input elements are unique, there will be no
/// repeat values in each combination.
///
/// For a `values` vector of length *n*, the number of items emitted
/// is *n! / r! / (n-r)!* when *0 <= r <= n* or zero when *r > n*.
///
/// # Arguments
///
/// * `values` - A vector of values from which the combinations are
/// chosen
///
/// * `r` - The length of the emitted combinations
///
/// * `fun` - The function to iterate over the combinations
///
/// # See also
///
/// This function gleefully stolen from Python
/// [`itertools.combinations`](http://docs.python.org/2/library/itertools.html#itertools.combinations).
pub fn each_combination<T:Copy>(values : &[T], r : uint, fun : &fn(combo : &[T]) -> bool) {
    let length          = values.len();
    if r == 0 || r > length { return; }
    let max_indices0    = length - r;
    let mut indices     = vec::from_fn(r, |i| i);
    let mut combination = vec::from_fn(r, |i| values[i]);
    loop {
        if !fun(combination) { return; }
        // Increment the indices
        let mut i = r - 1;
        indices[i] += 1;
        while i > 0 && indices[i] > max_indices0 + i {
            // indices[i] now too large; decrement i, increment indices[i]
            // and we'll fix up the following indices later
            i -= 1;
            indices[i] += 1;
        }
        // Can't fix up 'done'
        if indices[0] > max_indices0 { break; }
        // Fix up the indices and the combination from i to r-1
        combination[i] = values[indices[i]];
        for uint::range(i + 1, r) |i| {
            indices[i] = indices[i-1] + 1;
            combination[i] = values[indices[i]];
        }
    }
}

/// Iterate over `r`-length subsequences of elements from `values`.
///
/// This is an alternative to each_combination that uses references to
/// avoid copying the elements of the values vector.
/// 
/// To avoid memory allocations and copying, the iterator will be
/// passed a reference to a vector containing references to the
/// elements in the original `values` vector.
///
/// # Arguments
///
/// * `values` - A vector of values from which the combinations are
/// chosen
///
/// * `r` - The length of the emitted combinations
///
/// * `fun` - The function to iterate over the combinations
pub fn each_combination_ref<'v,T>(values : &'v [T], r : uint, fun : &fn(combo : &[&'v T]) -> bool) {
    each_combination(vec::from_fn(values.len(), |i| &values[i]), r, fun);
}

/// Reverse a slice of a vector in place.
///
/// Reverse the elements in the vector between `start` and `end - 1`.
///
/// # Arguments
///
/// * `v` - The mutable vector to be modified
///
/// * `start` - Index of the first element of the slice
///
/// * `end` - Index one past the final element to be reversed.
///
/// # Example
///
/// Assume a mutable vector `v` contains `[1,2,3,4,5]`. After the call:
///
/// ~~~
///
/// reverse_part(v, 1, 4);
///
/// ~~~
///
/// `v` now contains `[1,4,3,2,5]`.
///
/// # Safety note
///
/// Behavior is undefined if `start` or `end` do not represent valid
/// positions in `v`.
pub fn reverse_part<T>(v : &mut [T], start : uint, end : uint) {
    let mut i = start;
    let mut j = end - 1;
    while i < j {
        v[i] <-> v[j];
        i += 1;
        j -= 1;
    }
}

/// Iterate over all permutations of vector `values`.
///
/// Permutations are produced in lexicographic order with respect to
/// the order of elements in `values` (so if `values` is sorted then
/// the permutations are lexicographically sorted).
///
/// The total number of permutations produced is `len(values)`!. If
/// `values` contains repeated elements, then some permutations are
/// repeated.
///
/// To avoid memory allocations and copying, the iterator will be
/// passed a reference to a vector containing references to the
/// elements in the original `values` vector.
///
/// # Arguments
///
/// * `values` - A vector of values from which the permutations are
/// chosen
///
/// * `fun` - The function to iterate over the combinations
pub fn each_permutation<T : Copy>(values : &[T], fun : &fn(perm : &[T]) -> bool) {
    let length = values.len();
    let mut permutation = vec::from_fn(length, |i| values[i]);
    if length <= 1 {
        fun(permutation);
        return;
    }
    let mut indices = vec::from_fn(length, |i| i);
    loop {
        if !fun(permutation) { return; }
        // find largest k such that indices[k] < indices[k+1]
        // if no such k exists, all permutations have been generated
        let mut k = length - 2;
        while k > 0 && indices[k] >= indices[k+1] {
            k -= 1;
        }
        if k == 0 && indices[0] > indices[1] { return; }
        // find largest l such that indices[k] < indices[l]
        // k+1 is guaranteed to be such
        let mut l = length - 1;
        while indices[k] >= indices[l] {
            l -= 1;
        }
        // swap indices[k] and indices[l]; sort indices[k+1..]
        // (they're just reversed)
        indices[k] <-> indices[l];
        unsafe {
            reverse_part(indices, k+1, length);
        }
        // fixup permutation based on indices
        for uint::range(k, length) |i| {
            permutation[i] = values[indices[i]];
        }
    }
}

/// Iterate over all permutations of vector `values`.
///
/// This is an alternative to each_permutation that uses references to
/// avoid copying the elements of the values vector.
/// 
/// To avoid memory allocations and copying, the iterator will be
/// passed a reference to a vector containing references to the
/// elements in the original `values` vector.
///
/// # Arguments
///
/// * `values` - A vector of values from which the permutations are
/// chosen
///
/// * `fun` - The function to iterate over the permutations
pub fn each_permutation_ref<'v,T>(values : &'v [T], fun : &fn(perm : &[&'v T]) -> bool) {
    each_permutation(vec::from_fn(values.len(), |i| &values[i]), fun);
}

#[cfg(test)]
mod tests {

    fn dup<T:Copy>(values : &[&T]) -> ~[T] {
        vec::from_fn(values.len(), |i| *values[i])
    }

    #[test]
    fn test_reverse_part() {
        let mut values = [1,2,3,4,5];
        reverse_part(values,1,4);
        assert!(values == [1,4,3,2,5]);
    }

    #[test]
    fn test_zero() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,0) |p| { v.push(vec::from_slice(p)); }
        assert!(v == ~[]);
    }

    #[test]
    fn test_zero_ref() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination_ref(values,0) |p| { v.push(dup(p)); }
        assert!(v == ~[]);
    }

    #[test]
    fn test_one() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,1) |p| {
            v.push(vec::from_slice(p));
        }
        assert!(v == ~[~[1],~[2],~[3],~[4]]);
    }

    #[test]
    fn test_one_ref() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination_ref(values,1) |p| {
            v.push(dup(p));
        }
        assert!(v == ~[~[1],~[2],~[3],~[4]]);
    }

    #[test]
    fn test_two() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,2) |p| {
            v.push(vec::from_slice(p));
        }
        assert!(v == ~[~[1,2],~[1,3],~[1,4],~[2,3],~[2,4],~[3,4]]);
    }

    #[test]
    fn test_two_ref() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination_ref(values,2) |p| {
            v.push(dup(p));
        }
        assert!(v == ~[~[1,2],~[1,3],~[1,4],~[2,3],~[2,4],~[3,4]]);
    }

    #[test]
    fn test_three() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,3) |p| {
            v.push(vec::from_slice(p));
        }
        assert!(v == ~[~[1,2,3],~[1,2,4],~[1,3,4],~[2,3,4]]);
    }

    #[test]
    fn test_three_ref() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination_ref(values,3) |p| {
            v.push(dup(p));
        }
        assert!(v == ~[~[1,2,3],~[1,2,4],~[1,3,4],~[2,3,4]]);
    }

    #[test]
    fn test_four() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,4) |p| {
            v.push(vec::from_slice(p));
        }
        assert!(v == ~[~[1,2,3,4]]);
    }

    #[test]
    fn test_four_ref() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination_ref(values,4) |p| {
            v.push(dup(p));
        }
        assert!(v == ~[~[1,2,3,4]]);
    }

    #[test]
    fn test_permutations0() {
        let values = [];
        let mut v : ~[~[int]] = ~[];
        for each_permutation(values) |p| {
            v.push(vec::from_slice(p));
        }
        assert!(v == ~[~[]]);
    }

    #[test]
    fn test_permutations0_ref() {
        let values = [];
        let mut v : ~[~[int]] = ~[];
        for each_permutation_ref(values) |p| {
            v.push(dup(p));
        }
        assert!(v == ~[~[]]);
    }

    #[test]
    fn test_permutations1() {
        let values = [1];
        let mut v : ~[~[int]] = ~[];
        for each_permutation(values) |p| {
            v.push(vec::from_slice(p));
        }
        assert!(v == ~[~[1]]);
    }

    #[test]
    fn test_permutations1_ref() {
        let values = [1];
        let mut v : ~[~[int]] = ~[];
        for each_permutation_ref(values) |p| {
            v.push(dup(p));
        }
        assert!(v == ~[~[1]]);
    }

    #[test]
    fn test_permutations2() {
        let values = [1,2];
        let mut v : ~[~[int]] = ~[];
        for each_permutation(values) |p| {
            v.push(vec::from_slice(p));
        }
        assert!(v == ~[~[1,2],~[2,1]]);
    }

    #[test]
    fn test_permutations2_ref() {
        let values = [1,2];
        let mut v : ~[~[int]] = ~[];
        for each_permutation_ref(values) |p| {
            v.push(dup(p));
        }
        assert!(v == ~[~[1,2],~[2,1]]);
    }

    #[test]
    fn test_permutations3() {
        let values = [1,2,3];
        let mut v : ~[~[int]] = ~[];
        for each_permutation(values) |p| {
            v.push(vec::from_slice(p));
        }
        assert!(v == ~[~[1,2,3],~[1,3,2],~[2,1,3],~[2,3,1],~[3,1,2],~[3,2,1]]);
    }

    #[test]
    fn test_permutations3_ref() {
        let values = [1,2,3];
        let mut v : ~[~[int]] = ~[];
        for each_permutation_ref(values) |p| {
            v.push(dup(p));
        }
        assert!(v == ~[~[1,2,3],~[1,3,2],~[2,1,3],~[2,3,1],~[3,1,2],~[3,2,1]]);
    }

}
