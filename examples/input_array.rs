use simple_input::{input_array, input_array_with_validation};

fn main() {
    let arr: Vec<u8> = input_array("Enter 3 numbers: ");
    println!();
    println!("{:?}", arr);

    let validated_arr: Vec<u8> = input_array_with_validation(
        "Enter 3 numbers: ",
        Box::new(|input| {
            if input.len() != 3 {
                Err("Please enter 3 numbers".to_string())
            } else {
                Ok(())
            }
        }),
    );
    println!();
    println!("{:?}", validated_arr);
}
