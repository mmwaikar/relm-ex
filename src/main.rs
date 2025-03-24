mod button;
mod keyboard;
// mod keyboard_row;

use button::ButtonModel;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use keyboard::KeyboardOutput;
use prelude::FactoryVecDeque;
use relm4::*;
use relm4::{
    ComponentParts, ComponentSender, Controller, SimpleComponent,
};

#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

struct AppModel {
    counter: u8,
    buttons: FactoryVecDeque<ButtonModel>,
}

#[derive(Debug)]
enum AppMsg {
    CharAdded,
    CharRemoved,
    WordAccepted,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5,
                },

                #[local_ref]
                counter_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let buttons = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                KeyboardOutput::CharAdded => AppMsg::CharAdded,
                KeyboardOutput::CharRemoved => AppMsg::CharRemoved,
                KeyboardOutput::WordAccepted => AppMsg::WordAccepted,
            });

        let model = AppModel {
            counter,
            buttons,
        };

        let counter_box = model.buttons.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::CharAdded => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::CharRemoved => {
                self.counter = self.counter.wrapping_sub(1);
            }
            AppMsg::WordAccepted => {
                println!("word accepted");
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<AppModel>(0);
}
