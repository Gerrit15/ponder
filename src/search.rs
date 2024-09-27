use crate::*;

pub struct Search {
    //
}

impl Search {
    pub fn new() -> Search {
        Search {  }
    }
}

impl Page for Search {
    fn draw_page(&mut self, frame: &mut Frame, layout: Rect) {
        frame.render_widget(Paragraph::new("AAAAA"), layout);
    }
    fn key(&mut self, key: KeyCode) {
        //
    }
}
