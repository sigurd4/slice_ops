[![Build Status (nightly)](https://github.com/sigurd4/slice_ops/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/slice_ops/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/slice_ops/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/slice_ops/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/slice_ops/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/slice_ops/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/slice_ops/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/slice_ops/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/slice_ops/workflows/Test/badge.svg)](https://github.com/sigurd4/slice_ops/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/slice_ops/workflows/Lint/badge.svg)](https://github.com/sigurd4/slice_ops/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/slice_ops.svg)](https://crates.io/crates/slice_ops)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/slice_ops)](https://docs.rs/slice_ops)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/slice_ops)](https://app.codecov.io/github/sigurd4/slice_ops)

# slice_ops

Provides many useful utility methods for slices.

## integrate / differentiate

- `integrate`
- `differentiate`

```rust
use slice_ops::ops::*;

let mut x = [1, 5, 5, 6, 2, -1, 0, 0, 0];

x.differentiate();

assert_eq!(x, [1, 4, 0, 1, -4, -3, 1, 0, 0]);

x.integrate();

assert_eq!(x, [1, 5, 5, 6, 2, -1, 0, 0, 0]);
```

## find

- `find` / `rfind`
- `find_by` / `rfind_by`
- `find_by_key` / `rfind_by`

```rust
use slice_ops::ops::*;

//                   v
let x = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

let i = x.find(&5).unwrap();

assert_eq!(i, 4);
assert_eq!(x[i], 5);
```

## argmax / argmin

- `argmax` / `argmin`
- `argmax_by` / `argmin_by`
- `argmax_by_key` / `argmin_by_key`

```rust
use slice_ops::ops::*;

//                v
let x = [1, 5, 5, 6, 2, -1, 0, -4, -1, 6];

let i = x.argmax().unwrap();

assert_eq!(i, 3);
```

## visit

- `visit` / `visit_mut`
- `rvisit` / `rvisit_mut`
- `visit_async` / `visit_mut_async`
- `try_visit` / `try_visit_mut`
- `try_rvisit` / `try_rvisit_mut`
- `try_visit_async` / `try_visit_mut_async`

```rust
use slice_ops::ops::*;

let mut x = [0; 8];

let mut i = 0;

x.visit_mut(|e| {
    i += 1;
    *e = i;
});

assert_eq!(x, [1, 2, 3, 4, 5, 6, 7, 8]);
```

## ..._assign_all

- `add_assign_all` / `add_assign_all_async`
- `sub_assign_all` / `sub_assign_all_async`
- `mul_assign_all` / `mul_assign_all_async`
- `div_assign_all` / `div_assign_all_async`
- `rem_assign_all` / `rem_assign_all_async`
- `shl_assign_all` / `shl_assign_all_async`
- `shr_assign_all` / `shr_assign_all_async`
- `bitor_assign_all` / `bitor_assign_all_async`
- `bitand_assign_all` / `bitand_assign_all_async`
- `bitxor_assign_all` / `bitxor_assign_all_async`
- `neg_assign_all` / `neg_assign_all_async`
- `not_assign_all` / `not_assign_all_async`

```rust
use slice_ops::ops::*;

let mut x = [1, 2, 3, 4, 5, 6, 7, 8];

x.mul_assign_all(2);
   
assert_eq!(x, [2, 4, 6, 8, 10, 12, 14, 16]);
```

## shift

- `shift_many_left` / `shift_many_right`
- `shift_left` / `shift_right`

```rust
use slice_ops::ops::*;

let mut register = [4, 5, 6, 7, 8, 9];
let mut io = [1, 2, 3];

register.shift_many_right(&mut io);

assert_eq!(register, [1, 2, 3, 4, 5, 6]);
assert_eq!(io, [7, 8, 9]);
```

## spread

- `spread` / `spread_mut`

```rust
#![feature(generic_const_exprs)]

use slice_ops::ops::*;

let arr = [1, 2, 3];
let slice = arr.as_slice();

let [odd, even] = slice.spread();

assert_eq!(odd, [1, 3]);
assert_eq!(even, [2]);
```

## bit_rev_permutation

- `bit_rev_permutation`
- `digit_rev_permutation`

```rust
use slice_ops::ops::*;

let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];

arr.bit_rev_permutation();

assert_eq!(arr, [0b000, 0b100, 0b010, 0b110, 0b001, 0b101, 0b011, 0b111])
```

## grey_code_permutation

- `grey_code_permutation`

```rust
use slice_ops::ops::*;

let mut arr = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];

arr.as_mut_slice().grey_code_permutation();

assert_eq!(arr, [0b000, 0b001, 0b011, 0b010, 0b110, 0b111, 0b101, 0b100])
```

## trim

- `trim` / `trim_mut`
- `trim_front` / `trim_front_mut`
- `trim_back` / `trim_back_mut`

```rust
use slice_ops::ops::*;

let a = [0, 0, 0, 1, 2, 3, 0, 0, 0];

let at = a.trim(|&e| e == 0);

assert_eq!(at, &[1, 2, 3]);
```