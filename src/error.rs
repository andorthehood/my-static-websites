use std::fmt;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    // Add other error variants here as needed
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::Io(ref err) => write!(f, "IO error: {}", err),
            // Add other error messages here
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::Io(err)
    }
}

// Implement std::error::Error for MyError if needed, though it's often not necessary for application-level errors.
// impl std::error::Error for MyError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match *self {
//             MyError::Io(ref err) => Some(err),
//             // Add other error sources here
//         }
//     }
// }
