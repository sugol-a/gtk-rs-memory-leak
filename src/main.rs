use gdk_pixbuf::{prelude::PixbufLoaderExt, PixbufLoader};
use gtk4 as gtk;
use gtk::gsk;
use gtk::gdk;
use gdk::prelude::*;
use gtk::graphene;
use gtk::prelude::*;

fn activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    let data = include_bytes!("testimage.png");
    for _i in 0..20 {
        let loader = PixbufLoader::new();
        let _ = loader.write(data);
        let _ = loader.close();

        let pixbuf = loader.pixbuf().unwrap();

        let texture = gdk::Texture::for_pixbuf(&pixbuf);
        let rect = graphene::Rect::new(0.0, 0.0, 100.0, 100.0);
        let texture_node = gsk::TextureNode::new(&texture, &rect);
        
        let radius = graphene::Size::new(8.0, 8.0);
        let clip = gsk::RoundedRect::new(rect, radius, radius, radius, radius);
        let clip_node = gsk::RoundedClipNode::new(texture_node, &clip);

        println!("{:?}", clip_node);
    }

    window.show();
}

fn main() {
    let app = gtk::Application::new(Some("org.example.test"), Default::default());
    app.connect_activate(|a| {
        activate(a);
    });

    app.run();
}
