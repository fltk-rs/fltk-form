#[macro_use]
extern crate fltk_form_derive;

use fltk::{prelude::*, *};
use fltk_form::FltkForm;

#[derive(Copy, Debug, Clone, FltkForm)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[derive(Debug, Clone, FltkForm)]
pub struct MyStruct {
    label: &'static str,
    choices: Vec<MyEnum>,
}

impl MyStruct {
    pub fn default() -> Self {
        Self {
            label: "Choices",
            choices: vec![MyEnum::A; 5],
        }
    }
}

fn main() {
    let my_struct = MyStruct::default(); // <-- instantiate your struct

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_background_color(222, 222, 222);

    let mut win = window::Window::default().with_size(400, 400);
    let grp = group::Group::default()
        .with_size(300, 200)
        .center_of_parent();

    let mut form = my_struct.generate(); // <-- generate the form
    form.resize(grp.x() + 30, grp.y() - 30, grp.w() - 60, grp.h());

    grp.end();
    win.end();
    win.show();

    a.run().unwrap();
}
