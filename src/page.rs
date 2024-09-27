use crate::*;

pub trait Page {
    fn draw_page(&mut self, frame: &mut Frame);
    fn key(&mut self, key: KeyCode);
}

