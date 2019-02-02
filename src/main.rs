#[macro_use]
extern crate yew;
use yew::prelude::*;
 
struct Model {
}
 
impl Component for Model {
    type Message = ();
    type Properties = ();
 
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { }
    }
 
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
}
 
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>{ "Hello, world!" }</div>
        }
    }
}
 
fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
