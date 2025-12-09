## atomics

### atomicsusize vs usize
- the only real diff is of the instructions getting generated when we access the value
- for normal usize we will need exclusive reference but we can operate on atomicUsize with shared reference
- if one variable is not dependent on other then the CPU is allowed to change the order and CPU does this for specific performance
  gains.
  it could be that the compiler reorders it or the CPU execute it out of order.
  
### Acquire/Release
- it establish a happend before relationship between the thread that previously release the lock and the next thread that takes the lock

### fetch methods
- single/atomic operation it does not depend what the current value is it just fetch
the value and do the operation in an atomic step
- fetch_add/sub is a single atomic operation but the fetch_update is actually a compare_exchange_weak loop implementation 