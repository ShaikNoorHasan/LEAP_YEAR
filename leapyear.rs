
use std::io::{self, Write};
use std::fmt::Display;
use std::process;

fn main() {
       println!("\n\t*** Leap year checking calculator in Rust ***\n");

    let year: i32 = grab_input("Enter a Year to check the given year is leap year or not")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
        .trim()
        .parse()
        .unwrap_or_else(|e| exit_err(&e, 2));

   if year % 100 != 0 && year % 4 == 0  {
    println!("{} is a leap year.",year);
    }
 else if year % 400 ==0{ 
    println!("{} is a leap year.",year);
   }
   else { 
    println!("{} is not a leap year.",year);
   }
}

fn grab_input(msg: &str) -> io::Result<String> {
    let mut buf = String::new();
    print!("{}: ", msg);
    try!(io::stdout().flush());

    try!(io::stdin().read_line(&mut buf));
    Ok(buf)
}

fn exit_err<T: Display>(msg: T, code: i32) -> ! {
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)
}
