use rb62::get_hex;
use yew::{Component, ComponentLink, html, Html, InputData, KeyPressEvent, ShouldRender};
use yew::events::IKeyboardEvent;
use yew::services::ConsoleService;

struct App {
    clicked: bool,
    link: ComponentLink<Self>,
    text: String,
    hex_output: String,
    console: ConsoleService,
}

enum Msg {
    Click,
    Update(String),
    Enter,
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            text: "".to_string(),
            hex_output: "".to_string(),
            link: link,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                self.text = "6GGODyP2LIdbxIfYxy5UbN".to_string();
                self.update(Msg::Update(self.text.clone()));
                true // Indicate that the Component should re-render
            }
            Msg::Update(text) => {
                match get_hex(&text) {
                    Some(utf8s) => {
                        self.hex_output = std::str::from_utf8(&utf8s).unwrap().to_string();
                    }
                    None => {
                        self.hex_output = "Not valid input".to_string();
                    }
                }
                true
            }
            Msg::Enter => {
                self.console.log(&format!("You entered: '{}'", self.text));
                true
            }
            Msg::Nope => {
                false
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };

        html! {
            <body>
            <button onclick=&self.link.callback(|_| Msg::Click)>{ button_text }</button>
            <input placeholder="Input base62 encoding length 22"
                   size=32
                   value=&self.text
                   oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                   onkeypress=self.link.callback(|e: KeyPressEvent| {
                       if e.key() == "Enter" { Msg::Enter } else { Msg::Nope }
                   })
            />
            <br/>
            <label>{ &self.hex_output }</label>
            </body>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

// https://reqres.in/