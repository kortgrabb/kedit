use std::io::stdout;
use termion::{event::Key, raw::IntoRawMode};
use crate::terminal::Terminal;

pub struct Editor { 
    shall_quit: bool,
    terminal: Terminal
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                rip(&error);
            }

            if self.shall_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                rip(&error);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            shall_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.shall_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            println!("~\r");
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);

        if self.shall_quit {
            println!("Hasta la vista, baby!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }

        Terminal::cursor_show();
        let _ = Terminal::flush();
        Ok(())
    }
}

fn rip(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}