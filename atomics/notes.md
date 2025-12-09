## atomics

### atomicsusize vs usize
- the only real diff is of the instructions getting generated when we access the value
- for normal usize we will need exclusive reference but we can operate on atomicUsize with shared reference
-
