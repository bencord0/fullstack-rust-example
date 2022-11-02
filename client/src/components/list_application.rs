use yew::prelude::*;

pub struct ListApplication;

impl Component for ListApplication {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }
}
