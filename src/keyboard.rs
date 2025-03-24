use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

use crate::button::{self, ButtonModel, ButtonPressedMsg};

#[derive(Debug)]
pub enum KeyboardOutput {
    CharAdded,
    CharRemoved,
    WordAccepted,
}

#[relm4::factory(pub)]
impl FactoryComponent for ButtonModel {
    type Init = ButtonModel;
    type Input = ButtonPressedMsg;
    type Output = KeyboardOutput;
    type CommandOutput = KeyboardOutput;
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: &self.text.to_string(),
                set_width_chars: 3,
            },

            // #[name(r_button)]
            // gtk::Button {
            //     set_label: &self.get_str().as_str(),
            //     connect_clicked => match &self.text {
            //         &button::NEWLINE => ButtonPressedMsg::Enter,
            //         &button::BACKSPACE => ButtonPressedMsg::Backspace,
            //         _ => ButtonPressedMsg::ButtonPressed,
            //     },
            // },
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { text: value.text, row: value.row, status: value.status }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            ButtonPressedMsg::Enter => {
                println!("enter button pressed (from within the component): {}", self.get_str());
            },
            ButtonPressedMsg::Backspace => {
                println!("delete button pressed (from within the component): {}", self.get_str());
            },
            ButtonPressedMsg::ButtonPressed => {
                println!("regular button pressed (from within the component): {}", self.get_str());
            },
        }
    }
}