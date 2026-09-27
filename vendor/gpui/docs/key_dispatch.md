# Key Dispatch

Modified in this fork: corrected action handlers and JSON examples.

GPUI is designed for keyboard-first interactivity.

To expose functionality to the mouse, you render a button with a click handler.

To expose functionality to the keyboard, you bind an _action_ in a _key context_.

Actions are similar to framework-level events like `MouseDown`, `KeyDown`, etc, but you can define them yourself:

```rust
mod menu {
    #[derive(Clone, PartialEq, Default, Debug, gpui::Action)]
    #[action(namespace = menu)]
    pub struct MoveUp;

    #[derive(Clone, PartialEq, Default, Debug, gpui::Action)]
    #[action(namespace = menu)]
    pub struct MoveDown;
}
```

Actions are frequently unit structs, for which we have a macro. The above could also be written:

```rust
mod menu {
    gpui::actions!(menu, [MoveUp, MoveDown]);
}
```

Actions can also be more complex types:

```rust
mod menu {
    #[derive(Clone, PartialEq, serde::Deserialize, schemars::JsonSchema)]
    #[serde(rename_all = "lowercase")]
    enum Direction {
        Up,
        Down,
    }

    #[derive(Clone, PartialEq, serde::Deserialize, schemars::JsonSchema, gpui::Action)]
    #[action(namespace = menu)]
    struct Move {
        direction: Direction,
        select: bool,
    }
}
```

To handle actions on a view, wrap its handler in `cx.listener` and pass it to
`on_action`. These fragments assume a `Menu` view and the GPUI prelude imports:

```rust
impl Render for Menu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(cx.listener(|this: &mut Menu, _action: &menu::MoveUp, window, cx| {
                // Update `this`, then call `cx.notify()` to redraw.
            }))
            .on_action(cx.listener(|this: &mut Menu, _action: &menu::MoveDown, window, cx| {
                // Update `this`, then call `cx.notify()` to redraw.
            }))
            .children(unimplemented!())
    }
}
```

In order to bind keys to actions, you need to declare a _key context_ for part of the element tree by calling `key_context`.

```rust
impl Render for Menu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("menu")
            .on_action(cx.listener(|this: &mut Menu, _action: &menu::MoveUp, window, cx| {
                // Update `this`, then call `cx.notify()` to redraw.
            }))
            .on_action(cx.listener(|this: &mut Menu, _action: &menu::MoveDown, window, cx| {
                // Update `this`, then call `cx.notify()` to redraw.
            }))
            .children(unimplemented!())
    }
}
```

Key events are dispatched through the focused element. Track a `FocusHandle`
on the element with `track_focus` and focus it when opening the menu.

An application with a JSON keymap loader can target this context in its keymap. Note how actions are identified in the keymap by their fully-qualified type name.

```json
{
  "context": "menu",
  "bindings": {
    "up": "menu::MoveUp",
    "down": "menu::MoveDown"
  }
}
```

If you had opted for the more complex type definition, you'd provide the serialized representation of the action alongside the name:

```json
{
  "context": "menu",
  "bindings": {
    "up": ["menu::Move", {"direction": "up", "select": false}],
    "down": ["menu::Move", {"direction": "down", "select": false}],
    "shift-up": ["menu::Move", {"direction": "up", "select": true}],
    "shift-down": ["menu::Move", {"direction": "down", "select": true}]
  }
}
```
