use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct BlockProps {
    pub children: Children,
}

#[function_component]
pub fn Block(props: &BlockProps) -> Html {
    html! {
        <>
        { for props.children.iter() }
        </>
    }
}
