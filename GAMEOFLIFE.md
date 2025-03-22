# Game of life implementation and notes

## Cell definition
See [Derive reference](https://doc.rust-lang.org/rust-by-example/trait/derive.html) for `#[derive(...)]`.
It automaticaly create the traits listed.
`PartialEq` is for `a == b`, `Eq` is for `a == a`.

See [Representations reference](https://doc.rust-lang.org/stable/reference/type-layout.html#representations) for `#[repr(u8)]`. 
The Cell is stored as a byte.

Why the `#[wasm_bindgen]` macro in all struts?

## Rules for next cycle
I like that you can have multiple elements in match and the granularity to check the enum of `Cell` as well as the number.
Elegant.

## Formatting Universe as text
`fmt::Display` works the same as a __repr__ in python or a print definiton in R.

```rust
impl fmt::Display for Universe {}
```

is interesting. 
Does it have to implement all traits?
It seems so `Trait` is an interface.

Why doese it require a mutable `fmt::Formatter`? 
Of course is required by the `fmt::Display` Trait [here on reference](std::fmt::Display), but why?

`slice().chunks()` is used to divide the slice in chunks and return an iterator.
It's another way to divide the 1D array into the rows that forms the grid. 

Note: for enum, reference value as `Name::atribute`
