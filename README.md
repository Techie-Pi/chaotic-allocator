# Chaotic Allocator
Have you ever wondered why people _deallocate_ memory? Aren't they sad about their data just being removed from
existence? **This is the solution to your problem!**

Chaotic Allocator uses [Mimalloc](https://github.com/purpleprotocol/mimalloc_rust) under the hood to implement the 
``alloc``, ``alloc_zeroed`` and ``realloc`` functions. The ``dealloc`` function is left blank. That way no variable is
ever deallocated! All variables will be _immortal_ (until the memory is full, of course...)

## Usage
```rust
use chaoticalloc::ChaoticAlloc;

#[global_allocator]
static GLOBAL: ChaoticAlloc = ChaoticAlloc;
```

## Installation
<details>
<summary>Before using <b><i>ChaoticAlloc</i></b> keep in mind you'll need <i>lots</i> of memory available</summary>
```toml

[dependencies]
chaoticalloc = { git = "https://github.com/Techie-Pi/chaotic-allocator", version = "*" }
```
</details>

## Features
- Fast allocation thanks to [Mimalloc](https://github.com/purpleprotocol/mimalloc_rust)
- The **fastest deallocation** possible. If you never deallocate, you don't have any performance overhead!