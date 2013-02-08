#[ link(name = "bisect", vers="1.0") ];
#[ crate_type = "lib" ];

#[warn(deprecated_mode)];
#[warn(deprecated_pattern)];
#[warn(vecs_implicitly_copyable)];
#[deny(non_camel_case_types)];

extern mod std;

/// Locate the insertion point for `x` in `a` to maintain sorted order.
///
/// If `x` is already present in `a`, the insertion point will be
/// before (to the left of) any existing entries.
/// 
/// # Arguments
///
/// * `a` - Ordered vector of elements
///
/// * `x` - Element to be found in `a`
///
/// * `lo` - Lowest element in `a` to examine
///
/// * `hi` - One larger than the highest element in `a` to examine
///
/// The parameters `lo` and `hi` may be used to specify a subsequence
/// of the vector which should be considered.
///
/// # Return value
///
/// The returned value `i` partitions the array `a` into two parts so that
/// *(∀j : lo <= j < i : a[j] < x)* and *(∀j : i <= j < hi : x <= a[j])*.
/// In other words, all of the elements of `a` with indices less than `i`
/// are strictly less than `x`, while all af the elements of `a` with indices
/// greater than or equal to `i` are at least `x`.
pub pure fn bisect_left<T : core::cmp::Ord>(a : &[T], x : T, lo : uint, hi : uint) -> uint {
    let mut lo = lo;
    let mut hi = hi;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < x { lo = mid + 1; }
        else          { hi = mid; }
    }
    return lo;
}

/// Locate the insertion point for `x` in `a` to maintain sorted order.
///
/// If `x` is already present in `a`, the insertion point will be
/// after (to the right of) any existing entries.
/// 
/// # Arguments
///
/// * `a` - Ordered vector of elements
///
/// * `x` - Element to be found in `a`
///
/// * `lo` - Lowest element in `a` to examine
///
/// * `hi` - One larger than the highest element in `a` to examine
///
/// The parameters `lo` and `hi` may be used to specify a subsequence
/// of the vector which should be considered.
///
/// # Return value
///
/// The returned value `i` partitions the array `a` into two parts so that
/// *(∀j : lo <= j < i : a[j] <= x)* and *(∀j : i <= j < hi : x < a[j])*.
/// In other words, all of the elements of `a` with indices less than `i`
/// are at most `x`, while all af the elements of `a` with indices
/// greater than or equal to `i` are strictly greater than `x`.
pub pure fn bisect_right<T : core::cmp::Ord>(a : &[T], x : T, lo : uint, hi : uint) -> uint {
    let mut lo = lo;
    let mut hi = hi;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if x < a[mid] { hi = mid; }
        else          { lo = mid + 1; }
    }
    return lo;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_left() {
        let v = [1,2,3,4,5];
        assert bisect_left(v,0,0,v.len()) == 0;
        assert bisect_left(v,1,0,v.len()) == 0;
        assert bisect_left(v,3,0,v.len()) == 2;
        assert bisect_left(v,5,0,v.len()) == 4;
        assert bisect_left(v,6,0,v.len()) == 5;
    }

    #[test]
    fn test_right() {
        let v = [1,2,3,4,5];
        assert bisect_right(v,0,0,v.len()) == 0;
        assert bisect_right(v,1,0,v.len()) == 1;
        assert bisect_right(v,3,0,v.len()) == 3;
        assert bisect_right(v,5,0,v.len()) == 5;
        assert bisect_right(v,6,0,v.len()) == 5;
    }

}
