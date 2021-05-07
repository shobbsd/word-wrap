pub fn wrap(input: &str, line_length: usize) -> String {
    let mut str_collection: Vec<char> = input.chars().collect();
    let num_of_new_lines = str_collection.len() / line_length; // because im using usize and not f32 i wont get fractions?

    match num_of_new_lines == 0 {
        // usize I dont need to worry about it being negative
        true => {
            return String::from(input);
        }
        false => {
            for i in 0..num_of_new_lines {
                let insert_index = ((i + 1) * line_length) + i;
                str_collection.insert(insert_index, '\n');
            }
            str_collection.iter().collect::<String>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn give_line_length_greater_than_input_return_input() {
        let actual = wrap("hello world", 100);
        let expected = "hello world";
        assert_eq!(actual, expected);
    }
    #[test]
    fn give_line_length_less_than_input_return_str_with_new_line() {
        let actual = wrap("hello world", 6);
        let expected = "hello \nworld";
        assert_eq!(actual, expected);
    }
    #[test]
    fn can_handle_multiple_new_lines() {
        let actual = wrap("hello world", 3);
        let expected = "hel\nlo \nwor\nld";
        assert_eq!(actual, expected);
    }
    #[test]
    fn it_works() {
        let actual = wrap("fjhdskfbdsjfsl  fdsj", 4);
        let expected = "fjhd\nskfb\ndsjf\nsl  \nfdsj\n";
        assert_eq!(actual, expected);
    }
}
