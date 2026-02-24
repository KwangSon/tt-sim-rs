# AGENTS.md

## Setup commands
- Run tests: `cargo test`

## Code style
- 

## Reference Repositories

This project models virtual hardware intended to eventually support tt-kmd.
The following repositories should be studied for architectural reference.

### tt-kmd

Repository: https://github.com/tenstorrent/tt-kmd

What it is:
- Linux kernel-mode driver (C)
- Handles PCI probing, BAR mapping, interrupts, memory management
- Exposes device interface to userspace via ioctl and device nodes

What to study:

1. Device initialization flow
   - PCI probe/remove
   - Resource allocation
   - BAR region handling

2. Register interaction model
   - How MMIO registers are accessed
   - How command submission is triggered

3. Command queue management
   - Ring buffer logic
   - Head/tail pointer updates
   - Synchronization strategy

4. Interrupt handling
   - How completion is signaled
   - IRQ enable/disable logic

This repository defines the hardware expectations.
Our virtual device must model the structures that tt-kmd assumes exist.


### luwen

Repository: https://github.com/tenstorrent/luwen

What it is:
- Rust-based low-level system interface library
- Provides structured hardware interaction patterns
- Demonstrates Rust design for device-level control

What to study:

1. Rust hardware abstraction patterns
   - Struct-based device modeling
   - Safe wrapper patterns around low-level operations

2. Memory access organization
   - Clear separation between device state and API surface

3. Error handling style
   - Result-based propagation
   - Explicit state transitions

luwen is not a kernel driver.
It serves as a Rust reference for structuring hardware-facing code cleanly.

### tt-metal

Repository: https://github.com/tenstorrent/tt-metal

What it is:
- Low-level programming model and runtime for Tenstorrent hardware
- Defines how workloads are dispatched to Tensix cores
- Manages command submission, memory layout, and execution orchestration

What to study:

1. Execution model
   - How kernels are launched
   - How command buffers are structured
   - How synchronization is handled

2. Memory model
   - Device memory regions
   - L1/L2/DRAM abstractions
   - Buffer allocation and lifetime rules

3. Runtime ↔ driver expectations
   - What the runtime assumes about the hardware interface
   - How commands ultimately flow toward the kernel driver

tt-metal represents the software layer that originally interacted with the simulator.
Studying it helps ensure that our virtual hardware model reflects realistic execution and memory behavior expectations.

## How This Project Uses Them

- tt-kmd defines what the hardware must look like.
- luwen demonstrates how to model hardware cleanly in Rust.
- This repository implements a simplified virtual hardware model that satisfies tt-kmd expectations.
