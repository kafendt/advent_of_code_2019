fn handle_addition(input: &mut [u32], instruction_pos: usize) {
    println!("Found addition instruction at position {}", instruction_pos);
    let slice = &input[instruction_pos..instruction_pos + 4];
    println!("Slice: {:?}", slice);
    input[slice[3] as usize] = input[slice[1] as usize] + input[slice[2] as usize];
}

fn handle_multiplication(input: &mut [u32], instruction_pos: usize) {
    println!(
        "Found multiplication instruction at position {}",
        instruction_pos
    );
    let slice = &input[instruction_pos..instruction_pos + 4];
    println!("Slice: {:?}", slice);
    input[slice[3] as usize] = input[slice[1] as usize] * input[slice[2] as usize];
}

fn handle_finish(input: &[u32], instruction_pos: usize) {
    println!("Found finish instruction at position {}", instruction_pos);
    println!("Final result: {:?}", &input[..]);
}

fn main() {
    let mut input: [u32; 129] = [
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 6, 19, 23, 2, 23, 6, 27,
        1, 5, 27, 31, 1, 10, 31, 35, 2, 6, 35, 39, 1, 39, 13, 43, 1, 43, 9, 47, 2, 47, 10, 51, 1,
        5, 51, 55, 1, 55, 10, 59, 2, 59, 6, 63, 2, 6, 63, 67, 1, 5, 67, 71, 2, 9, 71, 75, 1, 75, 6,
        79, 1, 6, 79, 83, 2, 83, 9, 87, 2, 87, 13, 91, 1, 10, 91, 95, 1, 95, 13, 99, 2, 13, 99,
        103, 1, 103, 10, 107, 2, 107, 10, 111, 1, 111, 9, 115, 1, 115, 2, 119, 1, 9, 119, 0, 99, 2,
        0, 14, 0,
    ];

    for i in (0..input.len()).step_by(4) {
        let value = input[i];
        println!("Input before operation: {:?}", &input[..]);
        match value {
            1 => handle_addition(&mut input, i),
            2 => handle_multiplication(&mut input, i),
            99 => {
                handle_finish(&input, i);
                break;
            }
            _ => println!("Wrong instruction: {}", value),
        };
    }
}