use fltk::{prelude::*, *};
use fltk_form::{FltkForm, HasProps};

#[derive(Debug, Clone)]
pub struct MyStruct<T> {
    a: T,
    b: f64,
    c: String,
}

impl<T: Copy + Default + FltkForm> MyStruct<T> {
    pub fn default() -> Self {
        Self {
            a: T::default(),
            b: 3.0,
            c: String::new(),
        }
    }
}

impl<T: Copy + Default + FltkForm> FltkForm for MyStruct<T> {
    fn generate(&self) -> Box<dyn WidgetExt> {
        let mut p = group::Pack::default()
            .with_label(&format!("{}", "MyStruct"))
            .with_align(fltk::enums::Align::Left | fltk::enums::Align::Top);
        p.set_spacing(5);
        let mut i = self.a.generate();
        {
            i.set_align(fltk::enums::Align::Left);
            i.set_label("a");
        }
        let mut i = self.b.generate();
        {
            i.set_align(fltk::enums::Align::Left);
            i.set_label("b");
        }
        let mut i = self.c.generate();
        {
            i.set_align(fltk::enums::Align::Left);
            i.set_label("c");
        }
        p.end();
        let parent = p.parent().unwrap();
        p.resize(
            parent.x() + (parent.width() / 2),
            parent.y() + parent.h() / 9,
            parent.width() / 3,
            (3 * 30 + 5 * 3) as i32,
        );
        p.auto_layout();
        Box::new(p)
    }
    fn view(&self) -> Box<dyn WidgetExt> {
        let mut p = group::Pack::default()
            .with_label(&format!("{}", "MyStruct"))
            .with_align(fltk::enums::Align::Left | fltk::enums::Align::Top);
        p.set_spacing(5);
        let mut i = self.a.view();
        {
            i.set_align(fltk::enums::Align::Left);
            i.set_label("a");
        }
        let mut i = self.b.view();
        {
            i.set_align(fltk::enums::Align::Left);
            i.set_label("b");
        }
        let mut i = self.c.view();
        {
            i.set_align(fltk::enums::Align::Left);
            i.set_label("c");
        }
        p.end();
        let parent = p.parent().unwrap();
        p.resize(
            parent.x() + (parent.width() / 2),
            parent.y() + parent.h() / 9,
            parent.width() / 3,
            (3 * 30 + 5 * 3) as i32,
        );
        p.auto_layout();
        Box::new(p)
    }
}

fn main() {
    let my_struct = MyStruct::<f64>::default(); // <-- instantiate your struct

    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::set_background_color(222, 222, 222);

    let mut win = window::Window::default().with_size(400, 300);
    let mut grp = group::Group::default()
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

    a.run().unwrap();
}
