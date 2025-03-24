use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};
use relm4::*;

pub const NEWLINE: char = '\x0A';
pub const BACKSPACE: char = '\x08';

#[tracker::track]
struct Guess {
    word: String,
}

// #[derive(Clone, Copy, Debug, Default, PartialEq)]
// enum ButtonStatus {
//     #[default]
//     None,
//     Absent,
//     IncorrectPosition,
//     CorrectPosition,
// }

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ButtonModel {
    pub text: char,
    // row: usize,
    // status: ButtonStatus,
}

impl ButtonModel {
    pub fn new(text: char) -> ButtonModel {
        ButtonModel {
            text,
            // row,
            // status: ButtonStatus::None,
        }
    }

    pub fn get_str(&self) -> String {
        match self.text {
            NEWLINE => "Enter".to_string(),
            BACKSPACE => "<-".to_string(),
            _ => self.text.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ButtonPressedMsg {
    ButtonPressed,
    Enter,
    Backspace,
}

#[derive(Debug)]
pub struct CharEntered {
    pub text: char,
}

#[component(pub)]
impl SimpleComponent for ButtonModel {
    /// The type of data with which this component will be initialized.
    type Init = ButtonModel;
    /// The type of the messages that this component can receive.
    type Input = ButtonPressedMsg;
    /// The type of the messages that this component can send.
    type Output = CharEntered;

    view! {
        #[root]
        gtk::Button {
            set_label: model.get_str().as_str(),
            connect_clicked => match model.text {
                NEWLINE => ButtonPressedMsg::Enter,
                BACKSPACE => ButtonPressedMsg::Backspace,
                _ => ButtonPressedMsg::ButtonPressed,
            }
        }
    }

    // Initialize the UI.
    fn init(
        model: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
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
