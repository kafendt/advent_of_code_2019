pub fn handle_addition(input: &mut [u32], instruction_pos: usize) {
    let slice = &input[instruction_pos..instruction_pos + 4];
    input[slice[3] as usize] = input[slice[1] as usize] + input[slice[2] as usize];
}

pub fn handle_multiplication(input: &mut [u32], instruction_pos: usize) {
    let slice = &input[instruction_pos..instruction_pos + 4];
    input[slice[3] as usize] = input[slice[1] as usize] * input[slice[2] as usize];
}

pub fn handle_finish(input: &[u32], instruction_pos: usize) {
}
