use std::*;
use std::io::*;
use std::time::*;
use rand::prelude::*;
use crossterm::*;

fn clear_console(){
    print!("{esc}c", esc = 27 as char);
    print!("\x1B[2J\x1B[1;1H");
    print!("╔══════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗\n\
            ║                                                                                                                  ║\n\
            ║                                                                                                                  ║\n\
            ║                                                                                                                  ║\n\
            ║                                                                                                                  ║\n\
            ║                                                                                                                  ║\n\
            ╠──────────────────────────────────────────────────────────────────────────────────────────────────────────────────╣\n\
            ║                                                                                                                  ║\n\
            ╚══════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝");
    /*print!("╔══════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗\n\
            │                                                                                                                  │\n\
            │                                                                                                                  │\n\
            │                                                                                                                  │\n\
            │                                                                                                                  │\n\
            │                                                                                                                  │\n\
            ├──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤\n\
            │                                                                                                                  │\n\
            ╚══════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝");*/
    /*print!("┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐\n\
            │                                                                                                                  │\n\
            │                                                                                                                  │\n\
            │                                                                                                                  │\n\
            │                                                                                                                  │\n\
            │                                                                                                                  │\n\
            ├──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤\n\
            │                                                                                                                  │\n\
            └──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘");*/
    let _ = write!(stdout(), "{}", Goto(1, 2));
}

fn main() {
    //don't add the 'fn()' type to this. It will break.
    let print_help = ||
    {
        let _ = write!(stdout(), "'exit': exits the program.{0}'roll': rolls a d6 dice and outputs the result.{1}'help': gives a list of all commands.", Goto(1, 3), Goto(1, 4));
    };

    let time_now: SystemTime = SystemTime::now();

    let mut rng: ThreadRng = thread_rng();
    let rng_values: Vec<i32> = (1..=6).collect();

    clear_console();
    print_help();
    loop{
        let _ = rng.reseed();
        let mut input:String = String::new();

        let _ = write!(stdout(), "{0}Command: ", Goto(1, 7));
        let _ = stdout().flush();

        let _ = io::stdin().read_line(&mut input);

        match input.to_lowercase().trim(){
            "exit" => {
                clear_console();
                let _ = write!(stdout(), "{0}exiting...", Goto(1, 3));
                break;
            }
            "roll" => {
                clear_console();
                let dice_roll_value: i32 = rng_values[rng.gen_range(0..6)];
                let _ = write!(stdout(), "{1}You rolled: {0}", dice_roll_value, Goto(1, 3));
            }
            "help" => {
                clear_console();
                print_help();
            }
            _ => {
                clear_console();
                let _ = write!(stdout(), "{0}Invalid Command! Type 'help' for list of all commands.", Goto(1, 3));
            }
        }
        let _ = stdout().flush();
    }
    clear_console();
    let _ = write!(stdout(), "{1}{0} seconds passed", time_now.elapsed().unwrap().as_secs_f64(), Goto(1, 3));
    let _ = write!(stdout(), "{0}Press enter to close console...", Goto(1, 7));
    let _ = stdout().flush();
    let _ = stdin().read(&mut [0u8]).unwrap();
}
