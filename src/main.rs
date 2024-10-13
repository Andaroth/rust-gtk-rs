use std::cell::Cell;
use std::rc::Rc;

use gtk::gdk;
use gtk::{
    glib,
    Application,
    ApplicationWindow,
    Button,
    Orientation,
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
        let button_about = Button::builder()
            .label("About")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        
        let number = Rc::new(Cell::new(0));

        // Connect callbacks
        // When a button is clicked, `number` and label of the other button will be changed
        let _ = button_increase.connect_clicked(glib::clone!(
            #[weak]
            number,
            #[weak]
            button_increase,
            #[weak]
            button_decrease,
            move |_| {
                number.set(number.get() + 1);
                let _ = &button_decrease.set_label(&number.get().to_string());
                let _ = &button_increase.set_label("+");
            }
        ));
        let _ = button_decrease.connect_clicked(glib::clone!(
            #[weak]
            button_increase,
            #[weak]
            button_decrease,
            move |_| {
                number.set(number.get() - 1);
                let _ = &button_increase.set_label(&number.get().to_string());
                let _ = &button_decrease.set_label("-");
            }
        ));

        let bytes = glib::Bytes::from_static(LOGO_SVG);
            let logo = gdk::Texture::from_bytes(&bytes).expect("logo-andatape.svg to load");
            let dialog = gtk::AboutDialog::builder()
                .program_name("About")
                .version("0.1.0")
                .website("https://andapirate.com")
                .license_type(gtk::License::MitX11)
                .authors(["Axel Andaroth", "HEXOFO"])
                .logo(&logo)
                .build();
        let _ = button_about.connect_clicked(move |_| dialog.present());

        let container = gtk::Box::builder() // box container
            .orientation(Orientation::Vertical) // as a column
            .build();
        container.append(&button_increase); // add content in column
        container.append(&button_decrease); // add content in column
        container.append(&button_about); // add content in column
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