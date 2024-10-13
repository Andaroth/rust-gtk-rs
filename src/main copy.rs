use std::cell::Cell;
use std::rc::Rc;

use gtk::gdk;
use gtk::{
    glib,
    Application,
    ApplicationWindow,
    Button,
    Orientation
};
use gtk::prelude::*;

static LOGO_SVG: &[u8] = include_bytes!("logo_andatape.svg");

fn main() {
    let app = Application::builder()
        .application_id("com.andapirate.gtk-rs")
        .build();

    app.connect_activate(|app| {
        let button_increase = Button::builder()
            .label("+")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        let button_decrease = Button::builder()
            .label("-")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        
        let number = Rc::new(Cell::new(0));

        // Connect callbacks
        // When a button is clicked, `number` and label of the other button will be changed
        &button_increase.connect_clicked(glib::clone!(
            #[weak]
            number,
            #[weak]
            button_increase,
            #[weak]
            button_decrease,
            move |_| {
                number.set(number.get() + 1);
                &button_decrease.set_label(&number.get().to_string());
                &button_increase.set_label("+");
            }
        ));
        &button_decrease.connect_clicked(glib::clone!(
            #[weak]
            button_increase,
            #[weak]
            button_decrease,
            move |_| {
                number.set(number.get() - 1);
                &button_increase.set_label(&number.get().to_string());
                &button_decrease.set_label("-");
            }
        ));

        let container = gtk::Box::builder() // box container
            .orientation(Orientation::Vertical) // as a column
            .build();
        container.append(&button_increase); // add content in column
        container.append(&button_decrease); // add content in column
        // render window with container
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Anda Pirate")
            .child(&container)
            .build();

        window.present()
    });

    app.run();
}
