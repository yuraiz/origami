use adw::subclass::prelude::*;
use gtk::glib;

mod group;

use group::Group;

mod imp {
    use gtk::prelude::StaticType;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/grouped_layout/grouped_layout.blp")]
    pub struct GroupedLayoutPage;

    #[glib::object_subclass]
    impl ObjectSubclass for GroupedLayoutPage {
        const NAME: &'static str = "OriDemoGroupedLayoutPage";
        type Type = super::GroupedLayoutPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Group::static_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GroupedLayoutPage {}
    impl WidgetImpl for GroupedLayoutPage {}
    impl BinImpl for GroupedLayoutPage {}
}

glib::wrapper! {
    pub struct GroupedLayoutPage(ObjectSubclass<imp::GroupedLayoutPage>)
        @extends gtk::Widget;
}
