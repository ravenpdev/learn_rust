**The Stack and Heap**

All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

When your code calls a functions, the values passed into the function (including, potentially pointer to data on the heap) and the function's local variable get pushed on the stack. When the function is over, those values get popped off the stack.

**Ownership Rules**

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
