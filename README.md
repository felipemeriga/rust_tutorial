# Rust Basics

Rust is a multi-paradigm, general-purpose programming language. Rust emphasizes performance, type safety, and concurrency. Rust enforces memory safety, which means, that all references point to a valid memory address, without requiring the use of a garbage collector or reference counting present in another languages. All the memory gets assigned during compile time, that is why this language is really impressive.

- `cargo` - the package manager of the language, like npm
- `toml` - a file extension, that is used to define your project, like a `go.mod` or `package.json`
- `macros` - are functions and procedures that you can use without importing any package, they are usually called with the `!` sign.
- `immutable/mutable` - by default in Rust, all the variables are immutable, if you want them mutable, you need to specify it with `mut` operator.
- You can define variables with the same name, as long they have different types
- If you start the name of a variable with underscore, Rust compiler will not throw errors for that

Rust is a memory safety, and focused on concurrency. Many programming languages have mechanisms like reference counting, and garbage collection. The presence of these tools, have a trade-off, in some languages like Javascript, when the garbage collector is freeing up memory, all the application is forced to stop.

On Rust, all the memory gets assigned during compile time, but you may ask that are some dynamic variables, that may need their memory assigned on runtime, like we have with Golang. But, different from a lot of languages, Rust allows only a single variable name/alias, to own a memory location at any point in time. This is called ownership, from one variable to another, and when you assign one variable to another, you are basically moving the ownership, and Rust will automatically free the memory from the last owner.

You can’t have two variable names pointing to the same memory address (Except for shared Ownership. Rust provides the reference-counted pointer types Rc and Arc. Because sometimes it’s difficult to find every value a single owner that has the lifetime you need).

That is why he is able to determine all the memory allocation needed on compile time, because with the ownership concept, you can track all the way down, how the data could be moved from one variable to another, allocation only the necessary, knowing exactly when the ownership will be transfer, so it can free memory.

## Pointers in Rust

Pointers are variables that stores memory addresses, and are present in a lot of different languages, but on Rust, it’s a little bit different. This happens, because in Rust, we have the ownership mechanism, that comes with the memory safety mechanism of the language.

In Rust we have two types of pointers, they are:

- References
- Smart pointers
- Raw pointers

## Reference

Reference(`&`), which is a pointer that borrows the value they point to. So different from another languages, when you use reference operator in Rust, you are basically borrowing the value which the reference points to, and as Rust works with ownership concept, you can’t change the value of value you own.

For example:

```bash
let first_test: u32 = 10;
let second_test: &u32 = &first_test;
```

Here we can see `second_test` is receiving the reference o `first_test`. In Golang for example, as second_test has the reference, it could change the content which the reference points to. But, in Rust we can’t do it, because `second_test` is borrowing the value, so we can only use that value, but  never change it, if you try to change you would get an error.

This is a memory safe mechanism from ownership, because the compiler knows that the ownership of the value belongs to `first_test`, and `second_test`, is only borrowing it, so the amount of memory to allocate at compile time is very predictable.

## Raw pointers

It’s pointers like we know on another programming languages. Much of Rust’s safety comes from compile-time checks, but raw pointers don’t have such guarantees, and are [unsafe](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/unsafe.html) to use.

## Smart Pointers

Rust has a variety of smart pointers defined in the standard that provide functionality beyond that provided by references, including a *reference counting* smart pointer type. This pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.

[https://doc.rust-lang.org/book/ch15-00-smart-pointers.html#:~:text=A pointer is a general,the value they point to](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html#:~:text=A%20pointer%20is%20a%20general,the%20value%20they%20point%20to).

## Reference Cycles

Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a *memory leak*). Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust. We can see that Rust allows memory leaks by using `Rc<T>` and `RefCell<T>`: it’s possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.