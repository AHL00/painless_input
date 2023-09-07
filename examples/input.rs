use painless_input::{input, input_with_validation};

fn main() {
    let num: u8 = input("Enter a number: ");
    println!();
    println!("{}", num);

    let validated_num: u8 = input_with_validation(
        "Enter a number: ",
        Box::new(|input| {
            if *input > 10 {
                Err("Please enter a number less than 10".to_string())
            } else {
                Ok(())
            }
        }),
    );
    println!();
    println!("{}", validated_num);
}
