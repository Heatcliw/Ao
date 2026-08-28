use std::io::{self, BufRead};

use crate::{Client, Request, check_command, client, publish};

pub struct Data(pub String);

pub fn get_data() -> io::Result<Data> {
    let mut data = String::new();

    io::stdin().read_line(&mut data)?;

    Ok(Data(data.trim().to_string()))
}

pub fn get_multiline_data(request: &mut Request) -> io::Result<()> {
    let stdin = io::stdin();
    let mut data = String::new();

    for line in stdin.lock().lines() {
        let line = line?;

        data.push_str(&line);
        data.push('\n');
    }

    request.push(&data);

    Ok(())
}
