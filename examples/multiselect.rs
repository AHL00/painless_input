use painless_input::multiselect_input;

fn main() {
    let options = vec![
        "Option 1",
        "Option 2",
        "Option 3",
        "Option 4",
        "Option 5",
        "Option 6",
        "Option 7",
        "Option 8",
        "Option 9",
        "Option 10",
    ];

    let selected = multiselect_input("Select an option: ", "Done", &options);
    println!();

    println!("You selected: {:?}", selected);
}