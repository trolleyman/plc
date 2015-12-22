
use edit::Editor;

use gtk::{self, Window, Frame, EventBox, DrawingArea, WindowPosition};
use gtk::signal::Inhibit;
use gtk::traits::*;
use gdk::EventType;

pub struct Gui {
	win: &'static mut Window,
	edit: Editor,
}
impl Gui {
	pub fn new(win: &'static mut Window) -> Gui {
		win.set_title("Propositional Logic Calculator");
		win.set_border_width(10);
		win.set_window_position(WindowPosition::Center);
		win.set_double_buffered(true);
		win.set_default_size(350, 70);
		
		win.connect_delete_event(|_, _| {
			gtk::main_quit();
			Inhibit(true)
		});
		
		let da_frame = Frame::new(None).unwrap();
		{
			let eb = EventBox::new().unwrap();
			win.connect_key_press_event(|_, e| {
				if e._type == EventType::KeyPress {
					::get_gui().map(|gui| { gui.edit.handle_input(e) } ).unwrap_or(Inhibit(false))
				} else {
					Inhibit(false)
				}
			});
			let da = DrawingArea::new().unwrap();
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
}
