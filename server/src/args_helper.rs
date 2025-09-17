use std::path::Path;

pub fn validate_args(args: &[String]) -> Result<(), String> {
    validate_length(args.len() as i32)?;
    validate_path(&args[1])?;

    Ok(())
}

fn validate_length(vector_length: i32) -> Result<(), String> {
    const MAX_ARGS: i32 = 2;
    if vector_length != MAX_ARGS {
        return Err(format!(
            "Invalid number of args... Expected {}, actual {}",
            MAX_ARGS, vector_length
        ));
    }

    Ok(())
}

fn validate_path(path_str: &str) -> Result<(), String> {
    // let path = Path::new(path_str).parent();
    // if !path.exists() {
    //     return Err(format!("Parent path {} does not exist", path_str));
    // }
    match Path::new(path_str).parent() {
        Some(parent) => {
            if !parent.exists() {
                return Err(format!("Parent path {:?} does not exist", parent));
            }
        }
        None => {
            return Err(format!("Path {} has no parent", path_str));
        }
    }

    Ok(())
}
