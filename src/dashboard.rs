use gio::prelude::*;
use gtk::prelude::*;

use crate::header::HeaderView;
use crate::style::BASE_STYLE;
use crate::system::SystemInfo;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

pub(crate) struct Dashboard {
    app: gtk::Application,
}

impl Dashboard {
    pub(crate) fn new() -> Self {
        let app = gtk::Application::new(Some("org.dancrhorton.sys-dash.rs"), Default::default())
            .expect("Initialization failed...");

        app.connect_activate(|_| {});

        Self { app }
    }

    pub(crate) fn run(&mut self) {
        self.app.connect_startup(move |app| {
            let provider = gtk::CssProvider::new();
            provider.load_from_path("custom.css").unwrap_or_default();
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );

            let provider = gtk::CssProvider::new();
            provider
                .load_from_data(BASE_STYLE.as_bytes())
                .expect("Failed to load CSS");
            // // We give the CssProvided to the default screen so the CSS rules we added
            // // can be applied to our window.
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            thread::spawn(move || loop {
                let _ = tx.send(1);
                thread::sleep(Duration::from_millis(500))
            });

            let system_info = RefCell::new(SystemInfo::new());
            let widgets = Rc::new(Widgets::new(&app));

            rx.attach(None, move |_| {
                update(&system_info, &widgets);
                glib::Continue(true)
            });
        });

        self.app.run(&Vec::new());
    }

    pub(crate) fn destroy(&self) {}
}

fn update(system_info: &RefCell<SystemInfo>, widgets: &Rc<Widgets>) {
    system_info.borrow_mut().update();
    let system_info = system_info.borrow();
    widgets.header.update(&system_info);
}

struct Widgets {
    _mwnd: gtk::ApplicationWindow,
    header: HeaderView,
}

impl Widgets {
    fn new(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);

        window.set_title("System Dashboard");
        window.set_icon_name(Some("application-default-icon"));
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);

        let header = HeaderView::new();

        let widgets_grid = gtk::GridBuilder::new()
            .row_spacing(12)
            .vexpand(true)
            .hexpand(true)
            .build();

        let main_view_box = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .build();

        main_view_box.pack_start(header.widget(), false, false, 0);
        main_view_box.pack_start(&widgets_grid, false, false, 0);

        window.add(&main_view_box);
        window.show_all();

        Self {
            _mwnd: window,
            header,
        }
    }
}
