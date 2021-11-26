#[macro_use]
extern crate fltk_form_derive;

use fltk::{prelude::*, *};
use fltk_form::{FltkForm, HasProps, FlImage};

#[derive(Copy, Debug, Clone, FltkForm)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[derive(Debug, Clone, FltkForm)]
pub struct MyStruct {
    a: f64,
    b: f64,
    c: String,
    d: MyEnum,
    e: bool,
    f:FlImage,
}

impl MyStruct {
    pub fn default() -> Self {
        Self {
            a: 0.0,
            b: 3.0,
            c: String::from("fltk-rs"),
            d: MyEnum::A,
            e: true,
            f:FlImage(String::from("fltk-form/examples/orange_circle.svg")),
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
    let mut btn = button::Button::default()
        .with_label("print")
        .with_size(80, 30)
        .below_of(&grp, 5)
        .center_x(&grp);
    win.end();
    win.show();

    let v = form.get_prop("b"); // <-- get a single property
    assert_eq!(v, Some("3.0".to_owned()));

    btn.set_callback(move |_| {
        println!("{:?}", form.get_props()); // <-- get a HashMap of the properties
    });

    while a.wait(){
        win.redraw();
    }
}
