pub fn shift(total_letters: i32, min_index: i32, max_index: i32, letter: char, shift: i32) {
    let mut offset: i32 = 0;
    let mut remaining_shift: i32;
    let mut edge_index: i32 = 0;
    let curr_index: i32 = (letter as u8) as i32;

    get_offset(curr_index, min_index, max_index, &mut offset, shift);

    remaining_shift = shift.abs() - offset;
    remaining_shift %= total_letters;

    assign_indexes(
        &mut remaining_shift,
        &mut edge_index,
        shift,
        min_index,
        max_index,
    );

    let final_index = edge_index + remaining_shift;

    println!(
        "total_letters: {}, min_index: {}, max_index: {}, letter: {}, shift: {}",
        total_letters, min_index, max_index, letter, shift
    );
    println!(
        "curr_index: {}, offset: {}, edge_index: {}, remaining_shift: {}, final_index: {}",
        curr_index, offset, edge_index, remaining_shift, final_index
    );
}

fn get_offset(curr_index: i32, min_index: i32, max_index: i32, offset: &mut i32, shift: i32) {
    if shift < 0 {
        *offset = curr_index - min_index;
    } else {
        *offset = max_index - curr_index;
    }
}

fn assign_indexes(
    remaining_shift: &mut i32,
    edge_index: &mut i32,
    shift: i32,
    min_index: i32,
    max_index: i32,
) {
    if shift < 0 {
        *remaining_shift = -(*remaining_shift + 1);
        *edge_index = max_index;
    } else {
        *remaining_shift -= 1;
        *edge_index = min_index;
    }
}
