
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
		
		c.select_font_face("CMU Serif", FontSlant::Normal, FontWeight::Normal);
		c.set_antialias(Antialias::Best);
		c.set_font_size(24.0);
		c.new_path();
		
		{
			c.text_path("H");
			let p = c.copy_path();
			let ex = c.fill_extents();
			println!("extent: {:?}", ex);
			c.translate(- ex.0 - 10.0, - ex.1 - 10.0);
		}
		
		for l in self.edit.lines().iter() {
			let s = format!("{:#}", l);
			c.text_path(&s);
			let p = c.copy_path();
			let ex = c.fill_extents();
			c.translate(-ex.0, -ex.1);
			c.new_path();
			c.append_path(&p);
			c.fill();
			break;
		}
	}
}
