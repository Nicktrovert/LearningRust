use std::io;
use std::io::*;
use std::time::SystemTime;
use rand::prelude::*;

fn clear_console(){
    print!("{esc}c", esc = 27 as char);
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let separator_line: &str = "════════════════════════════════════════════════════════════════════════════";

    //don't add the 'fn()' type to this. It will break.
    let print_help = ||
    {
        println!("{3}\n{0} \n{1} \n{2} \n{3}", "'exit': exits the program.", "'roll': rolls a d6 dice and outputs the result.", "'help': gives a list of all commands.", separator_line);
    };

    let time_now: SystemTime = SystemTime::now();

    let mut rng: ThreadRng = thread_rng();
    let rng_values: Vec<i32> = (1..=6).collect();

    print_help();
    loop{
        let _ = rng.reseed();
        let mut input:String = String::new();

        println!("Command: ");

        let _ = io::stdin().read_line(&mut input);

        match input.to_lowercase().trim(){
            "exit" => {
                clear_console();
                println!("\nexiting...");
                break;
            }
            "roll" => {
                clear_console();
                let dice_roll_value: i32 = rng_values[rng.gen_range(0..6)];
                println!("{1}\n\nYou rolled: {0} \n\n{1}", dice_roll_value, separator_line);
            }
            "help" => {
                clear_console();
                print_help();
            }
            _ => {
                clear_console();
                println!("{0}\n\nInvalid Command! Type 'help' for list of all commands.\n\n{0}", separator_line);
            }
        }
    }
    clear_console();
    println!("\n{0} seconds passed\n{1}", time_now.elapsed().unwrap().as_secs_f64(), separator_line);
    println!("Press enter to close console...");
    let _ = stdin().read(&mut [0u8]).unwrap();
}
