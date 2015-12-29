
use edit::Editor;

use std::fmt::Write;

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
		
		const FONT_SIZE: f64 = 17.0;
		const SCALE: f64 = FONT_SIZE * 1.0; // 1.0 for Helvetica, 
		c.select_font_face("Times New Roman", FontSlant::Normal, FontWeight::Normal);
		c.set_antialias(Antialias::Best);
		c.set_font_size(FONT_SIZE);
		c.new_path();
		c.translate((SCALE * 0.1).floor(), (SCALE + 10.0).floor());
		
		// Act like there are 10 lines in the proof for the sake of spacing.
		let lines_len = if self.edit.lines().len() < 10 { 10 } else { self.edit.lines().len() };
		let start_offset = ((SCALE / 2.5) + SCALE * 0.5 * ((lines_len as f64).log10().floor() + 1.0)).floor();
		c.translate(start_offset, 0.0);
		for l in self.edit.lines().iter() {
			let mut undo_x = 0.0;
			{ // Render the line number (Align the points all at the same x co-ordinate)
				let s = format!("{}.", l.no + 1);
				c.new_path();
				c.text_path(&s);
				let p = c.copy_path();
				let ex = c.fill_extents();
				c.new_path();
				//print!("ex0: {}, ex1: {}, ex2: {}, ex3: {}", ex.0, ex.1, ex.2, ex.3);
				let offset = -ex.2.floor();
				//println!(", undo_x: {}", undo_x);
				c.translate(offset, 0.0);
				c.append_path(&p);
				c.translate(-offset, 0.0);
				undo_x = 0.0;
				c.fill();
			}
			
			{ // Render the `step` part of the line
				let s = l.step.to_gui_string(true);
				c.new_path();
				let trans_x = SCALE * 0.5;
				undo_x += trans_x;
				c.translate(trans_x, 0.0);
				c.text_path(&s);
				c.fill();
			}
			
			{ // Render the `method` part of the line
				let s = l.method.to_gui_string(false);
				c.new_path();
				let trans_x = SCALE * 20.0;
				undo_x += trans_x;
				c.translate(trans_x, 0.0);
				c.text_path(&s);
				c.fill();
			}
			
			{ // Render the dependencies of the line
				let mut s = String::with_capacity(32);
				s.push('{');
				let max = l.deps.len().wrapping_sub(1);
				for (i, dep) in l.deps.iter().enumerate() {
					let _ = write!(s, "{}", dep + 1);
					if i != max {
						s.push(',');
						s.push(' ');
					}
				}
				s.push('}');
				let trans_x = SCALE * 8.0;
				undo_x += trans_x;
				c.translate(trans_x, 0.0);
				c.text_path(&s);
				c.fill();
			}
			
			c.translate(-undo_x, SCALE + 10.0);
		}
	}
}
