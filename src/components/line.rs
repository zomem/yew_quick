use yew::{function_component, html, use_memo, Html, Properties};

use regex::Regex;
use stylist::css;

use crate::{Cursor, SafeType};

#[derive(Clone, PartialEq)]
struct LineCss {
    width: String,
    height: String,
    padding_top: String,
    padding_bottom: String,
    border_radius: String,
    background_color: String,
    background_image: String,
    cursor: String,
    d: String,
    hover_bg_color: String,
    hover_width: String,
    hover_height: String,
    flex_shrink: String,
}

#[derive(Clone, PartialEq)]
struct LineCssProps {
    size: String,
    radius: String,

    bg_color: String,
    bg_image: String,

    cursor: Cursor,
    d: String,
    h_bg_color: String,
    h_size: String,
    flex_shrink: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LineProps {
    #[prop_or(String::from("auto auto"))]
    pub size: String,
    #[prop_or(SafeType::None)]
    pub safe: SafeType,
    #[prop_or(String::from("0"))]
    pub radius: String,

    #[prop_or(String::from("transparent"))]
    pub bg_color: String,
    #[prop_or(String::from("0"))]
    pub bg_image: String,

    #[prop_or(Cursor::Unset)]
    pub cursor: Cursor,
    #[prop_or(String::from("1"))]
    pub flex_shrink: String,

    #[prop_or(String::from("0"))]
    pub d: String,
    #[prop_or(String::from(""))]
    pub h_bg_color: String,
    #[prop_or(String::from(""))]
    pub h_size: String,
}

/// ### 使用示例
///```
/// size: String,
/// radius: String,

/// bg_color: String,
/// bg_image: String,

/// cursor: Cursor,
/// d: String,
/// h_bg_color: String,
/// h_size: String,
/// flex_shrink: String,
///```
///
#[function_component]
pub fn Line(props: &LineProps) -> Html {
    let box_css_p: LineCssProps = LineCssProps {
        size: props.size.clone(),

        radius: props.radius.clone(),

        bg_color: props.bg_color.clone(),
        bg_image: props.bg_image.clone(),

        cursor: props.cursor.clone(),

        flex_shrink: props.flex_shrink.clone(),

        d: props.d.clone(),
        h_bg_color: props.h_bg_color.clone(),
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

            let temp_radius = box_css_p.radius.as_str();

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

            let p_top = match props.safe {
                SafeType::None => "0".to_owned(),
                SafeType::Top => "env(safe-area-inset-top)".to_owned(),
                SafeType::Bottom => "0".to_owned(),
            };
            let p_bottom = match props.safe {
                SafeType::None => "0".to_owned(),
                SafeType::Top => "0".to_owned(),
                SafeType::Bottom => "env(safe-area-inset-bottom)".to_owned(),
            };

            LineCss {
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
                padding_top: p_top,
                padding_bottom: p_bottom,
                border_radius: if w_rex.is_match(temp_radius) {
                    temp_radius.to_owned()
                } else {
                    temp_radius_c
                },
                background_color: box_css_p.bg_color.clone(),
                background_image: if box_css_p.bg_image == "0" {
                    "none".to_owned()
                } else {
                    if box_css_p.bg_image.contains("linear-gradient") {
                        box_css_p.bg_image.clone()
                    } else {
                        "url(".to_string() + box_css_p.bg_image.clone().as_str() + ")"
                    }
                },
                cursor: box_css_p.cursor.get_name(),
                flex_shrink: box_css_p.flex_shrink.clone(),

                d: box_css_p.d.clone(),
                hover_bg_color: if box_css_p.h_bg_color == String::default() {
                    box_css_p.bg_color.clone()
                } else {
                    box_css_p.h_bg_color.clone()
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
            padding-top: ${padding_top};
            padding-bottom: ${padding_bottom};
            border-radius: ${radius};
            background-color: ${bg_color};
            background-image: ${bg_img};
            cursor: ${cursor};
            flex-shrink: ${flex_shrink};
            transition: all ${d}s;

            &:hover {
                background-color: ${hover_bg_color};
                width: ${hover_width};
                height: ${hover_height};
            }
        "#,
        padding_bottom = box_css.padding_bottom,
        padding_top = box_css.padding_top,
        width = box_css.width,
        height = box_css.height,
        radius = box_css.border_radius,
        bg_color = box_css.background_color,
        bg_img = box_css.background_image,
        cursor = box_css.cursor,
        flex_shrink = box_css.flex_shrink,
        d = box_css.d,
        hover_bg_color = box_css.hover_bg_color,
        hover_width = box_css.hover_width,
        hover_height = box_css.hover_height,
    );

    html! {
        <div {class} />
    }
}
