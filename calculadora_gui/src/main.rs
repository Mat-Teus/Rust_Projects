use iced::{
    executor,
    Application, Command, Element, Settings, Theme,
    widget::{button, column, row, text},
    Length, Alignment,
};

fn main() -> iced::Result {
    App::run(Settings {
        window: window_settings(),
        ..Default::default()
    })
}

fn window_settings() -> iced::window::Settings {
    iced::window::Settings {
        size: iced::Size::new(400.0, 500.0),
        resizable: false,
        position: iced::window::Position::Centered,
        ..Default::default()
    }
}

struct App {
    display: String,
    valor: f64,
    operacao: Option<Operacao>,
}

#[derive(Debug, Clone, Copy)]
enum Operacao {
    Soma,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, Clone)]
enum Message {
    Numero(u8),
    Operacao(Operacao),
    Igual,
    Limpar,
    Ponto,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                display: "0".to_string(),
                valor: 0.0,
                operacao: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calculadora")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Numero(n) => {
                if self.display == "0" {
                    self.display = n.to_string();
                } else {
                    self.display.push_str(&n.to_string());
                }
            }

            Message::Operacao(op) => {
                self.valor = self.display.parse().unwrap_or(0.0);
                self.operacao = Some(op);
                self.display = "0".to_string();
            }

            Message::Igual => {
                let atual = self.display.parse::<f64>().unwrap_or(0.0);

                if let Some(op) = self.operacao {
                    let resultado = match op {
                        Operacao::Soma => self.valor + atual,
                        Operacao::Sub => self.valor - atual,
                        Operacao::Mult => self.valor * atual,
                        Operacao::Div => self.valor / atual,
                    };

                    if resultado.fract() == 0.0 {
                        self.display = format!("{:.2}", resultado);
                    } else {
                        self.display = resultado.to_string();
                    }
                }
            }

            Message::Ponto => {
                if !self.display.contains('.') {
                        if self.display.is_empty() {
                            self.display.push_str("0.");
                        } else {
                        self.display.push('.');
                    }
                }
}

            Message::Limpar => {
                self.display = "0".to_string();
                self.valor = 0.0;
                self.operacao = None;
            }
        }

        Command::none()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn view(&self) -> Element<Message> {
        column![
            text(&self.display)
                .size(35)
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Right),

            row![
                button(text("C"))
                    .width(Length::FillPortion(3))
                    .height(Length::Fixed(60.0))
                    .on_press(Message::Limpar),

                op_button("÷", Operacao::Div),
            ],

            row![num_button(7), num_button(8), num_button(9), op_button("×", Operacao::Mult)],
            row![num_button(4), num_button(5), num_button(6), op_button("-", Operacao::Sub)],
            row![num_button(1), num_button(2), num_button(3), op_button("+", Operacao::Soma)],

            row![
                button(text("0"))
                    .width(Length::FillPortion(2))
                    .height(Length::Fixed(60.0))
                    .on_press(Message::Numero(0)),

                button(text("."))
                    .width(Length::FillPortion(2))
                    .height(Length::Fixed(60.0))
                    .on_press(Message::Ponto),

                    button(text("="))
                    .width(Length::FillPortion(2))
                    .height(Length::Fixed(60.0))
                    .on_press(Message::Igual),
            ],
        ]
        .spacing(15)
        .padding(20)
        .into()
    }
}

// 🔘 botão número
fn num_button(n: u8) -> iced::widget::Button<'static, Message> {
    button(text(n.to_string()))
        .width(Length::Fill)
        .height(Length::Fixed(60.0)) // 👈 altura fixa
        .on_press(Message::Numero(n))
}

fn op_button(label: &str, op: Operacao) -> iced::widget::Button<'static, Message> {
    button(text(label))
        .width(Length::Fill)
        .height(Length::Fixed(60.0)) // 👈 altura fixa
        .on_press(Message::Operacao(op))
}