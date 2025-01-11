# Unsafe Raw Pointer Modification of a Vector in Rust

This repository demonstrates a common error in Rust involving unsafe raw pointers and vector manipulation.  Incorrect use of raw pointers can lead to memory corruption, data races, and program crashes. The example shows how modifying a vector via a raw pointer after potential reallocation can cause issues.  The solution provides a safe alternative.

## How to reproduce

1. Clone this repository.
2. Navigate to the repository's directory.
3. Run the `bug.rs` file using `rustc bug.rs && ./bug`. This demonstrates the unsafe and incorrect behavior.
4. Run the `bugSolution.rs` file using `rustc bugSolution.rs && ./bugSolution`.  This shows the correct, safe approach.

## Solution

The provided `bugSolution.rs` shows a safer way to modify vector contents.  Instead of using raw pointers, it directly utilizes the vector's methods, avoiding memory safety risks.