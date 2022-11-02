use yew::prelude::*;
use crate::components::{
    NewApplication,
    ListApplication,
};

pub struct Model {}

impl Component for Model {
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
            <div>
                <h2>{"Applications"}</h2>

                <NewApplication />

                <br />

                <ListApplication />
            </div>
        }
    }
}
