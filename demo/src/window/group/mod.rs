use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/group/group.blp")]
    pub struct GroupPage;

    #[glib::object_subclass]
    impl ObjectSubclass for GroupPage {
        const NAME: &'static str = "OriDemoGroupPage";
        type Type = super::GroupPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GroupPage {}
    impl WidgetImpl for GroupPage {}
    impl BinImpl for GroupPage {}
}

glib::wrapper! {
    pub struct GroupPage(ObjectSubclass<imp::GroupPage>)
        @extends gtk::Widget;
}
