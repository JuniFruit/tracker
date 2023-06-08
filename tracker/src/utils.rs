use std::io;
pub fn read_line(output: &mut String) {
    io::stdin().read_line(output).expect("Something went wrong");
}

