use yew::prelude::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,
}

pub fn handle_props_class(props: &Props) -> &'static str {
    props.class.unwrap_or_default()
}
