extern crate gtk_sys;
extern crate gtk;
extern crate libappindicator;

use gtk::{WidgetExt, MenuShellExt, MenuItemExt};
use libappindicator::{AppIndicator, AppIndicatorStatus};

fn main() {
    gtk::init().unwrap();
    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
    indicator.set_icon_full("/usr/share/gxkb/flags/ua.png", "icon");
    let mut m = gtk::Menu::new();
    let mi = gtk::ImageMenuItem::new_from_stock(gtk_sys::GTK_STOCK_QUIT, None);
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    m.append(&mi);
    indicator.set_menu(&mut m);
    m.show_all();
    gtk::main();
}
