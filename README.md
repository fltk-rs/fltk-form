# fltk-form

This crate aims to simplify generating gui from a data structure.

## Usage
```toml
[dependencies]
fltk = "1.2.16"
fltk-form = "0.1"
fltk-form-derive = "0.1"
```

You can also `git clone` the repo and run the example directly:
```
$ cargo run --example basic
```

## Example
```rust
#[macro_use]
extern crate fltk_form_derive;

use fltk::{prelude::*, *};
use fltk_form::{FltkForm, HasProps};

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
    pub fn default() -> Self {
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
    let my_struct = MyStruct::default(); // <-- instantiate your struct

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_background_color(222, 222, 222);

    let mut win = window::Window::default().with_size(400, 300);
    let mut grp = group::Group::default()
        .with_size(300, 200)
        .center_of_parent();

    let form = my_struct.generate(); // <-- generate the form
    
    grp.end();
    let mut btn = button::Button::default()
        .with_label("print")
        .with_size(80, 30)
        .below_of(&grp, 5)
        .center_x(&grp);
    grp.set_frame(enums::FrameType::EngravedFrame);
    win.end();
    win.show();

    let v = form.get_prop("b"); // <-- get a single property
    assert_eq!(v, Some("3.0".to_owned()));

    btn.set_callback(move |_| {
        println!("{:?}", form.get_props()); // <-- get a HashMap of the properties
    });

    a.run().unwrap();
}
```

![alt_test](https://github.com/fltk-rs/fltk-form/raw/main/screenshots/form.jpg)


You can also rename properties using the rename_prop() method:
```rust
#[macro_use]
extern crate fltk_form_derive;

use fltk::{prelude::*, *};
use fltk_form::{FltkForm, HasProps};

#[derive(Copy, Debug, Clone, FltkForm)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, FltkForm)]
pub struct MyStruct {
    very_long_name: f64,
    second_value: f64,
    full_name: String,
    Choices: MyEnum,
    do_it: bool,
}

impl MyStruct {
    pub fn default() -> Self {
        Self {
            very_long_name: 0.0,
            second_value: 3.0,
            full_name: String::new(),
            Choices: MyEnum::A,
            do_it: true,
        }
    }
}

fn main() {
    let my_struct = MyStruct::default();

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_background_color(222, 222, 222);

    let mut win = window::Window::default().with_size(400, 300);
    let mut grp = group::Group::default()
        .with_size(300, 200)
        .center_of_parent()
        .with_label("Form 1")
        .with_align(enums::Align::Top | enums::Align::Left);

    let mut form = my_struct.generate();
    form.set_label("");
    form.rename_prop("very_long_name", "First Value");
    form.rename_prop("second_value", "Second Value");
    form.rename_prop("full_name", "Full name");
    form.rename_prop("do_it", "Do it?");

    grp.end();
    grp.set_frame(enums::FrameType::EngravedFrame);
    let mut btn = button::Button::default()
        .with_label("print")
        .with_size(80, 30)
        .below_of(&grp, 5)
        .center_x(&grp);
    win.end();
    win.show();

    let v = form.get_prop("First Value");
    assert_eq!(v, Some("0.0".to_owned()));

    btn.set_callback(move |_| {
        println!("{:?}", form.get_props());
    });

    a.run().unwrap();
}
```
![alt_test](https://github.com/fltk-rs/fltk-form/raw/main/screenshots/form2.jpg)
