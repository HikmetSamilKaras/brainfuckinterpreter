A brainfuck virtual machine written in rust.

Example usage:

```rust
fn main() {
    let program = "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.";
    brain_fuck(program);
}
```

prints "Hello World!"
