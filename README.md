# fltk-form

This crate aims to simplify generating gui from a data structure.

## Usage
(Not yet released, so the following will not work)

```toml
[dependencies]
fltk = 1.2.16
fltk-form = 0.1
fltk-form-derive = 0.1
```

But you can git clone the repo and run the example directly:
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

![alt_test](https://github.com/MoAlyousef/fltk-form/raw/master/screenshots/form.jpg)
