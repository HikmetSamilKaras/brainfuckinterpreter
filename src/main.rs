use std::collections::HashMap;
fn brain_fuck(program: &str,input: &str) {
    const TAPE_SIZE: usize = 30_000;

    let mut arr: [u8; TAPE_SIZE]  = [0; TAPE_SIZE];

    let mut data_ptr = 0;
    let mut instruction_ptr = 0;
    
    let program = program.as_bytes();

    let mut jump = HashMap::new();

    let mut stack = Vec::new();
    
    for (i,c) in program.iter().enumerate() {
        let d = *c as char;
        if d == '[' {
            stack.push(i);
        }
        else if d == ']' {
            let t = stack.pop().unwrap();
            jump.insert(t,i);
            jump.insert(i,t);
        }
    }
    
    let mut input = input.as_bytes().iter();
    
    while instruction_ptr < program.len() {
        match program[instruction_ptr] as char {
            '+' => arr[data_ptr] = arr[data_ptr].wrapping_add(1),
            '-' => arr[data_ptr] = arr[data_ptr].wrapping_sub(1),
            '>' => data_ptr = (data_ptr + 1) % TAPE_SIZE,
            '<' => data_ptr = if data_ptr == 0 { TAPE_SIZE - 1 } else { data_ptr - 1 },
            '[' => if arr[data_ptr] == 0 {
                instruction_ptr = jump[&instruction_ptr]
            },
            ']' => if arr[data_ptr] != 0 {
                instruction_ptr = jump[&instruction_ptr]
            }
            '.' => print!("{}", arr[data_ptr] as char),
            ',' => if let Some(&byte) = input.next() {
                arr[data_ptr] = byte;
            }
            _ => ()
        }
        instruction_ptr += 1;
    }
}

fn main() {
    let program = "";
    let instruction = "";
    brain_fuck(program,instruction);
}
