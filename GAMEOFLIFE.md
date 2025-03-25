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

### Initialize the universe with a single spaceship

Easier possible design is the glider: a 3x3 design with a schema.

```
010
001
111
```

How do I stamp it onto the universe?
I need coordinates, with the upper left corner acting as a reference.

The position of the upper corner is the index of getIndex. It is the same for all the positions, actually. 
If I call the rows of the glider `gRow` and the columns of the glider `gCol` the position of each element with respect to the universe is:

```
startingCol = x
startingRow = y
gIdx = getIndex(row: y + gRow, column: x + gCol )
```

I have to know the width and the height of the glider (or any other element.)

I should split the operatioon in half:

- A place operation which is the responsability of the Universe
  - Manage the positional calculations
  - Decide the rules for adding the element into the world (what happend if the cell is Alive?)
- A Glider element, which carries the information on the structure.
  - width, heigh, a vector of elements.
  - fn to get row, col from vector index.
  - Should it inherit from Universe? They are more or less the same thing functionwise. Maybe I will refactor it. 
  - The glider is a more specific version of a generic element/structure.

#### Implementing as inheritance
In this specific case inheritance would be fitting because the `Spaceship` could directly access all methods of `Universe`; namely the display and visualization.
Rust do not have inheritance built in.

I could just add traits, have a field of type `Universe` inside the `Spaceship` struct, bt then I would have to implement for each trait that `Spaceshif.Universe` calls the function of `Universe`.
It feels too convoluted. The children would always need to implement all relevant traits of the father; what would be the point? It feels like to much boilerplate...

Should I use a trait `Conway` as the basic implementation with some defaults and then have it apply to both universe and Spaceship?
On the other hand I could just add a coupple of functions to `Universe` itself: whould that be as clean?
I am trying using traits. I think for a better implementation I should rethink the whole thing to use composition, but for now I will go for the dirty soltion.
Well wasm_bindgen do not support traits, all this is moot...

Let's just implement a coupple of methods on `Universe`.

glider just became a different initialization of Universe. It works pretty well.
Placing works great! Remember that the position starts from 1 and not 0. Remember to put them far enough from each other!

#### TODOs:

- A system that allows to rotate a Universe element: to change the direction of the glider.
- Better way to define the structures. I want to be able to encode the structures in a easier way that as the single cells in code.

### Random initial Universe
Not bad. Not sure why the guide uses a js crate instead of a generic rust random crate.

### Refactoring cells as single bit
For now I will do this in a branch.

git is a great thing.

## Testing game of life
Note why `get_cells` returns a slice? I suppose that `&self.cells` instead of `self.cells` is needed because otherwise the borrow checker would "give away" the cells?
Either that or use clone?

Default test not working. Error.
wasm_bindgen_test_configure in `test/web.rs` is set to test only browser, but there may be a bug.
Issue seems to be with the `--headless` flag: the test run not headless passes. Error appears only when using the headless flag.
[Some github issue](https://github.com/rustwasm/wasm-pack/issues/1355) suggests that it is because of missing drivers.
It is definetly missing drivers for chrome and safari. Firefox works without issues in headless.

