use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::html;

pub type PanelWidget = WidgetModel<Model>;

pub struct Model {
    panel: Option<protocol::Panel>,
}

impl Default for Model {
    fn default() -> Self {
        Self { panel: None }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub panel: Option<protocol::Panel>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.panel = props.panel.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        if let Some(panel) = self.panel.as_ref() {
            if let Some(ref title) = panel.title {
                html! {
                    <div class="panel",>
                        <div class="panel-header",>
                            <p class="panel-header-title",>{ title }</p>
                        </div>
                        <div class="panel-content",>
                            <widgets::Layout: layout=Some(panel.body.clone()), />
                        </div>
                    </div>
                }
            } else {
                html! {
                    <div class="panel",>
                        <div class="panel-content",>
                            <widgets::Layout: layout=Some(panel.body.clone()), />
                        </div>
                    </div>
                }
            }
        } else {
            html! {
                <widgets::Spinner: />
            }
        }
    }
}