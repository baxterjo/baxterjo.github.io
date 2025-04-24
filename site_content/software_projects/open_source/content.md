This is a bucket for small open source contributions.

#### [Etherparse Traffic Class Support](https://github.com/JulianSchmid/etherparse/pull/118)

My team was using [`etherparse`](https://docs.rs/etherparse/latest/etherparse/) as a packet parser for an ip router implementation. We needed to implement traffic classes for IPv6 packets in our router. The `etherparse` crate did not yet have this feature so I implemented it. When contributing to open source projects in a work capacity I always try to make sure that I can copy/paste my solution from the open source PR into our codebase. This means I can get the ticket completed in a timely manner while still making a valuable contribution to our community. Then when the PR is released in the dependency crate, I can go back and migrate to that implementation with (hopefully) minimal changes.

#### [Rust Git Version Submodule Support](https://github.com/fusion-engineering/rust-git-version/pull/23)

I use the `git_version` crate in many of my Rust projects. One day I realized I would like the same functionality for my submodules. I created the prototype in a build script and then started in on work for my first `proc_macro` in Rust. Writing code that writes code is hard, but I powered through it and the `git_submodule_versions!()` macro was born!
