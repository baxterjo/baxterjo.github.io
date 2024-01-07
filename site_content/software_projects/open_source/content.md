This is a bucket for small open source contributions.

#### [Rust Git Version Submodule Support](https://github.com/fusion-engineering/rust-git-version/pull/23)

I use the `git_version` crate in many of my Rust projects. One day I realized I would like the same functionality for my submodules. I created the prototype in a build script and then started in on work for my first `proc_macro` in Rust. Writing code that writes code is hard, but I powered through it and the `git_submodule_versions!()` macro was born!
