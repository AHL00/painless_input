use std::io::Write;

//TODO: Fix moving when cursor is at the end of the line and the move is more than the length of the line
//TODO: Arrow key movement

/// Input a string from the user, parse it to the specified type, and validate it using a closure.
/// The closure should return a result which is a () if the input is valid or a string error message to be shown if the input is invalid.
/// ## Example
/// ```
/// use simple_input::input_with_validation;
/// 
/// let input: i32 = input_with_validation("Enter a number: ", Box::new(|x: &i32| {
///    if *x > 10 {
///       Ok(())
///   } else {
///      Err(String::from("Number should be greater than 10"))
///  }
/// }));
/// println!();
/// ```
pub fn input_with_validation<T>(
    input_str: &str,
    validation: Box<dyn Fn(&T) -> Result<(), String>>,
) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input_internal(input_str, Some(validation))
}

/// Input a string from the user and parse it to the specified type.
/// ## Example
/// ```
/// use simple_input::input;
/// 
/// let input: i32 = input("Enter a number: ");
/// println!();
/// ```
pub fn input<T>(input_str: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input_internal(input_str, None)
}

/// Input an array from the user, parse it to the specified type, and validate it using a closure.
/// The array is inputted like this; first prints [ and then ask for input. On enter, if the input is empty, it will stop. Otherwise, it will parse and ask for another input.
/// ## Example
/// ```
/// use simple_input::input_array_with_validation;
/// 
/// let input: Vec<i32> = input_array("Enter numbers: ");
/// println!();
/// ```
pub fn input_array<T>(input_str: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input_array_internal(input_str, None)
}

/// Input an array from the user, parse it to the specified type, and validate it using a closure. The closure should return a result which is () if the input is valid or a string error message to be shown if the input is invalid.
/// The array is inputted like this; first prints [ and then ask for input. On enter, if the input is empty, it will stop. Otherwise, it will parse and ask for another input.
/// ## Example
/// ```
/// use simple_input::input_array_with_validation;
/// 
/// let input: Vec<i32> = input_array_with_validation("Enter numbers: ", Box::new(|x: &Vec<i32>| {
///   if x.len() > 5 {
///      Ok(())
/// } else {
///    Err(String::from("Number of elements should be greater than 5"))
/// }
/// }));
/// println!();
/// ```
pub fn input_array_with_validation<T>(
    input_str: &str,
    validation: Box<dyn Fn(&Vec<T>) -> Result<(), String>>,
) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input_array_internal(input_str, Some(validation))
}

fn input_internal<T>(
    input_str: &str,
    validation: Option<Box<dyn Fn(&T) -> Result<(), String>>>,
) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    crossterm::execute!(std::io::stdout(), crossterm::style::Print(input_str)).unwrap();
    std::io::stdout().flush().unwrap();

    // This is used to show error message and delete it correctly when user enters something
    let mut current_err_msg_len = 0;

    let mut input = String::new();
    let mut res: T;

    let validation_closure = if let Some(value) = validation {
        value
    } else {
        Box::new(|_: &_| Ok(()))
    };

    loop {
        let key_event = crossterm::event::read().unwrap();

        match key_event {
            crossterm::event::Event::Key(key) => {
                if key.kind != crossterm::event::KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    crossterm::event::KeyCode::Enter => {
                        let parsed_input = input.parse::<T>();

                        if parsed_input.is_ok() {
                            res = parsed_input.unwrap();

                            let validation_res = validation_closure(&res);
                            if validation_res.is_ok() {
                                break;
                            } else {
                                // If input is not valid, show a red bg white text error message after clearing the length of the current_input
                                clear_left(input.len() as u16);

                                let error_msg = format!("{}", validation_res.unwrap_err());

                                error_display(error_msg.as_str(), &mut current_err_msg_len);

                                continue;
                            }
                        } else {
                            // If input is not valid, show a red bg white text error message after clearing the length of the current_input
                            clear_left(input.len() as u16);

                            let error_msg = format!("Invalid input: '{}'; try again", input);

                            error_display(error_msg.as_str(), &mut current_err_msg_len);

                            continue;
                        }
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        if current_err_msg_len > 0 {
                            clear_right(current_err_msg_len as u16);
                            current_err_msg_len = 0;
                        }

                        input.push(c);
                        crossterm::execute!(std::io::stdout(), crossterm::style::Print(c)).unwrap();
                        std::io::stdout().flush().unwrap();
                    }
                    crossterm::event::KeyCode::Backspace => {
                        if input.is_empty() {
                            continue;
                        }

                        input.pop();
                        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveLeft(1))
                            .unwrap();
                        crossterm::execute!(std::io::stdout(), crossterm::style::Print(" "))
                            .unwrap();
                        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveLeft(1))
                            .unwrap();
                        std::io::stdout().flush().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    res
}

fn input_array_internal<T>(
    input_str: &str,
    validation: Option<Box<dyn Fn(&Vec<T>) -> Result<(), String>>>,
) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    crossterm::execute!(
        std::io::stdout(),
        crossterm::style::Print(input_str),
        crossterm::style::Print("[")
    )
    .unwrap();
    std::io::stdout().flush().unwrap();

    // Input data like this
    // First print [ and then ask for input
    // Then print , and ask for input
    // If enter is pressed without any input, it will stop
    // After that print ]
    // Example:
    // [1, 2, 3, 4, 5]

    let mut current_input = String::new();
    let mut result = Vec::new();
    let mut input_str_vec: Vec<String> = Vec::new();

    // This is used to show error message and delete it correctly when user enters something
    let mut current_err_msg_len = 0;

    let validation_closure = if let Some(value) = validation {
        value
    } else {
        Box::new(|_: &_| Ok(()))
    };

    loop {
        let key_event = crossterm::event::read().unwrap();

        match key_event {
            crossterm::event::Event::Key(key) => {
                if key.kind != crossterm::event::KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    crossterm::event::KeyCode::Enter => {
                        // If final element with no input
                        if current_input.is_empty() {
                            // If error message is shown, clear it
                            if current_err_msg_len > 0 {
                                clear_right(current_err_msg_len as u16);
                                current_err_msg_len = 0;
                            }

                            if input_str_vec.len() > 0 {
                                // Clear the last ", " from terminal
                                clear_left(2);
                            }

                            // This is the end so print ]
                            crossterm::execute!(std::io::stdout(), crossterm::style::Print("]"))
                                .unwrap();

                            std::io::stdout().flush().unwrap();

                            // Validation
                            let validation_res = validation_closure(&result);
                            if validation_res.is_ok() {
                                break;
                            } else {
                                // If input is not valid, show a red bg white text error message after clearing the length of the current_input

                                // Start with 1 for "]"
                                let mut clear_amount = 1;

                                for (i, input_str) in input_str_vec.iter().enumerate() {
                                    clear_amount += input_str.len();

                                    // if not the last element, add 2 for ", "
                                    if i != input_str_vec.len() - 1 {
                                        clear_amount += 2;
                                    }
                                }

                                clear_left(clear_amount as u16);

                                // crossterm::execute!(std::io::stdout(), crossterm::style::Print("["))
                                //     .unwrap();

                                let error_msg = format!("{}", validation_res.unwrap_err());

                                error_display(error_msg.as_str(), &mut current_err_msg_len);

                                // Start the input again by resetting everything
                                result.clear();
                                input_str_vec.clear();
                                current_input.clear();

                                continue;
                            }
                        }
                        // If there is input
                        else {
                            // Add parsed input to result
                            let parse_res = current_input.parse::<T>();

                            if parse_res.is_ok() {
                                result.push(parse_res.unwrap());
                            } else {
                                // If input is not valid, show a red bg white text error message after clearing the length of the current_input
                                clear_left(current_input.len() as u16);

                                let error_msg =
                                    format!("Invalid input: '{}'; try again", current_input);

                                error_display(error_msg.as_str(), &mut current_err_msg_len);

                                current_input.clear();

                                continue;
                            }

                            // Add the current input to input_str_vec
                            input_str_vec.push(current_input.clone());

                            // Clear current_input
                            current_input.clear();

                            // Print ", "
                            crossterm::execute!(std::io::stdout(), crossterm::style::Print(", "))
                                .unwrap();
                        }
                    }
                    crossterm::event::KeyCode::Backspace => {
                        if current_input.is_empty() {
                            // This means the user wants to delete the last element
                            // So we pop the last element from the result
                            if !result.is_empty() {
                                // If error message is shown, clear it
                                if current_err_msg_len > 0 {
                                    clear_right(current_err_msg_len as u16);
                                    current_err_msg_len = 0;
                                }

                                result.pop();

                                // clear the ", " from terminal
                                clear_left(2);

                                // delete the last input_str_vec and clear it from terminal
                                let chars_to_clear = input_str_vec.pop().unwrap().len();

                                clear_left(chars_to_clear as u16);

                                std::io::stdout().flush().unwrap();
                            }
                        } else {
                            // This means just delete the last character from current_input
                            current_input.pop();
                            // Then delete from terminal
                            clear_left(1);
                        }
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        // If error message is shown, clear it
                        if current_err_msg_len > 0 {
                            clear_right(current_err_msg_len as u16);
                            current_err_msg_len = 0;
                        }

                        current_input.push(c);
                        crossterm::execute!(std::io::stdout(), crossterm::style::Print(c)).unwrap();
                        std::io::stdout().flush().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    result
}


fn error_display(error_msg: &str, error_len_var: &mut usize) {
    // Make it red text and red underline
    crossterm::execute!(
        std::io::stdout(),
        crossterm::style::Print("\x1b[41;31;4m"),
        crossterm::style::Print(&error_msg),
        crossterm::style::Print("\x1b[0m")
    )
    .unwrap();

    // move cursor left
    crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::MoveLeft(error_msg.len() as u16)
    )
    .unwrap();
    // flush stdout
    std::io::stdout().flush().unwrap();

    *error_len_var = error_msg.len();
}

fn clear_left(chars: u16) {
    for _ in 0..chars {
        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
        crossterm::execute!(std::io::stdout(), crossterm::style::Print(" ")).unwrap();
        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveLeft(1)).unwrap();
    }

    std::io::stdout().flush().unwrap();
}

fn clear_right(chars: u16) {
    for _ in 0..chars {
        crossterm::execute!(std::io::stdout(), crossterm::style::Print(" ")).unwrap();
    }

    crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveLeft(chars)).unwrap();

    std::io::stdout().flush().unwrap();
}
