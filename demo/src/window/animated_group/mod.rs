use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::{gdk, glib};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/window/animated_group/animated_group.blp")]
    pub struct AnimatedGroupPage {
        #[template_child]
        pub(super) group: TemplateChild<ori::AnimatedGroup>,
        #[template_child]
        pub(super) drop_target: TemplateChild<gtk::DropTarget>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AnimatedGroupPage {
        const NAME: &'static str = "OriDemoAnimatedGroupPage";
        type Type = super::AnimatedGroupPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AnimatedGroupPage {
        fn constructed(&self) {
            self.parent_constructed();

            self.drop_target.connect_drop(
                clone!(@to-owned self as imp => @default-return false, move
                    |_, value, _, _ | {
                        let Ok(file_list) = value.get::<gdk::FileList>() else { return false; };

                        let files = file_list.files();

                        let pictures = files.iter().map(|file| {
                            gtk::Picture::builder()
                            .file(file)
                            .content_fit(gtk::ContentFit::Cover)
                            .overflow(gtk::Overflow::Hidden)
                            .css_classes(["card"])
                            .build()
                        });

                        imp.group.remove_chldren();

                        pictures.for_each(|pic| {
                            imp.group.append(&pic);
                        });

                        true
                    }
                ),
            );
        }
    }
    impl WidgetImpl for AnimatedGroupPage {}
    impl BinImpl for AnimatedGroupPage {}
}

glib::wrapper! {
    pub struct AnimatedGroupPage(ObjectSubclass<imp::AnimatedGroupPage>)
        @extends gtk::Widget;
}
