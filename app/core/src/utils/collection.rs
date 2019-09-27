pub fn is_out_of_bounds<T>(index: usize, array: &Vec<T>) -> bool {
    let length = array.len();
    index < length
}
