# rickroll-rs

**WARNING**: this crate is for the purpose of demonstration about how terrible a rust dependency could do, please DO NOT ACTUALLY USE IT IN PRODUCTION. It's not in the crates.io, but the crates there could do something even worse.

Rickrolling at compile-time.

For any crate that depends on this, the first 5 seconds of *Never gonna give you up* by *Rick Astley* will be played for *N*+1 times when compiling this crate, where `N` is the number of macro `rickroll_rs::rickroll!` used in the crate. The extra 1 time is in the `build.rs`
