#[macro_use]
extern crate fltk_form_derive;

use fltk::{prelude::*, *};
use fltk_form::{FlImage, FltkForm};

#[derive(Debug, Clone, FltkForm)]
pub struct MyStruct {
    empty: &'static str, // just to add spacing
    f: FlImage,
}

impl MyStruct {
    pub fn default() -> Self {
        Self {
            empty: "",
            f: FlImage(String::from("fltk-form/examples/orange_circle.svg")),
        }
    }
}

fn main() {
    let my_struct = MyStruct::default(); // <-- instantiate your struct

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_background_color(222, 222, 222);

    let mut win = window::Window::default().with_size(400, 300);
    let mut grp = group::Scroll::default()
        .with_size(300, 200)
        .center_of_parent();

    let mut form = my_struct.generate(); // <-- generate the form
    form.resize(form.x() - 50, form.y(), form.w() + 30, form.h());

    grp.end();
    grp.set_frame(enums::FrameType::EngravedFrame);
    win.end();
    win.show();

    while a.wait() {
        win.redraw();
    }
}
