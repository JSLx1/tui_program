// libraries
use std::{
        io::{stdout, Write}, thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}
    };

use crossterm::{
    cursor::{MoveTo, MoveToNextLine}, 
    event::{self, poll, Event, KeyCode}, 
    execute, 
    // style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, 
    terminal::{disable_raw_mode, enable_raw_mode, size, SetSize, Clear, ClearType}, 
    ExecutableCommand
};
use ui::TimePrinter;

mod ui;

fn in_raw() -> std::io::Result<()> {
    
    let (cols, rows) = size()?;
    // Resize terminal and scroll up.
    execute!(
        stdout(),
        SetSize(10, 10),
        // ScrollUp(5),
    )?;

    // Be a good citizen, cleanup
    execute!(stdout(), SetSize(cols, rows))?;


    // enable raw mode
    enable_raw_mode().unwrap();
    // execute!(stdout(), MoveToNextLine(1))?;
    
    let mut stdout = stdout();
    let mut t = Duration::new(0, 0);
    let clock = TimePrinter::new(2, 2, 3, 3);

    loop {
        // show time
        t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
                +Duration::from_secs(3600*9);       // KST: UTC+09:00
        clock.show(&mut stdout, t);

        // listen event
        if poll(Duration::from_millis(100)).unwrap() {
            // execute!(stdout, MoveTo(0, 0))?;         
            
            match event::read().unwrap() {
                Event::Key(ev) => {
                    match ev.code {
                        // ESC key means exit..
                        KeyCode::Esc => {break;}
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // sleep
        sleep(Duration::from_millis(50));
    }

    clock.unshow(&mut stdout);
    execute!(stdout, MoveToNextLine(1))?;

    disable_raw_mode().unwrap();
    println!("out");

    return Ok(());
}

// main
fn main() -> std::io::Result<()> {
    /* 
    // using the macro
    execute!(
        stdout(),
        // SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    println!(" ");

    // or using functions
    stdout()
        .execute(SetBackgroundColor(Color::Blue))?
        // .execute(SetForegroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;

    println!(" ");
    */
    
    in_raw().unwrap();

    Ok(())
}