A brainfuck virtual machine written in rust.

Example usage:

fn main() {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let instruction = "";
    brain_fuck(program,instruction);
}

prints "Hello World!"
