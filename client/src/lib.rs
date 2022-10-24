use wasm_bindgen::prelude::*;
use yew::prelude::*;

enum Message {
    AddOne,
}

struct Model {
    value: u64,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Message::AddOne)}>
                    { "+1" }
                </button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    yew::start_app::<Model>();

    Ok(())
}
