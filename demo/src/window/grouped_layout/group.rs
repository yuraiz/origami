use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/grouped_layout/group.blp")]
    pub struct Group;

    #[glib::object_subclass]
    impl ObjectSubclass for Group {
        const NAME: &'static str = "OriGroup";
        type Type = super::Group;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Group {}
    impl WidgetImpl for Group {}
}

glib::wrapper! {
    pub struct Group(ObjectSubclass<imp::Group>)
        @extends gtk::Widget;
}
