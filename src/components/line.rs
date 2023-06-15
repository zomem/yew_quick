use yew::{function_component, html, use_memo, Html, Properties};

use stylist::css;

use crate::prelude::{Cursor, SafeType, TimingFn};
use crate::utils::{add_op_space, is_have_unit};

#[derive(Clone, PartialEq)]
struct LineCss {
    width: String,
    height: String,
    margin: String,
    padding_top: String,
    padding_bottom: String,
    border_radius: String,
    background_color: String,
    background_image: String,
    cursor: String,

    opacity: String,

    duration: String,
    timing_fn: String,
    hover_opacity: String,
    hover_margin: String,
    hover_radius: String,
    hover_bg_color: String,
    hover_width: String,
    hover_height: String,

    flex_shrink: String,

    dark_bg_color: String,
}

#[derive(Clone, PartialEq)]
struct LineCssProps {
    size: String,
    radius: String,
    margin: String,

    bg_color: String,
    bg_image: String,

    cursor: Cursor,

    opacity: String,

    duration: String,
    timing_fn: TimingFn,
    h_opacity: String,
    h_margin: String,
    h_radius: String,
    h_bg_color: String,
    h_size: String,

    flex_shrink: String,

    d_bg_color: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct LineProps {
    #[prop_or(String::from("auto auto"))]
    pub size: String,
    #[prop_or(SafeType::None)]
    pub safe: SafeType,
    #[prop_or(String::from("0"))]
    pub radius: String,
    #[prop_or(String::from("0"))]
    pub margin: String,

    #[prop_or(String::from("transparent"))]
    pub bg_color: String,
    #[prop_or(String::from("0"))]
    pub bg_image: String,

    #[prop_or(Cursor::Unset)]
    pub cursor: Cursor,
    #[prop_or(String::from("1"))]
    pub flex_shrink: String,

    #[prop_or(String::from("inherit"))]
    pub opacity: String,

    #[prop_or(String::from("0"))]
    pub duration: String,
    #[prop_or(TimingFn::Ease)]
    pub timing_fn: TimingFn,
    #[prop_or(String::from(""))]
    pub h_opacity: String,
    #[prop_or(String::from(""))]
    pub h_margin: String,
    #[prop_or(String::from(""))]
    pub h_radius: String,
    #[prop_or(String::from(""))]
    pub h_bg_color: String,
    #[prop_or(String::from(""))]
    pub h_size: String,

    #[prop_or(String::from(""))]
    pub d_bg_color: String,
}

/// ### 使用示例
///```
/// size: String,
/// radius: String,
/// margin: String, // "1 2 2 1"
/// bg_color: String,
/// bg_image: String,
/// cursor: Cursor,
/// flex_shrink: String,
/// opacity: String,
/// duration: String,
/// timing_fn: TimingFn, // transition 的动画方式
/// h_opacity: String,  //hover 样式 "0.7"
/// h_margin: String,  //hover 样式 "0 0 12 12"
/// h_radius: String,    //hover 样式 "12"
/// h_bg_color: String,
/// h_size: String,
/// d_bg_color: String,  // dark 模式
///```
///
#[function_component]
pub fn Line(props: &LineProps) -> Html {
    let box_css_p: LineCssProps = LineCssProps {
        size: props.size.clone(),

        radius: props.radius.clone(),
        margin: props.margin.clone(),

        bg_color: props.bg_color.clone(),
        bg_image: props.bg_image.clone(),

        cursor: props.cursor.clone(),
        opacity: props.opacity.clone(),
        flex_shrink: props.flex_shrink.clone(),

        duration: props.duration.clone(),
        timing_fn: props.timing_fn.clone(),
        h_opacity: props.h_opacity.clone(),
        h_margin: props.h_margin.clone(),
        h_radius: props.h_radius.clone(),
        h_bg_color: props.h_bg_color.clone(),
        h_size: props.h_size.clone(),

        d_bg_color: props.d_bg_color.clone(),
    };

    let box_css = use_memo(
        |box_css_p| {
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

            let temp_width_op = add_op_space(temp_width);
            let tmep_width_f = if temp_width.contains("%") {
                format!("{}", temp_width)
            } else {
                format!("{}{}", temp_width, "px")
            };
            let temp_height_op = add_op_space(temp_height);
            let temp_height_f = if temp_height.contains("%") {
                format!("{}", temp_height)
            } else {
                format!("{}{}", temp_height, "px")
            };

            let temp_h_width_op = add_op_space(temp_h_width);
            let tmep_h_width_f = if temp_h_width.contains("%") {
                format!("{}", temp_h_width)
            } else {
                format!("{}{}", temp_h_width, "px")
            };
            let temp_h_height_op = add_op_space(temp_h_height);
            let temp_h_height_f = if temp_h_height.contains("%") {
                format!("{}", temp_h_height)
            } else {
                format!("{}{}", temp_h_height, "px")
            };

            let temp_radius = box_css_p.radius.as_str();
            let temp_margin = box_css_p.margin.as_str();

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
            let radius_value = if is_have_unit(temp_radius) {
                temp_radius.to_owned()
            } else {
                temp_radius_c
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
            let margin_value = if is_have_unit(temp_margin) {
                temp_margin.to_owned()
            } else {
                temp_margin_c
            };

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
                } else if is_have_unit(temp_width) {
                    temp_width_op
                } else {
                    tmep_width_f
                },
                height: if temp_height == "auto" {
                    "auto".to_owned()
                } else if is_have_unit(temp_height) {
                    temp_height_op
                } else {
                    temp_height_f
                },
                opacity: box_css_p.opacity.clone(),
                margin: margin_value.clone(),
                padding_top: p_top,
                padding_bottom: p_bottom,
                border_radius: radius_value.clone(),
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

                duration: box_css_p.duration.clone(),
                timing_fn: box_css_p.timing_fn.get_name(),
                hover_opacity: if box_css_p.h_opacity == String::default() {
                    box_css_p.opacity.clone()
                } else {
                    box_css_p.h_opacity.clone()
                },
                hover_margin: if box_css_p.h_margin == String::default() {
                    margin_value
                } else {
                    let temp = box_css_p.h_margin.as_str();
                    if is_have_unit(temp) {
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
                    if is_have_unit(temp) {
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
                hover_bg_color: if box_css_p.h_bg_color == String::default() {
                    "none".to_string()
                } else {
                    box_css_p.h_bg_color.clone()
                },
                hover_width: if temp_h_width == "auto" {
                    "auto".to_owned()
                } else if is_have_unit(temp_h_width) {
                    temp_h_width_op
                } else {
                    tmep_h_width_f
                },
                hover_height: if temp_h_height == "auto" {
                    "auto".to_owned()
                } else if is_have_unit(temp_h_height) {
                    temp_h_height_op
                } else {
                    temp_h_height_f
                },
                dark_bg_color: if box_css_p.d_bg_color == String::default() {
                    box_css_p.bg_color.clone()
                } else {
                    box_css_p.d_bg_color.clone()
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
            margin: ${margin};
            border-radius: ${radius};
            background-color: ${bg_color};
            background-image: ${bg_img};
            cursor: ${cursor};
            flex-shrink: ${flex_shrink};

            transition: all ${duration}s ${timing_fn};

            &:hover {
                background-color: ${hover_bg_color};
                width: ${hover_width};
                height: ${hover_height};
                margin: ${hover_margin};
                border-radius: ${hover_radius};
                opacity: ${hover_opacity};
            }

            @media (prefers-color-scheme: dark) {
              & {
                background-color: ${dark_bg_color};
              }
            }
        "#,
        padding_bottom = box_css.padding_bottom,
        padding_top = box_css.padding_top,
        margin = box_css.margin,
        width = box_css.width,
        height = box_css.height,
        radius = box_css.border_radius,
        bg_color = box_css.background_color,
        bg_img = box_css.background_image,
        cursor = box_css.cursor,
        flex_shrink = box_css.flex_shrink,
        duration = box_css.duration,
        timing_fn = box_css.timing_fn,
        hover_opacity = box_css.hover_opacity,
        hover_margin = box_css.hover_margin,
        hover_radius = box_css.hover_radius,
        hover_bg_color = box_css.hover_bg_color,
        hover_width = box_css.hover_width,
        hover_height = box_css.hover_height,
        dark_bg_color = box_css.dark_bg_color,
    );

    html! {
        <div {class} />
    }
}
