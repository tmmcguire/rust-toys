#[ link(name = "combinations", vers="1.0") ];
#[ crate_type = "lib" ];

extern mod std;

use io::*;

pure fn each_combination<T>(values : &v/[T], by : uint, fun : &fn(combo : &[&v/T]) -> bool) {
    let length = values.len();
    if by == 0 || by > length { return; }
    let mut indices     = vec::from_fn(by, |i| i);
    let mut combination = vec::from_fn(by, |i| &values[i]);
    loop {
        if !fun(combination) { return; }
        // Increment the indices
        let mut i = by - 1;
        indices[i] += 1;
        while indices[i] == 1 + length - (by - i) && i != 0 {
            i -= 1;
            indices[i] += 1;
        }
        if indices[0] > length - by { break; }
        // Fix up the indices and the combination from i to r-1
        combination[i] = &values[indices[i]];
        for uint::range(i + 1, by) |i| {
            indices[i] = indices[i-1] + 1;
            combination[i] = &values[indices[i]];
        }
    }
}

#[cfg(test)]
mod tests {

    fn dup<T:Copy>(values : &[&T]) -> ~[T] {
        vec::from_fn(values.len(), |i| *values[i])
    }

    #[test]
    fn test_zero() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,0) |p| { v.push(dup(p)); }
        if v != ~[] {
            fail;
        }
    }

    #[test]
    fn test_one() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,1) |p| {
            v.push(dup(p));
        }
        if v != ~[~[1],~[2],~[3],~[4]] {
            fail;
        }
    }

    #[test]
    fn test_two() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,2) |p| {
            v.push(dup(p));
        }
        if v != ~[~[1,2],~[1,3],~[1,4],~[2,3],~[2,4],~[3,4]] {
            fail;
        }
    }

    #[test]
    fn test_three() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,3) |p| {
            v.push(dup(p));
        }
        if v != ~[~[1,2,3],~[1,2,4],~[1,3,4],~[2,3,4]] {
            fail;
        }
    }

    #[test]
    fn test_four() {
        let values = [1,2,3,4];
        let mut v : ~[~[int]] = ~[];
        for each_combination(values,4) |p| {
            v.push(dup(p));
        }
        if v != ~[~[1,2,3,4]] {
            fail;
        }
    }

}