#Simple png image stegnography tool

Implements the [PNG standard](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)

- encode `cargo run -- encode .\test.png RuSt "this is a message"`
- decode `cargo run -- decode .\test.png RuSt`
- remove `cargo run -- remove .\test.png RuSt`
- print `cargo run -- print .\test.png`
