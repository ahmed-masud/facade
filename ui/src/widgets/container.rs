use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::Container;
use yew::{html, Properties};

pub type ContainerWidget = WidgetModel<Model>;

pub struct Model {
    container: Container,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub container: Container,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            container: props.container.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.container = props.container.clone();
        None
    }

    fn main_view(&self) -> View<Self> {
        match self.container {
            Container::Blank => {
                html! {
                    <p>{ "Blank" }</p>
                }
            }
            Container::Tabs(_) => {
                html! {
                    <p>{ "Tabs" }</p>
                }
            }
            Container::Panel(ref panel) => {
                html! {
                    <div class="container",>
                        <widgets::Panel: panel=Some(panel.clone()), />
                    </div>
                }
            }
        }
    }
}
