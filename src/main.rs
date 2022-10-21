use gtk::prelude::*;
use gtk::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.otnemio.vyaapaar"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &Application) {
    let ui_src = include_str!("gtk_builder.ui");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    window.show();
}
