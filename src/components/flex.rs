use yew::{
    function_component, html, use_memo, Callback, Children, Html, MouseEvent, NodeRef, Properties,
};

use stylist::css;

use crate::prelude::{
    BorderStyle, BoxSizing, Cursor, FlexWay, FontStyle, FontWeight, ImageMode, Overflow, Position,
    TextAlign, TimingFn, WhiteSpace, WordBreak,
};
use crate::utils::{add_op_space, is_have_unit};

#[derive(Clone, PartialEq)]
struct FlexCss {
    width: String,
    height: String,
    padding: String,
    margin: String,
    box_sizing: String,
    border_radius: String,
    border_width: String,
    border_color: String,
    border_style: String,
    background_color: String,
    overflow: String,
    background_image: String,
    background_size: String,
    backdrop_filter: String,
    cursor: String,
    white_space: String,
    min_width: String,
    min_height: String,
    max_width: String,
    max_height: String,

    box_shadow: String,
    position: String,
    top: String,
    right: String,
    bottom: String,
    left: String,
    z_index: String,
    opacity: String,

    font_size: String,
    color: String,
    font_style: String,
    font_weight: String,
    letter_spacing: String,
    line_height: String,
    text_decoration: String,
    text_align: String,
    word_break: String,
    flex_shrink: String,

    flex_direction: String,
    justify_content: String,
    align_items: String,

    duration: String,
    timing_fn: String,
    hover_opacity: String,
    hover_padding: String,
    hover_margin: String,
    hover_radius: String,
    hover_border_width: String,
    hover_border_color: String,
    hover_bg_color: String,
    hover_color: String,
    hover_shadow: String,
    hover_width: String,
    hover_height: String,

    dark_bg_color: String,
    dark_shadow: String,
    dark_border_color: String,
    dark_color: String,
}

#[derive(Clone, PartialEq)]
struct FlexCssProps {
    flex: FlexWay,
    size: String,
    padding: String,
    margin: String,
    box_sizing: BoxSizing,
    radius: String,
    border_width: String,
    border_color: String,
    border_style: BorderStyle,
    bg_color: String,
    bg_image: String,
    image_mode: ImageMode,
    overflow: Overflow,
    backdrop: String,
    cursor: Cursor,
    white_space: WhiteSpace,
    min_size: String,
    max_size: String,

    shadow: String,
    position: Position,
    top: String,
    right: String,
    bottom: String,
    left: String,
    z_index: String,
    opacity: String,

    font_size: String,
    color: String,
    font_style: FontStyle,
    font_weight: FontWeight,
    letter_spacing: String,
    line_height: String,
    text_decoration: String,
    text_align: TextAlign,
    word_break: WordBreak,
    flex_shrink: String,

    duration: String,
    timing_fn: TimingFn,
    h_opacity: String,
    h_padding: String,
    h_margin: String,
    h_radius: String,
    h_border_width: String,
    h_border_color: String,
    h_bg_color: String,
    h_color: String,
    h_shadow: String,
    h_size: String,

    d_bg_color: String,
    d_shadow: String,
    d_border_color: String,
    d_color: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct FlexProps {
    pub flex: FlexWay,
    #[prop_or(String::from("auto auto"))]
    pub size: String,
    #[prop_or(String::from("0"))]
    pub padding: String,
    #[prop_or(String::from("0"))]
    pub margin: String,
    #[prop_or(BoxSizing::BorderBox)]
    pub box_sizing: BoxSizing,
    #[prop_or(String::from("0"))]
    pub radius: String,
    #[prop_or(String::from("0"))]
    pub border_width: String,
    #[prop_or(String::from("transparent"))]
    pub border_color: String,
    #[prop_or(BorderStyle::No)]
    pub border_style: BorderStyle,
    #[prop_or(String::from("transparent"))]
    pub bg_color: String,
    #[prop_or(String::from("0"))]
    pub bg_image: String,
    #[prop_or(ImageMode::Auto)]
    pub image_mode: ImageMode,
    #[prop_or(Overflow::Visible)]
    pub overflow: Overflow,
    #[prop_or(String::from("none"))]
    pub backdrop: String,
    #[prop_or(Cursor::Unset)]
    pub cursor: Cursor,
    #[prop_or(WhiteSpace::Normal)]
    pub white_space: WhiteSpace,
    #[prop_or(String::from("auto auto"))]
    pub min_size: String,
    #[prop_or(String::from("auto auto"))]
    pub max_size: String,

    #[prop_or(String::from("none"))]
    pub shadow: String,
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

    #[prop_or(String::from("medium"))]
    pub font_size: String,
    #[prop_or(String::from("#181818"))]
    pub color: String,
    #[prop_or(FontStyle::Normal)]
    pub font_style: FontStyle,
    #[prop_or(FontWeight::Normal)]
    pub font_weight: FontWeight,
    #[prop_or(String::from("normal"))]
    pub letter_spacing: String,
    #[prop_or(String::from("normal"))]
    pub line_height: String,
    #[prop_or(String::from("none"))]
    pub text_decoration: String,
    #[prop_or(TextAlign::Left)]
    pub text_align: TextAlign,
    #[prop_or(WordBreak::Normal)]
    pub word_break: WordBreak,
    #[prop_or(String::from("1"))]
    pub flex_shrink: String,

    #[prop_or(String::from("0"))]
    pub duration: String,
    #[prop_or(TimingFn::Ease)]
    pub timing_fn: TimingFn,
    #[prop_or(String::from(""))]
    pub h_opacity: String,
    #[prop_or(String::from(""))]
    pub h_padding: String,
    #[prop_or(String::from(""))]
    pub h_margin: String,
    #[prop_or(String::from(""))]
    pub h_radius: String,
    #[prop_or(String::from(""))]
    pub h_border_width: String,
    #[prop_or(String::from(""))]
    pub h_border_color: String,
    #[prop_or(String::from(""))]
    pub h_bg_color: String,
    #[prop_or(String::from(""))]
    pub h_color: String,
    #[prop_or(String::from(""))]
    pub h_shadow: String,
    #[prop_or(String::from(""))]
    pub h_size: String,

    #[prop_or(String::from(""))]
    pub d_bg_color: String,
    #[prop_or(String::from(""))]
    pub d_shadow: String,
    #[prop_or(String::from(""))]
    pub d_border_color: String,
    #[prop_or(String::from(""))]
    pub d_color: String,

    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub node: NodeRef,
}

/// ### 使用示例
/// ```
/// flex: FlexWay,
/// size: String,  // "22" "10 20" "auto 100%" "50% calc(50vh-100px)"
/// padding: String, // "1 2 2 1"
/// margin: String, // "1 2 2 1"
/// box_sizing: BoxSizing,
/// radius: String,   // "10" "10% 30% 50% 70%"
/// border_width: String, // "2" "10 2 6 0"
/// border_color: String, // "red" "#f22 #cca #663 #0aa"
/// border_style: BorderStyle,
/// bg_color: String,  // "#f22"
/// bg_image: String,  // "https://xxx.png" "linear-gradient(#e66465, #9198e5)"
/// image_mode: ImageMode,
/// overflow: Overflow,
/// backdrop: String,  // "blur(10px)"
/// cursor: Cursor,
/// white_space: WhiteSpace,
/// min_size: String,  // "100 auto"
/// max_size: String,  // "200 auto"
/// shadow: String, // "10px 5px 5px red"
/// position: Position,
/// top: String,
/// right: String,
/// bottom: String,
/// left: String,
/// z_index: String,
/// opacity: String,
/// font_size: String,
/// color: String,
/// font_style: FontStyle,
/// font_weight: FontWeight,
/// letter_spacing: String,
/// line_height: String,
/// text_decoration: String,
/// text_align: TextAlign,
/// word_break: WordBreak,
/// flex_shrink: String,
/// duration: String, // transition 时间 s
/// timing_fn: TimingFn, // transition 的动画方式
/// h_opacity: String,  //hover 样式 "0.7"
/// h_padding: String,  //hover 样式 "0 0 12 12"
/// h_margin: String,  //hover 样式 "0 0 12 12"
/// h_radius: String,    //hover 样式 "12"
/// h_border_width: String, //hover 样式 "12"
/// h_border_color: String, //hover 样式 "#ffa"
/// h_bg_color: String,  // hover 样式
/// h_color: String,  // hover 样式
/// h_shadow: String,  // hover 样式
/// h_size: String,  // hover 样式
/// d_bg_color: String,  // dark 模式
/// d_shadow: String, // dark 模式
/// d_border_color: String, // dark 模式
/// d_color: String, // dark 模式
/// ```
#[function_component]
pub fn Flex(props: &FlexProps) -> Html {
    let box_css_p: FlexCssProps = FlexCssProps {
        flex: props.flex.clone(),
        size: props.size.clone(),
        padding: props.padding.clone(),
        margin: props.margin.clone(),
        box_sizing: props.box_sizing.clone(),
        radius: props.radius.clone(),
        border_width: props.border_width.clone(),
        border_color: props.border_color.clone(),
        border_style: props.border_style.clone(),
        bg_color: props.bg_color.clone(),
        bg_image: props.bg_image.clone(),
        image_mode: props.image_mode.clone(),
        overflow: props.overflow.clone(),
        backdrop: props.backdrop.clone(),
        cursor: props.cursor.clone(),
        white_space: props.white_space.clone(),
        min_size: props.min_size.clone(),
        max_size: props.max_size.clone(),
        shadow: props.shadow.clone(),
        position: props.position.clone(),
        top: props.top.clone(),
        right: props.right.clone(),
        bottom: props.bottom.clone(),
        left: props.left.clone(),
        z_index: props.z_index.clone(),
        opacity: props.opacity.clone(),

        font_size: props.font_size.clone(),
        color: props.color.clone(),
        font_style: props.font_style.clone(),
        font_weight: props.font_weight.clone(),
        letter_spacing: props.letter_spacing.clone(),
        line_height: props.line_height.clone(),
        text_decoration: props.text_decoration.clone(),
        text_align: props.text_align.clone(),
        word_break: props.word_break.clone(),
        flex_shrink: props.flex_shrink.clone(),

        duration: props.duration.clone(),
        timing_fn: props.timing_fn.clone(),
        h_opacity: props.h_opacity.clone(),
        h_padding: props.h_padding.clone(),
        h_margin: props.h_margin.clone(),
        h_radius: props.h_radius.clone(),
        h_border_width: props.h_border_width.clone(),
        h_border_color: props.h_border_color.clone(),
        h_bg_color: props.h_bg_color.clone(),
        h_color: props.h_color.clone(),
        h_shadow: props.h_shadow.clone(),
        h_size: props.h_size.clone(),

        d_bg_color: props.d_bg_color.clone(),
        d_shadow: props.d_shadow.clone(),
        d_border_color: props.d_border_color.clone(),
        d_color: props.d_color.clone(),
    };

    let box_css = use_memo(
        |box_css_p| {
            let f_vec: Vec<String> = box_css_p
                .flex
                .get_name()
                .split("")
                .filter(|x| x != &String::default() && x != &"f")
                .map(|x| match x {
                    "r" => "row".to_string(),
                    "c" => "center".to_string(),
                    "b" => "space-between".to_string(),
                    "a" => "space-around".to_string(),
                    "e" => "flex-end".to_string(),
                    "s" => "flex-start".to_string(),
                    _ => "center".to_string(),
                })
                .collect();
            let temp_size = box_css_p.size.split(" ").collect::<Vec<&str>>();
            let temp_h_size = if box_css_p.h_size == String::default() {
                temp_size.clone()
            } else {
                box_css_p.h_size.split(" ").collect::<Vec<&str>>()
            };
            let temp_min_size = box_css_p.min_size.split(" ").collect::<Vec<&str>>();
            let temp_max_size = box_css_p.max_size.split(" ").collect::<Vec<&str>>();

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

            let temp_min_width = temp_min_size[0];
            let temp_min_height = if temp_min_size.len() == 1 {
                temp_min_size[0]
            } else {
                temp_min_size[1]
            };
            let temp_max_width = temp_max_size[0];
            let temp_max_height = if temp_max_size.len() == 1 {
                temp_max_size[0]
            } else {
                temp_max_size[1]
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

            let temp_min_width_op = add_op_space(temp_min_width);
            let tmep_min_width_f = if temp_min_width.contains("%") {
                format!("{}", temp_min_width)
            } else {
                format!("{}{}", temp_min_width, "px")
            };
            let temp_min_height_op = add_op_space(temp_min_height);
            let temp_min_height_f = if temp_min_height.contains("%") {
                format!("{}", temp_min_height)
            } else {
                format!("{}{}", temp_min_height, "px")
            };
            let temp_max_width_op = add_op_space(temp_max_width);
            let tmep_max_width_f = if temp_max_width.contains("%") {
                format!("{}", temp_max_width)
            } else {
                format!("{}{}", temp_max_width, "px")
            };
            let temp_max_height_op = add_op_space(temp_max_height);
            let temp_max_height_f = if temp_max_height.contains("%") {
                format!("{}", temp_max_height)
            } else {
                format!("{}{}", temp_max_height, "px")
            };

            let temp_padding = box_css_p.padding.as_str();
            let temp_margin = box_css_p.margin.as_str();
            let temp_radius = box_css_p.radius.as_str();
            let temp_border_width = box_css_p.border_width.as_str();

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
            let padding_value = if is_have_unit(temp_padding) {
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
            let margin_value = if is_have_unit(temp_margin) {
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
            let radius_value = if is_have_unit(temp_radius) {
                temp_radius.to_owned()
            } else {
                temp_radius_c
            };
            let temp_border_width_c = temp_border_width
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
            let border_width_value = if is_have_unit(temp_border_width) {
                temp_border_width.to_owned()
            } else {
                temp_border_width_c
            };
            FlexCss {
                flex_direction: if f_vec[0] == "center".to_owned() {
                    "column".to_string()
                } else {
                    f_vec[0].clone()
                },
                justify_content: f_vec[1].clone(),
                align_items: f_vec[2].clone(),
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
                padding: padding_value.clone(),
                margin: margin_value.clone(),
                box_sizing: box_css_p.box_sizing.get_name(),
                border_radius: radius_value.clone(),
                border_width: border_width_value.clone(),
                border_color: box_css_p.border_color.clone(),
                border_style: box_css_p.border_style.get_name(),
                background_color: box_css_p.bg_color.clone(),
                overflow: box_css_p.overflow.get_name(),
                background_image: if box_css_p.bg_image == "0" {
                    "none".to_owned()
                } else {
                    if box_css_p.bg_image.contains("linear-gradient") {
                        box_css_p.bg_image.clone()
                    } else {
                        "url(".to_string() + box_css_p.bg_image.clone().as_str() + ")"
                    }
                },
                background_size: box_css_p.image_mode.get_name(),
                backdrop_filter: box_css_p.backdrop.clone(),
                cursor: box_css_p.cursor.get_name(),
                white_space: box_css_p.white_space.get_name(),
                min_width: if temp_min_width == "auto" {
                    "auto".to_owned()
                } else if is_have_unit(temp_min_width) {
                    temp_min_width_op
                } else {
                    tmep_min_width_f
                },
                min_height: if temp_min_height == "auto" {
                    "auto".to_owned()
                } else if is_have_unit(temp_min_height) {
                    temp_min_height_op
                } else {
                    temp_min_height_f
                },
                max_width: if temp_max_width == "auto" {
                    "none".to_owned()
                } else if is_have_unit(temp_max_width) {
                    temp_max_width_op
                } else {
                    tmep_max_width_f
                },
                max_height: if temp_max_height == "auto" {
                    "none".to_owned()
                } else if is_have_unit(temp_max_height) {
                    temp_max_height_op
                } else {
                    temp_max_height_f
                },
                box_shadow: box_css_p.shadow.clone(),
                position: box_css_p.position.get_name(),
                top: if box_css_p.top.contains("%") {
                    box_css_p.top.clone()
                } else {
                    if box_css_p.top == String::from("auto") {
                        box_css_p.top.clone()
                    } else {
                        box_css_p.top.clone() + "px"
                    }
                },
                right: if box_css_p.right.contains("%") {
                    box_css_p.right.clone()
                } else {
                    if box_css_p.right == String::from("auto") {
                        box_css_p.right.clone()
                    } else {
                        box_css_p.right.clone() + "px"
                    }
                },
                bottom: if box_css_p.bottom.contains("%") {
                    box_css_p.bottom.clone()
                } else {
                    if box_css_p.bottom == String::from("auto") {
                        box_css_p.bottom.clone()
                    } else {
                        box_css_p.bottom.clone() + "px"
                    }
                },
                left: if box_css_p.left.contains("%") {
                    box_css_p.left.clone()
                } else {
                    if box_css_p.left == String::from("auto") {
                        box_css_p.left.clone()
                    } else {
                        box_css_p.left.clone() + "px"
                    }
                },
                z_index: box_css_p.z_index.clone(),
                opacity: box_css_p.opacity.clone(),

                font_size: if is_have_unit(&box_css_p.font_size) {
                    box_css_p.font_size.to_owned()
                } else {
                    box_css_p.font_size.clone() + "px"
                },
                color: box_css_p.color.clone(),
                font_style: box_css_p.font_style.get_name(),
                font_weight: box_css_p.font_weight.get_name(),
                letter_spacing: if is_have_unit(&box_css_p.letter_spacing) {
                    box_css_p.letter_spacing.to_owned()
                } else {
                    box_css_p.letter_spacing.clone() + "px"
                },
                line_height: if is_have_unit(&box_css_p.line_height) {
                    box_css_p.line_height.to_owned()
                } else {
                    box_css_p.line_height.clone() + "px"
                },
                text_decoration: box_css_p.text_decoration.clone(),
                text_align: box_css_p.text_align.get_name(),
                word_break: box_css_p.word_break.get_name(),
                flex_shrink: box_css_p.flex_shrink.clone(),

                duration: box_css_p.duration.clone(),
                timing_fn: box_css_p.timing_fn.get_name(),
                hover_opacity: if box_css_p.h_opacity == String::default() {
                    box_css_p.opacity.clone()
                } else {
                    box_css_p.h_opacity.clone()
                },
                hover_padding: if box_css_p.h_padding == String::default() {
                    padding_value
                } else {
                    let temp = box_css_p.h_padding.as_str();
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
                hover_border_width: if box_css_p.h_border_width == String::default() {
                    border_width_value
                } else {
                    let temp = box_css_p.h_border_width.as_str();
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
                hover_border_color: if box_css_p.h_border_color == String::default() {
                    "none".to_string()
                } else {
                    box_css_p.h_border_color.clone()
                },
                hover_bg_color: if box_css_p.h_bg_color == String::default() {
                    "none".to_string()
                } else {
                    box_css_p.h_bg_color.clone()
                },
                hover_color: if box_css_p.h_color == String::default() {
                    "none".to_string()
                } else {
                    box_css_p.h_color.clone()
                },
                hover_shadow: if box_css_p.h_shadow == String::default() {
                    "none".to_string()
                } else {
                    box_css_p.h_shadow.clone()
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
                dark_shadow: if box_css_p.d_shadow == String::default() {
                    box_css_p.shadow.clone()
                } else {
                    box_css_p.d_shadow.clone()
                },
                dark_border_color: if box_css_p.d_border_color == String::default() {
                    box_css_p.border_color.clone()
                } else {
                    box_css_p.d_border_color.clone()
                },
                dark_color: if box_css_p.d_color == String::default() {
                    box_css_p.color.clone()
                } else {
                    box_css_p.d_color.clone()
                },
            }
        },
        box_css_p,
    );

    let class = css!(
        r#"
            display: flex;
            flex-direction: ${flex_direction};
            justify-content: ${justify_content};
            align-items: ${align_items};

            width: ${width};
            height: ${height};
            padding: ${padding};
            margin: ${margin};
            box-sizing: ${box_sizing};
            border-radius: ${radius};
            border-width: ${border_width};
            border-color: ${border_color};
            border-style: ${border_style};
            background-color: ${bg_color};
            overflow: ${overflow};
            background-image: ${bg_img};
            background-repeat: no-repeat;
            background-position: center;
            background-size: ${bg_size};
            backdrop-filter: ${backdrop};
            cursor: ${cursor};
            white-space: ${white_space};
            min-width: ${min_width};
            min-height: ${min_height};
            max-width: ${max_width};
            max-height: ${max_height};
            box-shadow: ${box_shadow};
            position: ${position};
            left: ${left};
            top: ${top};
            right: ${right};
            bottom: ${bottom};
            z-index: ${z_index};
            opacity: ${opacity};

            font-size: ${font_size};
            color: ${color};
            font-style: ${font_style};
            font-weight: ${font_weight};
            letter-spacing: ${letter_spacing};
            line-height: ${line_height};
            text-decoration: ${text_decoration};
            text-align: ${text_align};
            word-break: ${word_break};
            flex-shrink: ${flex_shrink};


            transition: all ${duration}s ${timing_fn};

            &:hover {
                background-color: ${hover_bg_color};
                color: ${hover_color};
                box-shadow: ${hover_shadow};
                width: ${hover_width};
                height: ${hover_height};
                padding: ${hover_padding};
                margin: ${hover_margin};
                border-radius: ${hover_radius};
                border-width: ${hover_border_width};
                border-color: ${hover_border_color};
                opacity: ${hover_opacity};
            }

            @media (prefers-color-scheme: dark) {
              & {
                background-color: ${dark_bg_color};
                color: ${dark_color};
                box-shadow: ${dark_shadow};
                border-color: ${dark_border_color};
              }
            }
        "#,
        flex_direction = box_css.flex_direction,
        justify_content = box_css.justify_content,
        align_items = box_css.align_items,
        width = box_css.width,
        height = box_css.height,
        padding = box_css.padding,
        margin = box_css.margin,
        box_sizing = box_css.box_sizing,
        radius = box_css.border_radius,
        border_width = box_css.border_width,
        border_color = box_css.border_color,
        border_style = box_css.border_style,
        bg_color = box_css.background_color,
        bg_img = box_css.background_image,
        bg_size = box_css.background_size,
        overflow = box_css.overflow,
        backdrop = box_css.backdrop_filter,
        cursor = box_css.cursor,
        white_space = box_css.white_space,
        min_width = box_css.min_width,
        min_height = box_css.min_height,
        max_width = box_css.max_width,
        max_height = box_css.max_height,
        box_shadow = box_css.box_shadow,
        position = box_css.position,
        top = box_css.top,
        right = box_css.right,
        bottom = box_css.bottom,
        left = box_css.left,
        z_index = box_css.z_index,
        opacity = box_css.opacity,
        font_size = box_css.font_size,
        color = box_css.color,
        font_style = box_css.font_style,
        font_weight = box_css.font_weight,
        letter_spacing = box_css.letter_spacing,
        line_height = box_css.line_height,
        text_decoration = box_css.text_decoration,
        text_align = box_css.text_align,
        word_break = box_css.word_break,
        flex_shrink = box_css.flex_shrink,
        duration = box_css.duration,
        timing_fn = box_css.timing_fn,
        hover_opacity = box_css.hover_opacity,
        hover_padding = box_css.hover_padding,
        hover_margin = box_css.hover_margin,
        hover_radius = box_css.hover_radius,
        hover_border_width = box_css.hover_border_width,
        hover_border_color = box_css.hover_border_color,
        hover_bg_color = box_css.hover_bg_color,
        hover_color = box_css.hover_color,
        hover_shadow = box_css.hover_shadow,
        hover_width = box_css.hover_width,
        hover_height = box_css.hover_height,
        dark_bg_color = box_css.dark_bg_color,
        dark_shadow = box_css.dark_shadow,
        dark_border_color = box_css.dark_border_color,
        dark_color = box_css.dark_color,
    );

    html! {
        <div {class} onclick={props.onclick.clone()} ref={props.node.clone()}>
        { for props.children.iter() }
        </div>
    }
}
