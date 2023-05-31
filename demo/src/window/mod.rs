mod animated_group;
mod group;
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
        pub(super) leaflet: TemplateChild<adw::Leaflet>,
        #[template_child]
        pub(super) sidebar: TemplateChild<gtk::Box>,
        #[template_child]
        pub(super) stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub(super) content: TemplateChild<gtk::Box>,
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
            group::GroupPage::static_type();
            animated_group::AnimatedGroupPage::static_type();

            klass.bind_template();
            klass.bind_template_callbacks();
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

    #[gtk::template_callbacks]
    impl SpoilerWindow {
        #[template_callback]
        pub(super) fn open_sidebar(&self) {
            self.leaflet.set_visible_child(&*self.sidebar);
        }

        #[template_callback]
        pub(super) fn open_content(&self) {
            self.leaflet.set_visible_child(&*self.content);
        }
    }
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
            let imp = obj.imp();
            let stack = &*imp.stack;

            let pages: Vec<_> = stack
                .pages()
                .iter::<gtk::StackPage>()
                .filter_map(|res| res.ok())
                .filter_map(|page| page.name())
                .collect();

            if pages.iter().any(|n| n == &name) {
                stack.set_visible_child_name(&name);
                imp.open_content();
            } else {
                eprintln!("Page {name} is not available");
                eprintln!("Supported pages: {}", pages.join(", "));
            }
        }

        obj
    }
}
