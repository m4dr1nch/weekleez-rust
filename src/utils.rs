use std::io;
use std::io::{Stdin, BufRead};

pub trait DiscardUntil {
    fn discard_until_newline(&mut self) -> Result<(), io::Error>;
}

impl DiscardUntil for Stdin {
    fn discard_until_newline(&mut self) -> Result<(), io::Error> {
        let mut buffered = self.lock();
        let amount = {
            let data = buffered.fill_buf()?;
            data.len()
        };
        buffered.consume(amount);
        Ok(())
    }
}

pub fn banner() {
    println!("  _      __        __    __"); 
    println!(" | | /| / /__ ___ / /__ / /__ ___ ___");
    println!(" | |/ |/ / -_) -_)  '_// / -_) -_)_ /");
    println!(" |__/|__/\\__/\\__/_/\\_\\/_/\\__/\\__//__/");
}

pub fn options() {
    println!("");
    println!("Options:");
    println!("[1] List tasks");
    println!("[2] Create task");
    println!("[3] Remove task");
    println!("[4] Mark task as completed");
    println!("[5] Mark task as not completed");
    println!("[q] Exit & save");
    println!("");
}
