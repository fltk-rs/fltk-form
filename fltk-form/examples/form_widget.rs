#[macro_use]
extern crate fltk_form_derive;

use fltk::{prelude::*, *};
use fltk_form::{Form, FltkForm};

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
}

impl MyStruct {
    pub fn new() -> Self {
        Self {
            a: 0.0,
            b: 3.0,
            c: String::new(),
            d: MyEnum::A,
            e: true,
        }
    }
}

fn main() {
    let my_struct = MyStruct::new(); // <-- instantiate your struct

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_background_color(222, 222, 222);

    let mut win = window::Window::default().with_size(400, 300);

    let form = Form::default().with_size(300, 200).center_of_parent().from_data(my_struct);
    form.end();
    
    let mut btn = button::Button::default()
        .with_label("print")
        .with_size(80, 30)
        .below_of(&*form, 5)
        .center_x(&*form);
    win.end();
    win.show();

    let v = form.get_prop("b"); // <-- get a single property
    assert_eq!(v, Some("3.0".to_owned()));

    btn.set_callback(move |_| {
        println!("{:?}", form.get_props()); // <-- get a HashMap of the properties
    });

    a.run().unwrap();
}