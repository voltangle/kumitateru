use crossterm::style::Stylize;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{io, process};
use crossterm::{terminal, cursor, ExecutableCommand};
use anyhow::Result;

fn construct_arrow_selection(header: &str, items: Vec<&str>, highlighted: i64, selected: bool) -> String {
    let mut result = String::from(header.to_owned() + "\n");
    let mut i = 0;
    let longest_item = "";
    let mut length_of_longest_item: i64 = 0;
    for item in items.clone() {
        if longest_item.len() < item.len() {
            length_of_longest_item = item.len() as i64;
        }
    }
    loop {
        if selected && highlighted == i as i64 {
            result.push_str(&*format!("{}", format!("{}) {}", i + 1, items[i].clone()).bold()));
        } else {
            result.push_str(&*format!("{}) {}", i + 1, items[i].clone()));
        }

        let mut filler = String::new();
        for _ in 0..(length_of_longest_item - items[i].len() as i64) {
            filler.push(' ');
        }
        result.push_str(&*filler);
        if highlighted == i as i64 && !selected { result.push_str("  <"); }

        result.push_str("\n");
        if i >= items.len() - 1 {
            break;
        }
        i += 1;
    }
    result
}

pub fn display_cli_selection(header: &str, items: Vec<&str>) -> Result<i64> {
    let mut highlighted: i64 = 0;
    // This is a thing to fix issues with resizing of the terminal window.
    // When the window resizes, a print of arrow selection is done again,
    // so we need some sort of protection against it. This variable will be
    // false if the window was just resized, because of that continue; statement
    // at that piece of code that handles resizing. If no resizing was done,
    // then it would pass to the end of the loop code and make this variable
    // true again, making selection text to be show again. I hope this clarifies
    // what this variable does :D
    let mut selection_to_show = true;
    let mut exiting_state = false;
    loop {
        if selection_to_show {
            print!("{}", construct_arrow_selection(&*("\n".to_owned() + header), items.clone(), highlighted, if exiting_state { true } else { false }));
            if exiting_state { break }
        }
        selection_to_show = false;

        enable_raw_mode();
        let event = read()?;
        match event {
            Event::Resize(w, h) => {
                disable_raw_mode();
                continue;
            }
            _ => {}
        }

        if event == Event::Key(KeyCode::Up.into()) {
            disable_raw_mode();
            if highlighted == 0 {
                highlighted = 4;
            } else {
                highlighted -= 1;
            }
            reset_selection(items.clone().len() as i64);
        }

        if event == Event::Key(KeyCode::Down.into()) {
            disable_raw_mode();
            if highlighted == 4 {
                highlighted = 0;
            } else {
                highlighted += 1;
            }
            reset_selection(items.clone().len() as i64);
        }

        if event == Event::Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('c') }) {
            disable_raw_mode();
            process::exit(1);
        }

        if event == Event::Key(KeyCode::Enter.into()) {
            disable_raw_mode();
            exiting_state = true;
            reset_selection(items.clone().len() as i64);
        }
        selection_to_show = true;
    }
    Ok(highlighted)
}

fn reset_selection(len: i64) {
    for _ in 0..len + 2 {
        io::stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine));
        io::stdout().execute(cursor::MoveUp(1));
    }
}