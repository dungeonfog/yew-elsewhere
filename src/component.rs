use super::ElsewhereService;
use yew::prelude::*;

pub struct Elsewhere {
    props: Props,
    link: ComponentLink<Self>,
    current_body: Html,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub name: String,
}

pub enum Msg {
    UpdateBody(Html),
}

impl Component for Elsewhere {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::UpdateBody);
        let current_body = ElsewhereService::get()
            .register_component(&props.name, callback)
            .unwrap_or_else(|| html! {});
        Self {
            props,
            link,
            current_body,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.name != self.props.name {
            ElsewhereService::get().unregister_component(&self.props.name);
            let callback = self.link.callback(Msg::UpdateBody);
            let new_body = ElsewhereService::get()
                .register_component(&props.name, callback)
                .unwrap_or_else(|| html! {});
            self.props = props;
            self.current_body = new_body;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateBody(new_body) => self.current_body = new_body,
        }
        true
    }

    fn view(&self) -> Html {
        self.current_body.clone()
    }

    fn destroy(&mut self) {
        ElsewhereService::get().unregister_component(&self.props.name);
    }
}
