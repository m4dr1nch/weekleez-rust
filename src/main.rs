use std::io::{stdout, Write, Read};
use std::collections::BTreeMap;

mod utils;
mod state;
mod task;

use utils::DiscardUntil;
use state::State;

fn main() {
    utils::banner();

    let mut state: State = State {
        tasks: BTreeMap::new(), 
        amount: 0,
        completed: 0,
    };

    loop {
        utils::options();

        print!("Select option: ");
        let _ = stdout().flush();

        let input: Option<char> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as char);

        let _ = std::io::stdin().discard_until_newline();

        let input = match input {
            None => continue,
            Some(res) => res,
        };

        match input {
            '1' => state::print_tasks(&mut state),
            '2' => state::add_task(&mut state),
            '3' => state::remove_task(&mut state),
            '4' => state::mark_completed(&mut state), 
            '5' => state::mark_uncompleted(&mut state),
            'q' => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid option")
        }
    }
}
