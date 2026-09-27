# GPUI 0.2.2

GPUI is a Rust UI framework by Zed Industries. This copy includes the Liquid
Glass changes listed in [MODIFICATIONS.md](MODIFICATIONS.md). GPUI is licensed
under [Apache-2.0](LICENSE-APACHE).

Modified in this fork: setup instructions and API overview below describe this
vendored copy.

## Dependency

Use the local fork. In `gpui/Cargo.toml`, the path is:

```toml
[dependencies]
gpui = { path = "../vendor/gpui" }
```

The `runtime_shaders` feature compiles Metal shaders when the app starts. Without
it, the build compiles the shaders using the Xcode Metal tools.

For macOS builds, install Rust stable, Xcode, and its command-line tools. Open
Xcode once to finish setup. Check the selected toolchain with `xcode-select -p`.
See the [project README](../../README.md) for app build commands.

## API overview

Create an `Application` and call `Application::run`. In its callback, open a
window with `App::open_window` and provide a root view.

- **`Entity<T>`** holds application state. Use its context to update the state
  and notify observers.
- **`Render`** builds the element tree for a view. Elements such as `div` handle
  layout, styling, and event handlers.
- **`Element`** supports custom layout and painting for controls that need them.
- **Actions** connect key bindings to application commands.
- **Contexts** provide access to application state, windows, and services.

## Guides and examples

- [Ownership and data flow](src/_ownership_and_data_flow.rs)
- [Contexts](docs/contexts.md)
- [Keyboard actions](docs/key_dispatch.md)
- [Runnable examples](examples/)
- [Upstream GPUI documentation](https://gpui.rs/)
