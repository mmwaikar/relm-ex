mod keyboard;

use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, Controller, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::*;

const NEWLINE: char = '\x0A';
const BACKSPACE: char = '\x08';

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
struct ButtonModel {
    text: char,
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
    // Enter,
    // Backspace,
}

#[derive(Debug)]
pub struct CharInput {
    pub text: char,
}

#[relm4::component]
impl SimpleComponent for ButtonModel {
    /// The type of data with which this component will be initialized.
    type Init = char;
    /// The type of the messages that this component can receive.
    type Input = ButtonPressedMsg;
    /// The type of the messages that this component can send.
    type Output = CharInput;

    view! {
        #[root]
        gtk::Button {
            set_label: model.text.to_string().as_str(),
            connect_clicked => ButtonPressedMsg::ButtonPressed
        }
    }

    // Initialize the UI.
    fn init(
        button_text: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ButtonModel { text: button_text };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ButtonPressedMsg::ButtonPressed => {
                println!("key pressed: {}", self.text);
                // self.counter = self.counter.wrapping_add(1);
            }
        }
    }
}

#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

struct AppModel {
    counter: u8,
    button: Controller<ButtonModel>,
}

#[derive(Debug)]
enum AppMsg {
    SetMode(AppMode),
    Increment,
    Decrement,
}

#[relm4::component]
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

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked => AppMsg::Increment
                },

                gtk::Button::with_label("Decrement") {
                    connect_clicked => AppMsg::Decrement
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5,
                },

                model.button.widget(),
            }
        }
    }

    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let button: Controller<ButtonModel> =
            ButtonModel::builder()
                .launch('w')
                .forward(sender.input_sender(), |msg| {
                    println!("key pressed: {}", msg.text);
                    AppMsg::SetMode(AppMode::View)
                });

        let model = AppModel { counter, button };

        // Insert the macro code generation here
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
            AppMsg::SetMode(m) => {
                println!("set mode: {:?}", m);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<AppModel>(0);
}
