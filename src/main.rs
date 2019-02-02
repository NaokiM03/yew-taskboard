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
            <section class="section", id="board",>
                <div class="container",>
                    <div class="columns",>
                        <div class="column status-1",>
                            <div class="tags has-addons",>
                                <span class="tag",>{ "未処理" }</span>
                                <span class="tag is-dark",>{ 0 } </span>
                            </div>
                        </div>
                    </div>
                </div>
             </section>
        }
    }
}
 
fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
