extern crate libappindicator_sys;
extern crate glib;
extern crate gtk;
extern crate gtk_sys;

pub use libappindicator_sys::*;
use libappindicator_sys::{AppIndicator as AppIndicatorRaw};
use glib::translate::{ToGlibPtr};

pub struct AppIndicator {
    air: *mut AppIndicatorRaw
}

impl AppIndicator {
    pub fn new(title: &str, icon: &str) -> AppIndicator {
        AppIndicator {
            air: unsafe {
                app_indicator_new(title.to_glib_none().0,
                                  icon.to_glib_none().0,
                                  AppIndicatorCategory::APP_INDICATOR_CATEGORY_APPLICATION_STATUS)
            }
        }
    }

    pub fn set_status(&mut self, status: AppIndicatorStatus) {
        unsafe {
            app_indicator_set_status(self.air, status);
        }
    }

    pub fn set_menu(&mut self, menu: &mut gtk::Menu) {
        unsafe {
            app_indicator_set_menu(self.air, menu.to_glib_none().0);
        }
    }

    pub fn set_label(&mut self, label: &str, guide: &str) {
        unsafe {
            app_indicator_set_label(self.air, label.to_glib_none().0, guide.to_glib_none().0);
        }
    }

    pub fn set_icon_full(&mut self, name: &str, desc: &str) {
        unsafe {
            app_indicator_set_icon_full(self.air, name.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    pub fn set_attention_icon_full(&mut self, name: &str, desc: &str) {
        unsafe {
            app_indicator_set_attention_icon_full(self.air, name.to_glib_none().0, desc.to_glib_none().0);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
