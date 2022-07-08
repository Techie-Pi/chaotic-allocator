# Chaotic Allocator
Have you ever wondered why people _deallocate_ memory? Aren't they sad about their data just being removed from
existence? **This is the solution to your problem!**

Chaotic Allocator uses [Mimalloc](https://github.com/purpleprotocol/mimalloc_rust) under the hood to implement the 
``alloc``, ``alloc_zeroed`` and ``realloc`` functions (although with some _tuning_).

The ``dealloc`` function is left blank. That way no variable is
ever deallocated! All variables will be _immortal_ (until the memory is full, of course...)

The ``alloc`` function has been tuned. Now each variable has the necessary space to be comfortable. Traditional
allocators pack variables to each other. That makes them overwhelmed! Now, they have more than enough space to be relaxed
and perform better at their job!

## Usage
```rust
use chaoticalloc::ChaoticAlloc;

#[global_allocator]
static GLOBAL: ChaoticAlloc = ChaoticAlloc;
```

## Installation
> **Warning** Before using **_ChaoticAlloc_** keep in mind you'll need _lots_ of memory available
```toml
[dependencies]
chaoticalloc = { git = "https://github.com/Techie-Pi/chaotic-allocator", version = "*" }
```

## Features
- Fast _and ethical_ allocation thanks to [Mimalloc](https://github.com/purpleprotocol/mimalloc_rust)
- The **fastest deallocation** possible. If you never deallocate, you don't have any performance overhead!
