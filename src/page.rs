use crate::*;

pub trait Page {
    fn draw_page(&mut self, frame: &mut Frame, out_layout: Rect);
    fn key(&mut self, key: KeyCode);
}

