# Contributing

This is a small personal project, kept public because it is more useful that
way. Issues and pull requests are welcome; slow replies are likely.

## Before opening a pull request

```sh
cd gpui && cargo build --release
cargo test --release
```

## What gets merged easily

- A bug with a reproduction, and the smallest change that fixes it.
- A fix to something the docs get wrong. The docs are meant to be accurate
  about what is measured and what is merely believed, so a correction there is
  as valuable as a code change.

## What to raise first

Anything that changes the shape of the public API, adds a dependency, or
enlarges the scope. Open an issue before writing the code — it is no fun to
write a patch that gets turned down on direction.

## Style

Match the code around your change: same naming, same comment density. Comments
here explain why a thing is the way it is, not what the line does.
