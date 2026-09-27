# Contexts

Modified in this fork: shortened the descriptions and corrected API wording.

A context parameter, usually named `cx`, gives a function access to application
state and services. The available operations depend on the context type.

## `App`

`App` owns entity data and global application state. Use it to read or update
the data held by an `Entity<T>`.

## `Context<T>`

`Context<T>` updates a particular entity, notifies its observers, and emits
its events. It dereferences to `App`, so it also provides application services.

## `AsyncApp` and `AsyncWindowContext`

Use `to_async` to create a context that can be held across `await` points.
Entity and window access can fail if their targets have been dropped.

## `TestAppContext`

This context provides test helpers, including simulated input. Accessing a
missing entity or window panics so the test reports a failure.

## `Window`

`Window` provides window state, layout, and drawing operations. It has a root
entity that implements `Render`. Use `WindowHandle::update` to access a window
from its handle.

## `Entity<T>`

`Entity<T>` is a handle to state owned by the app. Read or update it through a
context. An entity that implements `Render` is a view; calling `cx.notify()`
notifies observers of a change.
