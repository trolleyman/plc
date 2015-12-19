#![feature(const_fn)]
extern crate gtk;
//extern crate gtk_sys;

extern crate logic;

use std::{mem, ptr};
use gtk::{Window, WindowType};
use logic::prelude::*;
use gui::Gui;

pub mod gui;

static mut g_gui: *mut Gui = ptr::null_mut();

pub fn get_gui() -> Option<Gui> {
	unsafe { if g_gui.is_null() {
		None
	} else {
		Some(mem::transmute(g_gui))
	}}
}

fn main() {
	match gtk::init() {
		Err(()) => panic!("GTK cannot be initialized."),
		_ => {}
	}
	unsafe {
		let mut win = Window::new(WindowType::Toplevel).expect("Window could not be initialized.");
		let mut gui = Gui::new(mem::transmute(mem::transmute::<_, *mut char>(&mut win)));
		
		g_gui = &mut gui;
		
		gtk::main();
	}
}
