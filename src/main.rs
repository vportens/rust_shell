use std::{io::{self, Read, Write}, process::exit};
mod shell;
mod commands;

fn char_array_to_str<'a>(chars: &[char; 1024], buffer: &'a mut [u8; 1024]) -> &'a str {
    let mut index = 0;
    for &c in chars.iter() {
        if c == '\0' || index >= buffer.len() {
            break;
        }
        let bytes_written = c.encode_utf8(&mut buffer[index..]).len();
        index += bytes_written;
    }
    unsafe { std::str::from_utf8_unchecked(&buffer[..index]) }
}


fn test_shell(str: &str) {
    println!("hello from here : {}", str);

}

 
fn main() {
    println!("Minishell!");




    let mut input: [char; 1024] = ['\0'; 1024]; // Tableau de 1024 caractères initialisés à null 
    let mut utf8_buffer = [0u8; 1024];


    
    let mut test_string : String = String::new();

    let mut test_str = char_array_to_str(&input, &mut utf8_buffer);




    loop {
        print!("Minishell >");
        io::stdout().flush().unwrap();

        input = ['\0'; 1024];
        test_string.clear();
        
        // all this part will be transform went i receive enzo message, for the moment i simulate the char[] sending by readline then convert it to char[]
        io::stdin().read_line(& mut test_string).expect("Problem bro ?");

        if test_string == "exit\n" {
            println!("good bye");
            break;
        }
        for (i, c) in test_string.chars().enumerate(){
            if i > 1024 {
                break;
            }
            else {
            input[i] = c; 
            }
        }

        // convert char[] to str
        test_str = char_array_to_str(&input, &mut utf8_buffer);
        shell::processor::process_command(test_str);

    }

        
}
