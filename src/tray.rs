use systray;
use sharexin;

pub fn tray() {
    let mut app = match systray::Application::new() {
        Ok(w) => w,
        Err(_) => panic!("Can't create systray!"),
    };
    app.set_icon_from_file(&"/usr/share/icons/gnome/48x48/apps/text-editor.png"
        .to_string())
        .ok();
    app.add_menu_item(&"Tweet".to_string(), move |_| { sharexin::tweet(); })
        .ok();
    app.add_menu_item(&"Toot".to_string(), move |_| { sharexin::toot(); })
        .ok();
    app.add_menu_separator().ok();
    /*app.add_menu_item(&"Area - Tweet".to_string(), move |_| {
        image::image(0);
        dialog(twitter, true);
    }).ok();
    app.add_menu_item(&"Area - Toot".to_string(), move |_| {
        image::image(0);
        dialog(mastodon, true);
    }).ok();
    app.add_menu_item(&"Area - Imgur".to_string(), move |_| {
        image::image(0);
        imgur::send();
    }).ok();
    app.add_menu_separator().ok();
    app.add_menu_item(&"Window - Tweet".to_string(), move |_| {
        image::image(1);
        dialog(twitter, true);
    }).ok();
    app.add_menu_item(&"Window - Toot".to_string(), move |_| {
        image::image(1);
        dialog(mastodon, true);
    }).ok();
    app.add_menu_item(&"Window - Imgur".to_string(), move |_| {
        image::image(1);
        imgur::send();
    }).ok();
    app.add_menu_separator().ok();
    app.add_menu_item(&"Full - Tweet".to_string(), move |_| {
        image::image(2);
        dialog(twitter, true);
    }).ok();
    app.add_menu_item(&"Full - Toot".to_string(), move |_| {
        image::image(2);
        dialog(mastodon, true);
    }).ok();
    app.add_menu_item(&"Full - Imgur".to_string(), move |_| {
        image::image(2);
        imgur::send();
    }).ok();
    app.add_menu_separator().ok();*/
    app.add_menu_item(&"Quit".to_string(), |window| { window.quit(); })
        .ok();
    app.wait_for_message();
}
