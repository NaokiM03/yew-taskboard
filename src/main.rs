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

fn view_column(status: u32, status_text: &str) -> Html<Model> {
    html! {
        <div class=format!("column status-{}", status),>
            <div class="tags has-addons",>
                <span class="tag",>{ status_text }</span>
                <span class="tag is-dark",>{ 0 }</span>
            </div>
        </div>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section class="section", id="board",>
                <div class="container",>
                    <div class="columns",>
                        { view_column(1, "未対応") }
                        { view_column(2, "処理中") }
                        { view_column(3, "完了") }
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
