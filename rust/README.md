# Concurrent Programming

1. Concurrent Programming Model

- OS Thread
- Event-driven
- Coroutines
- Actor Model

2. Async in Rust(generate state machine)

- `Future` is lazy
- `Async` is zero-cost
- Don't provide internal runtime
- single/multi thread
- Rust supports `async` in compiler, but many apps depend on community's crate.
- `std` supports `Future` trait.
- Rust compiler supports `async/await`.
- `futures crate` provides many macro and functions.
- `async` code are supported by `async runtimes`.
- Rust does not allow declare `async` in trait.

3. Intro to `aync` and `swait`

- `async` transforms a chunk of code into a state machine which implemented `Future trait`.
- `futures::executor::block_on` 
`block_on` blocks current thread until `Future`'s runtime completed.
- `.await` would not block current thread, it just asynchronously wait `Future`'s accompliment.