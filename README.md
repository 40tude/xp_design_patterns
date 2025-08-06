```
cargo add tokio --features full
cargo run --example 04_state_machine
```
## 01..03
https://medium.com/@bugsybits/i-used-10-classic-design-patterns-in-rust-only-3-made-sense-77fb1e72cf10


## 04..
https://medium.com/@theopinionatedev/the-3-patterns-behind-every-scalable-rust-system-ive-built-06377a2fdad5


Most production systems end up with a hybrid of all 3 patterns:
            +---------------------+
            |   Command Receiver  |
            +----------+----------+
                       |
          [ Command Bus (Dispatcher) ]
                       |
      +----------------+---------------+
      |                |               |
+-----------+   +--------------+   +--------------+
| Validation|   | State Machine|   | Persistence  |
+-----------+   +--------------+   +--------------+
                       |
                  [ Channel Bus ]
                       |
               +-----------------+
               | Worker Executors|
               +-----------------+