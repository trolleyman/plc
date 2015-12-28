
use edit::Editor;

use gtk::{self, Widget, Window, Frame, EventBox, DrawingArea, WindowPosition};
use gtk::signal::Inhibit;
use gtk::traits::*;
use gdk::EventType;
use cairo::{Context, Antialias};
use cairo::enums::{FontSlant, FontWeight};

pub struct Gui {
	#[allow(dead_code)]
	win: &'static mut Window,
	edit: Editor,
}
impl Gui {
	pub fn new(win: &'static mut Window) -> Gui {
		win.set_title("Propositional Logic Calculator");
		win.set_border_width(10);
		win.set_window_position(WindowPosition::Center);
		win.set_double_buffered(true);
		win.set_default_size(600, 500);
		
		win.connect_delete_event(|_, _| {
			gtk::main_quit();
			Inhibit(true)
		});
		
		let da_frame = Frame::new(None).unwrap();
		{
			let eb = EventBox::new().unwrap();
			win.connect_key_press_event(|_, e| {
				if e._type == EventType::KeyPress {
					let ih = ::get_gui().edit.handle_input(e);
					if ih == Inhibit(true) {
						::get_gui().dirty();
					}
					ih
				} else {
					Inhibit(false)
				}
			});
			let da = DrawingArea::new().unwrap();
			da.connect_draw(|w: Widget, c: Context| {
				::get_gui().render(w, c);
				
				Inhibit(false)
			});
			eb.add(&da);
			da_frame.add(&eb);
		}
		win.add(&da_frame);
		
		win.show_all();
		
		Gui {
			win: win,
			edit: Editor::new(),
		}
	}
	
	pub fn dirty(&self) {
		self.win.queue_draw();
	}
	
	pub fn render(&self, w: Widget, c: Context) {
		let (_alloc_w, _alloc_h) = (w.get_allocated_width(), w.get_allocated_height());
		
		const FONT_SIZE: f64 = 15.0;
		c.select_font_face("Helvetica", FontSlant::Normal, FontWeight::Normal);
		c.set_antialias(Antialias::Best);
		c.set_font_size(FONT_SIZE);
		c.new_path();
		
		{
			c.text_path("H");
			let ex = c.fill_extents();
			c.translate(FONT_SIZE * 0.1, FONT_SIZE + 10.0);
		}
		
		let start_offset = (FONT_SIZE / 2.0) + FONT_SIZE * ((self.edit.lines().len() as f64).log10().floor() + 1.0);
		for l in self.edit.lines().iter() {
			let mut undo_x = 0.0;
			{ // Render the line number (Align the points all at the same x co-ordinate)
				let s = format!("{}.", l.no + 1);
				c.new_path();
				c.text_path(&s);
				let p = c.copy_path();
				let ex = c.fill_extents();
				c.new_path();
				//println!("ex0: {}, ex1: {}, ex2: {}, ex3: {}", ex.0, ex.1, ex.2, ex.3);
				undo_x = start_offset - ex.2;
				c.translate(undo_x, 0.0);
				c.append_path(&p);
				c.fill();
			}
			
			{ // Render the `step` part of the line
				let s = l.step.to_gui_string();
				c.new_path();
				let trans_x = FONT_SIZE;
				undo_x += trans_x;
				c.translate(trans_x, 0.0);
				c.text_path(&s);
				c.fill();
			}
			
			c.translate(-undo_x, FONT_SIZE + 10.0);
		}
	}
}
