# Contributing

Bug reports, documentation corrections, and pull requests are welcome. For a bug,
include steps to reproduce it, the expected result, and your operating system
and renderer versions. Include a screenshot for a rendering problem.

Discuss new dependencies, API changes, and major features in an issue before
starting work.

## Checks

For code changes, run from the repository root:

```sh
cd gpui
cargo build --release
cargo test --release
```

See [validation](validation/README.md) for dependencies and capture commands.
Check rendering changes over each background and material, including Identity.
For documentation changes, check links and compare instructions with the source.

Keep patches focused and follow the surrounding code style. Describe the change
and the checks you ran in the pull request.
