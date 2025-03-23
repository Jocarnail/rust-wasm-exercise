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

## Init and render

`|i|` is a closure in this case, an anonymous function. [Reference](https://doc.rust-lang.org/book/ch13-01-closures.html)

## JS part
`<pre>` is "preformatted", used for monospaced, code and similar. [Ref](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)

`<canvas>` is a drawing board. The 2d context works more or less like the turtle in python.
The grid is drawn as move, line, move, line.
Is saves the movements as paths, then stroke the whole object when `ctx.stroke()` is called.

Problem with `memory`. It appears that is not exported by `wasm_game_of_life_bg.js`.
Importing `wasm_game_of_life_bg.wasm` seems to resolve the error, bur the program behaves wrong.
Console log shows that `memory.buffer` and `cells` are correct. The numbers exists and are updated.
This do not reflect on the canvas though.
I forgot `return` in `getIndex`: after all JS expects it, not like rust or other where it's just the last statement.

Cool, everything works.

## Exercises.
