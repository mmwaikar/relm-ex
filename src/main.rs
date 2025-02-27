use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

const NEWLINE: char = '\x0A';
const BACKSPACE: char = '\x08';

#[derive(Clone, Copy, Default, PartialEq)]
pub enum AlphabetStatus {
    #[default]
    None,
    Absent,
    IncorrectPosition,
    CorrectPosition,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Key {
    pub key: char,
    pub row: usize,
    pub status: AlphabetStatus,
}

impl Key {
    fn new(key: char, row: usize) -> Key {
        Key {
            key,
            row,
            status: AlphabetStatus::None,
        }
    }

    fn get_str(&self) -> String {
        match self.key {
            NEWLINE => "Enter".to_string(),
            BACKSPACE => "<-".to_string(),
            _ => self.key.to_string(),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
struct KeyboardModel {
    pub first_row: Vec<Key>,
    pub second_row: Vec<Key>,
    pub third_row: Vec<Key>,
}

impl KeyboardModel {
    fn init() -> KeyboardModel {

        const FIRST_ROW: &str = "qwertyuiop";
        const SECOND_ROW: &str = "asdfghjkl";
        const THIRD_ROW: &str = "zxcvbnm";

        let mut tr_chars = THIRD_ROW.chars().collect::<Vec<char>>();
        tr_chars.push(BACKSPACE);
        tr_chars.insert(0, NEWLINE);
        dbg!(&tr_chars);

        let first_row: Vec<Key> = FIRST_ROW.chars().map(|c| Key::new(c, 1)).collect();
        let second_row: Vec<Key> = SECOND_ROW.chars().map(|c| Key::new(c, 2)).collect();
        let third_row: Vec<Key> = tr_chars.into_iter().map(|c| Key::new(c, 3)).collect();

        KeyboardModel {
            first_row,
            second_row,
            third_row,
        }
    }
}

#[derive(Debug)]
pub enum KeyboardMsg {
    KeyPressed(char),
    // Enter,
    // Backspace,
}

struct AppWidgets {
    label: gtk::Label,
}

impl SimpleComponent for KeyboardModel {
    /// The type of the messages that this component can receive.
    type Input = KeyboardMsg;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = KeyboardModel;
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Wordle")
            .default_width(300)
            .default_height(500)
            .build()
    }

    /// Initialize the UI and model.
    fn init(
        kb_model: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = KeyboardModel::init();

        let label = gtk::Label::new(Some(&format!("Counter:")));
        label.set_margin_all(5);

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let f_hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();
        f_hbox.set_margin_all(5);
        f_hbox.set_align(gtk::Align::Center);

        for &key in &model.first_row {
            let button = gtk::Button::with_label(&key.get_str());
            button.connect_clicked(clone!(
                #[strong]
                sender,
                move |_| {
                    sender.input(KeyboardMsg::KeyPressed(' '));
                }
            ));

            f_hbox.append(&button);
        }

        let s_hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();
        s_hbox.set_margin_all(5);
        s_hbox.set_align(gtk::Align::Center);

        for &key in &model.second_row {
            let button = gtk::Button::with_label(&key.get_str());
            button.connect_clicked(clone!(
                #[strong]
                sender,
                move |_| {
                    sender.input(KeyboardMsg::KeyPressed(' '));
                }
            ));

            s_hbox.append(&button);
        }

        let t_hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();
        t_hbox.set_margin_all(5);
        t_hbox.set_align(gtk::Align::Center);

        for &key in &model.third_row {
            let button = gtk::Button::with_label(&key.get_str());
            button.connect_clicked(clone!(
                #[strong]
                sender,
                move |_| {
                    sender.input(KeyboardMsg::KeyPressed(' '));
                }
            ));

            t_hbox.append(&button);
        }

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.set_align(gtk::Align::Center);
        vbox.append(&f_hbox);
        vbox.append(&s_hbox);
        vbox.append(&t_hbox);

        let widgets = AppWidgets { label };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            KeyboardMsg::KeyPressed(c) => {
                println!("key pressed: {}", c);
                // self.counter = self.counter.wrapping_add(1);
            } // KeyboardMsg::Decrement => {
              //     self.counter = self.counter.wrapping_sub(1);
              // }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<KeyboardModel>(KeyboardModel::init());
}
