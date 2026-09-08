# Ownership
Ownership is primarily a discipline of heap management

All heap data must be owned by exactly one variable.
Rust deallocates heap data once its owner goes out of scope.
Ownership can be transferred by moves, which happen on assignments and function calls.
Heap data can only be accessed through its current owner, not a previous owner.

## References
References provide the ability to read and write data without consuming ownership of it. References are created with borrows (& and &mut) and used with dereferences (*), often implicitly.

However, references can be easily misused. Rust’s borrow checker enforces a system of permissions that ensures references are used safely:

All variables can read, own, and (optionally) write their data.
Creating a reference will transfer permissions from the borrowed place to the reference.
Permissions are returned once the reference’s lifetime has ended.
Data must outlive all references that point to it.
In this section, it probably feels like we’ve described more of what Rust cannot do than what Rust can do. That is intentional! One of Rust’s core features is allowing you to use pointers without garbage collection, while also avoiding undefined behavior. Understanding these safety rules now will help you avoid frustration with the compiler later.

### Fixing ownership errors
When fixing an ownership error, you should ask yourself: is my program actually unsafe? If yes, then you need to understand the root cause of the unsafety. If no, then you need to understand the limitations of the borrow checker to work around them.


## The slice type
Slices are a special kind of reference that refer to sub-ranges of a sequence, like a string or a vector. At runtime, a slice is represented as a “fat pointer” which contains a pointer to the beginning of the range and a length of the range. One advantage of slices over index-based ranges is that the slice cannot be invalidated while it’s being used.
