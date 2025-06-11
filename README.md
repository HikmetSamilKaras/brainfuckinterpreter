A brainfuck virtual machine written in rust.

Example usage:

```rust
fn main() {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let instruction = "";
    brain_fuck(program,instruction);
}
```

prints "Hello World!"
