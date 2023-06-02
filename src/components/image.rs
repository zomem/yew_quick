use yew::{function_component, html, use_memo, Callback, Html, MouseEvent, NodeRef, Properties};

use regex::Regex;
use stylist::css;

use crate::{Cursor, PointerEvents, Position};

#[derive(Clone, PartialEq)]
struct ImageCss {
    width: String,
    height: String,
    padding: String,
    margin: String,
    border_radius: String,
    cursor: String,

    position: String,
    top: String,
    right: String,
    bottom: String,
    left: String,
    z_index: String,
    opacity: String,

    pointer_events: String,

    duration: String,
    hover_opacity: String,
    hover_padding: String,
    hover_margin: String,
    hover_radius: String,
    hover_width: String,
    hover_height: String,
}

#[derive(Clone, PartialEq)]
struct ImageCssProps {
    size: String,
    padding: String,
    margin: String,

    radius: String,

    cursor: Cursor,

    position: Position,
    top: String,
    right: String,
    bottom: String,
    left: String,
    z_index: String,
    opacity: String,

    pointer_events: PointerEvents,

    duration: String,
    h_opacity: String,
    h_padding: String,
    h_margin: String,
    h_radius: String,
    h_size: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    pub src: String,
    #[prop_or(String::from("auto auto"))]
    pub size: String,
    #[prop_or(String::from("0"))]
    pub padding: String,
    #[prop_or(String::from("0"))]
    pub margin: String,
    #[prop_or(String::from("0"))]
    pub radius: String,

    #[prop_or(Cursor::Unset)]
    pub cursor: Cursor,

    #[prop_or(Position::Static)]
    pub position: Position,
    #[prop_or(String::from("auto"))]
    pub top: String,
    #[prop_or(String::from("auto"))]
    pub right: String,
    #[prop_or(String::from("auto"))]
    pub bottom: String,
    #[prop_or(String::from("auto"))]
    pub left: String,
    #[prop_or(String::from("auto"))]
    pub z_index: String,
    #[prop_or(String::from("inherit"))]
    pub opacity: String,

    #[prop_or(String::from("0"))]
    pub duration: String,
    #[prop_or(String::from(""))]
    pub h_opacity: String,
    #[prop_or(String::from(""))]
    pub h_padding: String,
    #[prop_or(String::from(""))]
    pub h_margin: String,
    #[prop_or(String::from(""))]
    pub h_radius: String,
    #[prop_or(String::from(""))]
    pub h_size: String,

    #[prop_or(PointerEvents::Auto)]
    pointer_events: PointerEvents,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub node: NodeRef,
}

/// ### 使用示例
///```
/// size: String,
/// padding: String,
/// margin: String,
/// radius: String,
/// cursor: Cursor,
/// position: Position,
/// top: String,
/// right: String,
/// bottom: String,
/// left: String,
/// z_index: String,
/// opacity: String,
/// pointer_events: PointerEvents,
/// duration: String,
/// h_size: String,
/// h_opacity: String,  //hover 样式 "0.7"
/// h_padding: String,  //hover 样式 "0 0 12 12"
/// h_margin: String,  //hover 样式 "0 0 12 12"
/// h_radius: String,    //hover 样式 "12"
///```
///
#[function_component]
pub fn Image(props: &ImageProps) -> Html {
    let box_css_p: ImageCssProps = ImageCssProps {
        size: props.size.clone(),
        padding: props.padding.clone(),
        margin: props.margin.clone(),
        radius: props.radius.clone(),
        cursor: props.cursor.clone(),

        position: props.position.clone(),
        top: props.top.clone(),
        right: props.right.clone(),
        bottom: props.bottom.clone(),
        left: props.left.clone(),
        z_index: props.z_index.clone(),
        opacity: props.opacity.clone(),

        pointer_events: props.pointer_events.clone(),

        duration: props.duration.clone(),
        h_opacity: props.h_opacity.clone(),
        h_padding: props.h_padding.clone(),
        h_margin: props.h_margin.clone(),
        h_radius: props.h_radius.clone(),
        h_size: props.h_size.clone(),
    };

    let box_css = use_memo(
        |box_css_p| {
            let w_rex = Regex::new("[a-zA-Z]+").unwrap();
            let op_rex = Regex::new(r"([\+\-\*/])+").unwrap();
            let temp_size = box_css_p.size.split(" ").collect::<Vec<&str>>();
            let temp_h_size = if box_css_p.h_size == String::default() {
                temp_size.clone()
            } else {
                box_css_p.h_size.split(" ").collect::<Vec<&str>>()
            };

            let temp_width = temp_size[0];
            let temp_height = if temp_size.len() == 1 {
                temp_size[0]
            } else {
                temp_size[1]
            };

            let temp_h_width = temp_h_size[0];
            let temp_h_height = if temp_h_size.len() == 1 {
                temp_h_size[0]
            } else {
                temp_h_size[1]
            };

            let temp_width_op = op_rex.replace_all(temp_width, " $1 ").to_string();
            let tmep_width_f = if temp_width.contains("%") {
                format!("{}", temp_width)
            } else {
                format!("{}{}", temp_width, "px")
            };
            let temp_height_op = op_rex.replace_all(temp_height, " $1 ").to_string();
            let temp_height_f = if temp_height.contains("%") {
                format!("{}", temp_height)
            } else {
                format!("{}{}", temp_height, "px")
            };

            let temp_h_width_op = op_rex.replace_all(temp_h_width, " $1 ").to_string();
            let tmep_h_width_f = if temp_h_width.contains("%") {
                format!("{}", temp_h_width)
            } else {
                format!("{}{}", temp_h_width, "px")
            };
            let temp_h_height_op = op_rex.replace_all(temp_h_height, " $1 ").to_string();
            let temp_h_height_f = if temp_h_height.contains("%") {
                format!("{}", temp_h_height)
            } else {
                format!("{}{}", temp_h_height, "px")
            };

            let temp_padding = box_css_p.padding.as_str();
            let temp_margin = box_css_p.margin.as_str();
            let temp_radius = box_css_p.radius.as_str();

            let temp_padding_c = temp_padding
                .split(" ")
                .map(|x| {
                    if x.contains("%") {
                        x.to_string()
                    } else {
                        x.to_string() + "px"
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
            let padding_value = if w_rex.is_match(temp_padding) {
                temp_padding.to_owned()
            } else {
                temp_padding_c
            };
            let temp_margin_c = temp_margin
                .split(" ")
                .map(|x| {
                    if x.contains("%") {
                        x.to_string()
                    } else {
                        x.to_string() + "px"
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
            let margin_value = if w_rex.is_match(temp_margin) {
                temp_margin.to_owned()
            } else {
                temp_margin_c
            };
            let temp_radius_c = temp_radius
                .split(" ")
                .map(|x| {
                    if x.contains("%") || x.contains("/") {
                        x.to_string()
                    } else {
                        x.to_string() + "px"
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");

            let radius_value = if w_rex.is_match(temp_radius) {
                temp_radius.to_owned()
            } else {
                temp_radius_c
            };
            ImageCss {
                width: if temp_width == "auto" {
                    "auto".to_owned()
                } else if w_rex.is_match(temp_width) {
                    temp_width_op
                } else {
                    tmep_width_f
                },
                height: if temp_height == "auto" {
                    "auto".to_owned()
                } else if w_rex.is_match(temp_height) {
                    temp_height_op
                } else {
                    temp_height_f
                },
                padding: padding_value.clone(),
                margin: margin_value.clone(),
                border_radius: radius_value.clone(),

                cursor: box_css_p.cursor.get_name(),
                position: box_css_p.position.get_name(),
                top: box_css_p.top.clone(),
                right: box_css_p.right.clone(),
                bottom: box_css_p.bottom.clone(),
                left: box_css_p.left.clone(),
                z_index: box_css_p.z_index.clone(),
                opacity: box_css_p.opacity.clone(),

                pointer_events: box_css_p.pointer_events.get_name(),

                duration: box_css_p.duration.clone(),
                hover_opacity: if box_css_p.h_opacity == String::default() {
                    box_css_p.opacity.clone()
                } else {
                    box_css_p.h_opacity.clone()
                },
                hover_padding: if box_css_p.h_padding == String::default() {
                    padding_value
                } else {
                    let temp = box_css_p.h_padding.as_str();
                    if w_rex.is_match(temp) {
                        temp.to_string()
                    } else {
                        temp.split(" ")
                            .map(|x| {
                                if x.contains("%") {
                                    x.to_string()
                                } else {
                                    x.to_string() + "px"
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(" ")
                    }
                },
                hover_margin: if box_css_p.h_margin == String::default() {
                    margin_value
                } else {
                    let temp = box_css_p.h_margin.as_str();
                    if w_rex.is_match(temp) {
                        temp.to_string()
                    } else {
                        temp.split(" ")
                            .map(|x| {
                                if x.contains("%") {
                                    x.to_string()
                                } else {
                                    x.to_string() + "px"
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(" ")
                    }
                },
                hover_radius: if box_css_p.h_radius == String::default() {
                    radius_value
                } else {
                    let temp = box_css_p.h_radius.as_str();
                    if w_rex.is_match(temp) {
                        temp.to_string()
                    } else {
                        temp.split(" ")
                            .map(|x| {
                                if x.contains("%") || x.contains("/") {
                                    x.to_string()
                                } else {
                                    x.to_string() + "px"
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(" ")
                    }
                },
                hover_width: if temp_h_width == "auto" {
                    "auto".to_owned()
                } else if w_rex.is_match(temp_h_width) {
                    temp_h_width_op
                } else {
                    tmep_h_width_f
                },
                hover_height: if temp_h_height == "auto" {
                    "auto".to_owned()
                } else if w_rex.is_match(temp_h_height) {
                    temp_h_height_op
                } else {
                    temp_h_height_f
                },
            }
        },
        box_css_p,
    );

    let class = css!(
        r#"
            width: ${width};
            height: ${height};
            padding: ${padding};
            margin: ${margin};
            border-radius: ${radius};


            cursor: ${cursor};


            position: ${position};
            left: ${left};
            top: ${top};
            right: ${right};
            bottom: ${bottom};
            z-index: ${z_index};
            opacity: ${opacity};

            pointer-events: ${pointer_events};

            transition: all ${duration}s;

            &:hover {
                width: ${hover_width};
                height: ${hover_height};
                padding: ${hover_padding};
                margin: ${hover_margin};
                border-radius: ${hover_radius};
                opacity: ${hover_opacity};
            }
        "#,
        width = box_css.width,
        height = box_css.height,
        padding = box_css.padding,
        margin = box_css.margin,
        radius = box_css.border_radius,
        cursor = box_css.cursor,
        position = box_css.position,
        top = box_css.top,
        right = box_css.right,
        bottom = box_css.bottom,
        left = box_css.left,
        z_index = box_css.z_index,
        opacity = box_css.opacity,
        duration = box_css.duration,
        hover_opacity = box_css.hover_opacity,
        hover_padding = box_css.hover_padding,
        hover_margin = box_css.hover_margin,
        hover_radius = box_css.hover_radius,
        pointer_events = box_css.pointer_events,
        hover_width = box_css.hover_width,
        hover_height = box_css.hover_height,
    );

    let do_click = {
        let click = props.onclick.clone();
        Callback::from(move |e: MouseEvent| click.emit(e))
    };

    html! {
        <img {class} onclick={do_click} src={props.src.clone()} ref={props.node.clone()}/>
    }
}
