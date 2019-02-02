#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {
    state: State,
}

struct State {
    tasks: Vec<Task>,
}

struct Task {
    name: String,
    assignee: String,
    mandays: u32,
    status: u32,
}

enum Msg {
    IncreaseStatus(usize),
    DecreaseStatus(usize),
}

impl State {
    fn increase_status(&mut self, idx: usize) {
        self.tasks.get_mut(idx).filter(|e| e.status < 3).map(|e| e.status = e.status + 1);
    }
    fn decrease_status(&mut self, idx: usize) {
        self.tasks.get_mut(idx).filter(|e| e.status > 1).map(|e| e.status = e.status - 1);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            state: State {
                tasks: vec! [
                    Task { name: "Task 1".to_string(), assignee: "🐱".to_string(), mandays: 3, status: 1 },
                    Task { name: "Task 2".to_string(), assignee: "🐶".to_string(), mandays: 2, status: 1 },
                    Task { name: "Task 3".to_string(), assignee: "🐱".to_string(), mandays: 1, status: 2 },
                    Task { name: "Task 4".to_string(), assignee: "🐹".to_string(), mandays: 3, status: 3 },
                ]
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncreaseStatus(idx) => {
                self.state.increase_status(idx);
            }
            Msg::DecreaseStatus(idx) => {
                self.state.decrease_status(idx);
            }
        }
        true
    }
}

fn view_column(status: u32, status_text: &str, tasks: &Vec<Task>) -> Html<Model> {
    html! {
        <div class=format!("column status-{}", status),>
            <div class="tags has-addons",>
                <span class="tag",>{ status_text }</span>
                <span class="tag is-dark",>{ tasks.iter().filter(|e| e.status == status).count() }</span>
            </div>
            { for tasks.iter().enumerate().filter(|e| e.1.status == status).map(view_task) }
        </div>
    }
}

fn view_task((idx, task): (usize, &Task)) -> Html<Model> {
    html! {
        <div class="card",>
            <div class="card-content",>
                { &task.name }
            </div>
            <footer class="card-footer",>
                <div class="card-footer-item",>
                    { &task.assignee }
                </div>
                <div class="card-footer-item",>
                    { format!("{} 人日", &task.mandays) }
                </div>
            </footer>
            <footer class="card-footer",>
              <a class="card-footer-item", onclick=|_| Msg::DecreaseStatus(idx),>{ "◀︎" }</a>
              <a class="card-footer-item", onclick=|_| Msg::IncreaseStatus(idx),>{ "▶" }</a>
            </footer>
          </div>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section class="section", id="board",>
                <div class="container",>
                    <div class="columns",>
                        { view_column(1, "未対応", &self.state.tasks) }
                        { view_column(2, "処理中", &self.state.tasks) }
                        { view_column(3, "完了", &self.state.tasks) }
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
