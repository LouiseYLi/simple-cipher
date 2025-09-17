pub fn validate_args(args: &[String]) -> Result<(), String> {
    validate_length(args.len() as i32)?;
    validate_shift_value(args)?;

    Ok(())
}

fn validate_length(vector_length: i32) -> Result<(), String> {
    const MAX_ARGS: i32 = 4;
    if vector_length != MAX_ARGS {
        return Err(format!(
            "Invalid number of args... Expected 4, actual {}",
            vector_length
        ));
    }
    Ok(())
}

fn validate_shift_value(args: &[String]) -> Result<(), String> {
    if args[2].trim().parse::<i32>().is_err() {
        return Err(format!(
            "Invalid shift value... Expected integer, actual {}",
            args[2]
        ));
    }
    Ok(())
}
