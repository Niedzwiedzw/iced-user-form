use iced::{text_input, Button, Column, Element, Row, Sandbox, Settings, Text, TextInput};

#[derive(Debug)]
struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default)]
struct UserList {
    pub show_form_button: iced::button::State,
    pub show_form: bool,
    pub users: Vec<User>,
    pub username_input: text_input::State,
    pub password_input: text_input::State,
    pub username_input_value: String,
    pub password_input_value: String,
    pub add_user_button: iced::button::State,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    ShowFormClicked,
    UsernameChanged(String),
    PasswordChanged(String),
    SaveNewUser,
}

impl Sandbox for UserList {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "User list with form validation popup".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ShowFormClicked => self.show_form = !self.show_form,
            Message::UsernameChanged(v) => self.username_input_value = v,
            Message::PasswordChanged(v) => self.password_input_value = v,
            Message::SaveNewUser => {
                let user = User {
                    username: self.username_input_value.clone(),
                    password: self.password_input_value.clone(),
                };
                if user.username == user.password {
                    self.error = Some("username and password cannot be the same".into());
                } else {
                    self.error = None;
                }

                if self.error.is_some() {
                    return;
                }
                self.users.push(user);
                self.username_input_value.clear();
                self.password_input_value.clear();
                self.show_form = false;
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let data_container: Element<Self::Message> = if self.show_form {
            let column = Column::new()
                .padding(20)
                .align_items(iced::Alignment::Center)
                .push(TextInput::new(
                    &mut self.username_input,
                    "Username",
                    &self.username_input_value,
                    Self::Message::UsernameChanged,
                ))
                .push(TextInput::new(
                    &mut self.password_input,
                    "Password",
                    &self.password_input_value,
                    Self::Message::PasswordChanged,
                ))
                .push(
                    Button::new(&mut self.add_user_button, Text::new("add user"))
                        .on_press(Message::SaveNewUser),
                );
            if let Some(error) = self.error.as_ref() {
                return column
                    .push(Text::new(error.clone()).color(iced::Color::from_rgba(200., 0., 0., 0.8)))
                    .into();
            }
            column.into()
        } else {
            let users =
                self.users
                    .iter_mut()
                    .fold(Column::new(), |column, User { username, password }| {
                        let password = (0..password.len()).map(|_| '*').collect::<String>();
                        column.push(Text::new(format!("{username} / {password}",)))
                    });
            users.into()
        };
        Column::new()
            .padding(20)
            .align_items(iced::Alignment::Center)
            // .push(Text::new(format!("{:?}", &self)))
            .push(
                Button::new(
                    &mut self.show_form_button,
                    Row::new().spacing(10).push(Text::new("Add user")),
                )
                .on_press(Self::Message::ShowFormClicked),
            )
            .push(data_container)
            .into()
    }
}

fn main() -> iced::Result {
    UserList::run(Settings::default())
}
