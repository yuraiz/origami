mod grouped_layout;
mod loading_indicator;
mod shimmer_effect;
mod spoiler;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/window.blp")]
    pub struct SpoilerWindow {
        #[template_child]
        pub(super) stack: TemplateChild<gtk::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SpoilerWindow {
        const NAME: &'static str = "SpoilerWindow";
        type Type = super::SpoilerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            loading_indicator::LoadingIndicatorPage::static_type();
            shimmer_effect::ShimmerEffectPage::static_type();
            spoiler::SpoilerPage::static_type();
            grouped_layout::GroupedLayoutPage::static_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SpoilerWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().connect_close_request(|window| {
                window.application().unwrap().quit();
                gtk::Inhibit(true)
            });
        }
    }

    impl WidgetImpl for SpoilerWindow {}
    impl WindowImpl for SpoilerWindow {}
    impl ApplicationWindowImpl for SpoilerWindow {}
    impl AdwApplicationWindowImpl for SpoilerWindow {}
}

glib::wrapper! {
    pub struct SpoilerWindow(ObjectSubclass<imp::SpoilerWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionGroup, gio::ActionMap;
}

impl SpoilerWindow {
    pub fn new(app: &impl IsA<gio::Application>, page: Option<String>) -> Self {
        let obj: Self = glib::Object::builder().property("application", app).build();

        if let Some(name) = page {
            let pages: Vec<_> = obj
                .imp()
                .stack
                .pages()
                .iter::<gtk::StackPage>()
                .filter_map(|res| res.ok())
                .filter_map(|page| page.name())
                .collect();

            if pages.iter().any(|n| n == &name) {
                obj.imp().stack.set_visible_child_name(&name);
            } else {
                eprintln!("Page {name} is not available");
                eprintln!("Supported pages: {}", pages.join(", "));
            }
        }

        obj
    }
}
