
use gtk::{self, Button, Window, WindowPosition};
use gtk::signal::Inhibit;
use gtk::traits::*;

pub struct Gui {
	win: &'static mut Window
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
		
		let b = Button::new_with_label("Nope").unwrap();
		win.add(&b);
		
		win.show_all();
		
		Gui {
			win: win
		}
	}
}
