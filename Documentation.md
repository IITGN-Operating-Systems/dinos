# Documentation

---

# Volatile

---

**How are read-only and write-only accesses enforced? (enforcing)The ReadVolatile and WriteVolatile types make it impossible to write and read, respectively, the underlying pointer. How do they accomplish this?**

The ReadVolatile and WriteVolatile types enforce access restrictions by intentionally omitting specific trait implementations that would allow unintended operations. ReadVolatile provides methods for volatile reads but does not implement any methods or traits that enable writing, ensuring that the underlying value remains read-only. Similarly, WriteVolatile exposes only volatile writes, deliberately excluding any functionality that would allow reading. This design leverages Rust’s type system and ownership rules to enforce strict access control at compile time, preventing accidental misuse—especially in scenarios like memory-mapped I/O, where improper access can lead to undefined behaviour.

**What do the macros do? (macros) What do the readable!, writeable!, and readable_writeable! macros do?**

**readable! Macro**

- Generates code to make a type readable.
- Implements the Readable<T> trait.
- Allows volatile reads from the underlying value.
- Informs the ReadVolatile type that it can read the stored value.
- Returns a pointer to the value.

**writeable! Macro**

- Generates code to make a type writeable.
- Implements the Writeable<T> trait.
- Allows only volatile writes to the underlying value.
- Informs the WriteVolatile type that it can write to the stored value.
- Returns a mutable pointer to the value.

**readable_writeable! Macro**

- Generates code to make a type both readable and writeable.
- Implements the ReadableWriteable<T> trait.
- Combines Readable<T> and Writeable<T> functionalities.
- Allows both volatile reads and writes.
- Informs the Volatile type that it can read and write the stored value.

## StackVec

---

**Why does push return a Result? (push-fails) The push method from Vec in the standard library has no return value, but the push method from our StackVec does: it returns a Result indicating that it can fail. Why can StackVec::push() fail where Vec::push() does not?**

Vec::push() in the standard library does not return a Result because it dynamically allocates memory and grows as needed, whereas StackVec::push() must return a Result since it has a fixed capacity and cannot expand. When StackVec reaches its limit, push() cannot allocate more space, so it fails gracefully by returning an error instead of panicking, allowing the caller to handle the failure explicitly.

**Why is the 'a bound on T required? (lifetime)**

**struct StackVec<'a, T> { buffer: &'a mut [T], len: usize }**

**Rust automatically enforces the bound T: 'a and will complain if type T lives shorter than the lifetime 'a. For instance, if T is &'b str and 'b is strictly shorter than 'a, Rust won’t allow you to create the instance of StackVec<'a, &'b str>. Why is the bound required? What could go wrong if the bound wasn’t enforced by Rust?**

The bound T: 'a ensures that the type T does not outlive the lifetime 'a, preventing potential use-after-free issues. Without this bound, StackVec<'a, T> could store elements that have a shorter lifetime than 'a, leading to dangling references if they are accessed after their original lifetime has ended. For example, if T were &'b str with 'b shorter than 'a, StackVec might hold invalid references after 'b expires. By enforcing T: 'a, Rust guarantees that all elements in the buffer remain valid for the entire lifetime 'a, ensuring memory safety.

**Why does StackVec require T: Clone to pop()? (clone-for-pop) The pop method from Vec<T> in the standard library is implemented for all T, but the pop method from our StackVec is only implemented when T implements the Clone trait. Why might that be? What goes wrong when the bound is removed?**

Vec<T>::pop() simply removes and returns the last element because Vec manages its own heap-allocated storage. However, StackVec operates on a fixed-size mutable slice, which does not allow ownership transfer of individual elements. Since StackVec cannot move elements out of the buffer (as they are borrowed from an external slice), it must return a clone of the element instead. If T: Clone were not required, pop() would not be able to return an owned T without violating Rust’s ownership rules, potentially leading to dangling references or invalid memory access.

**Which tests make use of the Deref implementations? (deref-in-tests) Read through the tests we have provided in src/tests.rs. Which tests would fail to compile if the Deref implementation did not exist? What about the DerefMut implementation? Why?**

Tests that rely on implicit dereferencing, such as indexing (stack_vec[i]), slicing (&stack_vec[..]), or passing StackVec to functions expecting a slice (&[T]), would fail to compile without the Deref implementation. This is because Deref allows StackVec to be treated as a slice (&[T]), enabling these operations. Similarly, tests that modify elements through indexing (stack_vec[i] = value) or mutate slices (stack_vec.push(new_value)) would fail without DerefMut. DerefMut provides mutable access, allowing StackVec to behave like &mut [T], which is required for modifying elements in place.