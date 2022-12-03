# intersection

[crates.io](https://crates.io/crates/intersection)

## Table of contents

* 👋 [Introduction](#introduction)
* 💖 [Contributions & Feedback](#contributions)
* 🧾 [License](#license)

## Introduction

A crate to find the intersection between a collection of sets. Convenient when your
collection of sets consists of more than 2 sets.

There are two options:
* With `HashSet`, use [hash_set::intersection]().
* With `BTreeSet`, use [btree_set::intersection]().

**Why use this over `HashSet::intersection` or `BTreeSet::intersection`?**

The standard library intersection methods require a bit of ceremony when you want to intersect 3
or more sets with each other. These `intersection(other: &Set)` methods produce a lazy iterator,
which you then have to wrap into a `HashSet::from_iter` again. An alternative way is to use
the `BitAnd` implementation of either set. In both cases it requires a bit of boilerplate
which this library does for you 😄. Under the hood, this library uses `BitAnd` to produce an intersected
set of the given input sets.

## Contributions

Contributions, feedback or other correspondence are more than welcome! Feel free to send a message or create an issue 😄.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be licensed as above, without any additional terms or
conditions.
