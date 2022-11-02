use yew::prelude::*;

pub struct NewApplication {}

impl Component for NewApplication {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <form>
                <label>{"App Name: "}<input type="text" /></label>
            </form>
        }
    }
}
