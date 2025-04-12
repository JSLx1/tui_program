// libraries
use std::{
        io::stdout, 
        thread::sleep, 
        time::{Duration, SystemTime, UNIX_EPOCH}
    };

use crossterm::{
    cursor::{Hide, MoveToNextLine}, 
    event::{self, poll, Event, KeyCode}, 
    style::{Print, ResetColor, SetForegroundColor}, 
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}, 
    ExecutableCommand
};

use tui_program::Widget;
use tui_program::TimePrinter;


// ========== raw terminal ==========
fn in_raw_mode() -> std::io::Result<()> {
    
    // enable raw mode
    enable_raw_mode().unwrap();
    
    // set stdout
    let mut binding = stdout();
    let mut s_out = binding
        .execute(SetForegroundColor(crossterm::style::Color::Rgb{r: 225, g: 104, b: 5})).unwrap()
        .execute(Hide).unwrap();

    // set clock
    let mut current_time = Duration::new(0, 0);
    let clock = TimePrinter::new(2, 2, 6, 3);
    
    s_out.execute(Clear(ClearType::All)).unwrap();

    // repeat until ESC input detected
    loop {
        // show time
        current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
                +Duration::from_secs(3600*9);       // KST: UTC+09:00
        clock.show(&mut s_out, &current_time);

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

    // clean console
    clock.hide(&mut s_out);

    s_out
        .execute(ResetColor).unwrap()
        .execute(MoveToNextLine(1)).unwrap()
        .execute(Print("close")).unwrap();

    disable_raw_mode().unwrap();
    

    Ok(())
}

// main
fn main() -> std::io::Result<()> {
    in_raw_mode().unwrap();
    Ok(())
}