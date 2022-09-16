#[allow(unused_imports)]
use gio::SimpleAction;
#[allow(unused_imports)]
use glib::clone;
use gtk::prelude::*;
use gtk::{
    gio, glib, Application, ApplicationWindow, Builder, ListBox, Label, Button
};

// use csv::Reader;
// use std::error::Error;

// #[allow(dead_code)]
// fn readfromcsv() -> Result<(), Box<dyn Error>> {
//     let mut rdr = Reader::from_path("data/EQUITY_L.csv")?;
//     for result in rdr.records() {
//         let record = result?;
//         println!("{:?}", record);
//     }
//     Ok(())
// }

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}
#[allow(unused_variables)]
fn build_ui(application: &Application) {
    let ui_src = include_str!("gtk_builder.ui");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let listbox: ListBox = builder.object("listboxIns").expect("Couldn't get list");
    let original_state = 0;
    let label: Label = builder.object("insTitle").expect("Couldn't get Title Label");

    let button: Button = builder
                            .object("belBtn")
                            .expect("Couldn't get BEL Button");
    button.connect_clicked(move |button| {
        let parameter = 500;
        button
            .activate_action("win.updateTitle", Some(&parameter.to_variant()))
            .expect("The action does not exist.");
        });
    let button: Button = builder
                            .object("adanigreenBtn")
                            .expect("Couldn't get ADANIGREEN Button");
    button.connect_clicked(move |_| flip_coin(&label));
    

    
    for number in 0..=100 {
        let button = Button::builder()
                        .label(&number.to_string())
                        // .action_name("win.updateTitle")
                        // .action_target(&number.to_variant())
                        .build();
        button.connect_clicked(move |button| {
        let parameter = 2;
        button
            .activate_action("win.updateTitle", Some(&parameter.to_variant()))
            .expect("The action does not exist.");
        });
        listbox.append(&button);
    }

    // Add action "count" to `window` taking an integer as parameter
    // let action_setnicename= SimpleAction::new_stateful(
    //     "setNiceName",
    //     Some(&i32::static_variant_type()),
    //     &original_state.to_variant(),
    // );
    // action_setnicename.connect_activate(clone!(@weak label => move |action, parameter| {

    // }));


    // let action_update = SimpleAction::new_stateful(
    //     "updateTitle",
    //     Some(&i32::static_variant_type()),
    //     &original_state.to_variant(),
    // );
    // action_update.connect_activate(clone!(@weak label => move |action, parameter| {
    //     // Get state
    //     let mut state = action
    //         .state()
    //         .expect("Could not get state.")
    //         .get::<i32>()
    //         .expect("The variant needs to be of type `i32`.");

    //     // Get parameter
    //     let parameter = parameter
    //         .expect("Could not get parameter.")
    //         .get::<i32>()
    //         .expect("The variant needs to be of type `i32`.");

    //     // Increase state by parameter and store state
    //     state += parameter;
    //     action.set_state(&state.to_variant());

    //     // Update label with new state
    //     label.set_label(&format!("Counter: {state}"));
    // }));
    // window.add_action(&action_update);

    window.show();
}



fn flip_coin(label: &Label) {
    label.set_text("ssssfffgg");
}