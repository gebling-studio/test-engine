# refs

Smart pointers `Own` and `Weak` from the [refs](https://github.com/VladasZ/refs) crate.
The memory model of the whole engine.

## Why

UI is a graph of objects that point at each other: views hold subviews, buttons hold callbacks,
callbacks point back at views. The Rust borrow checker cannot express this graph without heavy
workarounds (`Rc<RefCell<>>`, lifetimes everywhere). This engine uses its own pointers instead, so UI
code looks like Swift or TypeScript:

```rust
self.button.on_tap(move || self.do_thing());
```

No `clone()`, no `borrow_mut()`, no lifetimes.

## How it works

- `Own<T>` — the single owner of an object. When `Own` drops, the object is freed.
- `Weak<T>` — a copyable pointer to an object owned by some `Own`. Does not affect lifetime.

Every `Own` registers its address in a global map together with a unique stamp (atomic counter).
A `Weak` remembers the address and the stamp. On every deref it checks: address is still in the map
and the stamp matches. If not — panic with the type name, instead of use-after-free.

The stamp protects from address reuse: when the allocator gives the same address to a new object,
the old `Weak` still reports dead, because the stamp differs.

## In views

The `#[view]` macro rewrites `#[init]` fields to `Weak<Field>`. Real ownership lives in
`ViewBase.subviews: Vec<Own<dyn View>>` — the view tree owns children, like in Swift.
Methods take `self: Weak<Self>`, so closures capture `self` by copy.

## Rules

- Objects live and die on the main thread. Dropping `Own` on another thread panics.
- Mutation is main thread only (checked at runtime with the default `checks` feature).
- Reads from background threads are allowed. Reading heap fields (`String`, `Vec`) while the main
  thread rewrites them is a known accepted risk — rare in practice. When in doubt use `from_main`.
- Two `Weak` copies can give two `&mut` to the same object. Not tracked. Accepted by design —
  do not hold a `&mut` while triggering events that may touch the same view.
