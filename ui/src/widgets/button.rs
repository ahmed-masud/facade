use crate::widgets::{View, Widget, WidgetModel};
use yew::html;

pub type ButtonWidget = WidgetModel<Model>;

pub struct Model {}

impl Widget for Model {
    type Message = ();
    type Properties = ();

    fn produce(_: &Self::Properties) -> Self {
        Self {}
    }

    fn main_view(&self) -> View<Self> {
        html! {
            <button>{ "Button!" }</button>
        }
    }
}
