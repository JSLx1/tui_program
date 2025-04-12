use std::{io::Stdout, time::Duration};
use crossterm::{
    cursor::MoveTo,
    ExecutableCommand,
    style::Print,
};

pub trait Widget {
    type PrintingData;
    fn get_size(&self) -> [u16; 2];
    fn show(&self, stdout: &mut Stdout, data: &Self::PrintingData);
    fn unshow(&self, stdout: &mut Stdout);
    fn move_and_unshow(&mut self, stdout: &mut Stdout, p_col: u16, p_row: u16);
}


// ========== SevenDigit ==========
struct SevenDigit {
    p_col: u16, p_row: u16,
    line_length: u16,
}

impl SevenDigit {
    fn new(p_col: u16, p_row: u16, line_length: u16) -> SevenDigit {
        SevenDigit {p_col, p_row, line_length}
    }
}

impl Widget for SevenDigit {
    type PrintingData = u64;

    fn get_size(&self) -> [u16; 2] {
        // return [col, row]
        [self.line_length+2, self.line_length*2+1]
    }

    fn unshow(&self, stdout: &mut Stdout) {
        let seg_p_col: [u16;8] = [0, 1, 0, 1,                self.line_length+1, 0,                  1,                  self.line_length+1];
        let seg_p_row: [u16;8] = [0, 0, 1, self.line_length, 1,                  self.line_length+1, self.line_length*2, self.line_length+1];
        let seg_is_col: u8 = 0b01010010;

        for i in 1..8 {
            // check this segment's direction and print
            if seg_is_col & (1<<(7-i)) > 0 {
                for j in 0..self.line_length {
                    stdout
                        .execute(MoveTo(self.p_col+seg_p_col[i]+j, self.p_row+seg_p_row[i])).unwrap()
                        .execute(Print(' ')).unwrap();
                }
            }
            else {
                for j in 0..self.line_length {
                    stdout
                        .execute(MoveTo(self.p_col+seg_p_col[i], self.p_row+seg_p_row[i]+j)).unwrap()
                        .execute(Print(' ')).unwrap();
                }
            }
        }
    }

    fn show(&self, stdout: &mut Stdout, data: &Self::PrintingData) {
        /*
         1       _
        234     |_|
        567     |_|
        */
        
        // index: [ignore, 1, 2, 3, ..., 7], 0b01234567
        let seg_p_col: [u16;8] = [0, 1, 0, 1,                self.line_length+1, 0,                  1,                  self.line_length+1];
        let seg_p_row: [u16;8] = [0, 0, 1, self.line_length, 1,                  self.line_length+1, self.line_length*2, self.line_length+1];
        let seg_is_col: u8 = 0b01010010;
        let on_off: u8 = match data {
            0 => 0b01101111,
            1 => 0b00001001,
            2 => 0b01011110,
            3 => 0b01011011,
            4 => 0b00111001,
            5 => 0b01110011,
            6 => 0b01110111,
            7 => 0b01001001,
            8 => 0b01111111,
            9 => 0b01111011,
            _ => 0b00000000
        };
        
        for i in 1..8 {
            // decide what should be printed on this segment
            let to_print: char = 
            if on_off & (1<<(7-i)) == 0 {' '} 
            else {
                if seg_is_col & (1<<(7-i)) > 0      {'_'}
                else                                {'|'}
            };

            // check this segment's direction and print
            if seg_is_col & (1<<(7-i)) > 0 {
                for j in 0..self.line_length {
                    stdout
                        .execute(MoveTo(self.p_col+seg_p_col[i]+j, self.p_row+seg_p_row[i])).unwrap()
                        .execute(Print(to_print)).unwrap();
                }
            }
            else {
                for j in 0..self.line_length {
                    stdout
                        .execute(MoveTo(self.p_col+seg_p_col[i], self.p_row+seg_p_row[i]+j)).unwrap()
                        .execute(Print(to_print)).unwrap();
                }
            }
        }
    }

    fn move_and_unshow(&mut self, stdout: &mut Stdout, p_col: u16, p_row: u16) {
        self.unshow(stdout);
        
        self.p_col = p_col;
        self.p_row = p_row;
    }
}


// ========== Dot ==========
struct Dot {
    p_col: u16, p_row: u16
}

impl Dot {
    fn new(p_col: u16, p_row: u16) -> Dot {
        Dot { p_col, p_row }
    }
}

impl Widget for Dot {
    type PrintingData = char;

    fn get_size(&self) -> [u16; 2] {
        [1, 1]
    }

    fn unshow(&self, stdout: &mut Stdout) {
        stdout
            .execute(MoveTo(self.p_col, self.p_row)).unwrap()
            .execute(Print(' ')).unwrap();
    }

    fn show(&self, stdout: &mut Stdout, _data: &Self::PrintingData) {
        stdout
            .execute(MoveTo(self.p_col, self.p_row)).unwrap()
            .execute(Print('.')).unwrap();
    }

    fn move_and_unshow(&mut self, stdout: &mut Stdout, p_col: u16, p_row: u16) {
        self.unshow(stdout);

        self.p_col = p_col;
        self.p_row = p_row;
    }
}


// ========== TimePrinter ==========
pub struct TimePrinter {
    p_col: u16, p_row: u16,
    line_length: u16, space: u16,

    digits: Vec<SevenDigit>,
    dots: Vec<Dot>,
}

impl TimePrinter {
    pub fn new(p_col: u16, p_row: u16, line_length: u16, space: u16) -> TimePrinter {
        let seg_col_size = line_length+2;
        let seg_row_size = line_length*2+1;

        TimePrinter { p_col, p_row, line_length, space, 
            digits: vec![
                // hour
                SevenDigit::new(p_col, p_row, line_length),
                SevenDigit::new(p_col+seg_col_size, p_row, line_length),

                // minute
                SevenDigit::new(p_col+seg_col_size*2+space, p_row, line_length),
                SevenDigit::new(p_col+seg_col_size*3+space, p_row, line_length),

                // second
                SevenDigit::new(p_col+seg_col_size*4+space*2, p_row, line_length),
                SevenDigit::new(p_col+seg_col_size*5+space*2, p_row, line_length),
            ],

            dots: vec![
                // first colon
                Dot::new(p_col+seg_col_size*2+space/2, p_row+(seg_row_size/4)),
                Dot::new(p_col+seg_col_size*2+space/2, p_row+(seg_row_size*3/4)),
                
                // second colon
                Dot::new(p_col+seg_col_size*4+space*3/2, p_row+(seg_row_size/4)),
                Dot::new(p_col+seg_col_size*4+space*3/2, p_row+(seg_row_size*3/4)),
            ]
        }
    }
}

impl Widget for TimePrinter {
    type PrintingData = Duration;

    fn get_size(&self) -> [u16; 2] {
        let seg_size: [u16; 2] = self.digits[0].get_size();

        [seg_size[0]*6+self.space*2, seg_size[1]]
    }


    fn unshow(&self, stdout: &mut Stdout) {
        // unshow dots
        for u in 0..4usize {
            self.dots[u].unshow(stdout);
        }

        // unshow digits
        for u in 0..6usize {
            self.digits[u].unshow(stdout);
        }
    }

    fn show(&self, stdout: &mut Stdout, data: &Self::PrintingData) {
        // current_time: UNIX_EPOCH ~ now

        let t: u64 = data.as_secs()%(3600*24);
        let t_digit: [u64; 6] = [t/36000, (t/3600)%10, (t%3600)/600, ((t%3600)/60)%10, (t%60)/10, (t%60)%10];

        // show dots
        for u in 0..4usize {
            self.dots[u].show(stdout, &'.');
        }

        // show digits
        for u in 0..6usize {
            self.digits[u].show(stdout, &t_digit[u]);
        }
    }

    fn move_and_unshow(&mut self, stdout: &mut Stdout, p_col: u16, p_row: u16) {
        self.unshow(stdout);
        
        self.p_col = p_col;
        self.p_row = p_row;
    }
}