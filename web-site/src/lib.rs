use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    text: String,
}

enum Msg {
    AddOne,
    AddTwo,
    Key(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            text: String::from("Inicio"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::AddTwo => self.value += 2,
            Msg::Key(e) => self.text = e,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let input_typing = self.link.callback(|e: InputData| Msg::Key(e.value));
        html! {
            <div>
                <input type="text" oninput=input_typing/>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ self.value }</button>
                <button onclick=self.link.callback(|_| Msg::AddTwo)>{ self.value }</button>
                <p>{ self.value }</p>
                <p>{ self.text.clone() }</p>
                <hr/>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
