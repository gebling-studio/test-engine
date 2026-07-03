# Main thread and dispatch

From the [hreads](https://github.com/VladasZ/hreads) crate. Same model as UIKit: one main thread
owns all UI state, background threads send work to it.

## Main thread

The engine calls `set_current_thread_as_main()` first thing at startup, on every platform.
After that `is_main_thread()` answers in two memory reads (thread-local id + atomic load).

This is strict: if nobody set the main thread, any check panics with
"Main thread is not set". There is no guessing.

All `Own`/`Weak` runtime checks and `MainLock` globals are built on top of this.

## Sending work to main

- `on_main(action)` — queue a closure. On the main thread it runs immediately, from any other
  thread it runs on the next frame. The engine drains the queue once per frame in
  `AppRunner::update()` via `invoke_dispatched()`.
- `from_main(action)` — same, but blocks the calling thread and returns the result.
  On a multithread tokio worker it uses `block_in_place`, so a blocked worker hands its queued
  tasks to other workers and does not starve the runtime.
- `after(delay, action)` — run a closure on main after a delay.
- `wait_async(future)` — run a future on tokio and block until it finishes.
  Panics when called on the main thread: the future may need `from_main`, which needs the frame
  loop, which the blocked main thread cannot run. That is a guaranteed deadlock.

## Rules

- Never block the main thread waiting for background work.
- `from_main` from a hot background loop is fine occasionally, not thousands of times per frame —
  every call waits up to one frame.
- Queued callbacks run at one defined point of the frame, never in the middle of layout or draw.
