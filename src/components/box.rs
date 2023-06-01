use yew::{function_component, html, use_memo, Callback, Children, Html, MouseEvent, Properties};

use regex::Regex;
use stylist::css;

use crate::{
    BorderStyle, BoxSizing, Cursor, Display, FontStyle, FontWeight, ImageMode, Overflow, Position,
    TextAlign, WhiteSpace, WordBreak,
};

#[derive(Clone, PartialEq)]
struct BoxCss {
    display: String,
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

    d: String,
    hover_bg_color: String,
    hover_color: String,
    hover_shadow: String,
    hover_width: String,
    hover_height: String,
}

#[derive(Clone, PartialEq)]
struct BoxCssProps {
    display: Display,
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

    d: String,
    h_bg_color: String,
    h_color: String,
    h_shadow: String,
    h_size: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BoxProps {
    #[prop_or(Display::InlineBlock)]
    pub display: Display,
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
    pub d: String,
    #[prop_or(String::from(""))]
    pub h_bg_color: String,
    #[prop_or(String::from(""))]
    pub h_color: String,
    #[prop_or(String::from(""))]
    pub h_shadow: String,
    #[prop_or(String::from(""))]
    pub h_size: String,

    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// ### 使用示例
/// ```
/// display: Display,
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
/// d: String, // transition 时间 s
/// h_bg_color: String,  // hover 样式
/// h_color: String,  // hover 样式
/// h_shadow: String,  // hover 样式
/// h_size: String,  // hover 样式
/// ```
#[function_component]
pub fn Box(props: &BoxProps) -> Html {
    let box_css_p: BoxCssProps = BoxCssProps {
        display: props.display.clone(),
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

        d: props.d.clone(),
        h_bg_color: props.h_bg_color.clone(),
        h_color: props.h_color.clone(),
        h_shadow: props.h_shadow.clone(),
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

            let temp_padding = box_css_p.padding.as_str();
            let temp_margin = box_css_p.margin.as_str();
            let temp_radius = box_css_p.radius.as_str();
            let temp_border_width = box_css_p.border_width.as_str();

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

            let temp_min_width_op = op_rex.replace_all(temp_min_width, " $1 ").to_string();
            let tmep_min_width_f = if temp_min_width.contains("%") {
                format!("{}", temp_min_width)
            } else {
                format!("{}{}", temp_min_width, "px")
            };
            let temp_min_height_op = op_rex.replace_all(temp_min_height, " $1 ").to_string();
            let temp_min_height_f = if temp_min_height.contains("%") {
                format!("{}", temp_min_height)
            } else {
                format!("{}{}", temp_min_height, "px")
            };
            let temp_max_width_op = op_rex.replace_all(temp_max_width, " $1 ").to_string();
            let tmep_max_width_f = if temp_max_width.contains("%") {
                format!("{}", temp_max_width)
            } else {
                format!("{}{}", temp_max_width, "px")
            };
            let temp_max_height_op = op_rex.replace_all(temp_max_height, " $1 ").to_string();
            let temp_max_height_f = if temp_max_height.contains("%") {
                format!("{}", temp_max_height)
            } else {
                format!("{}{}", temp_max_height, "px")
            };

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
            BoxCss {
                display: box_css_p.display.get_name(),
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
                padding: if w_rex.is_match(temp_padding) {
                    temp_padding.to_owned()
                } else {
                    temp_padding_c
                },
                margin: if w_rex.is_match(temp_margin) {
                    temp_margin.to_owned()
                } else {
                    temp_margin_c
                },
                box_sizing: box_css_p.box_sizing.get_name(),
                border_radius: if w_rex.is_match(temp_radius) {
                    temp_radius.to_owned()
                } else {
                    temp_radius_c
                },
                border_width: if w_rex.is_match(temp_border_width) {
                    temp_border_width.to_owned()
                } else {
                    temp_border_width_c
                },
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
                } else if w_rex.is_match(temp_min_width) {
                    temp_min_width_op
                } else {
                    tmep_min_width_f
                },
                min_height: if temp_min_height == "auto" {
                    "auto".to_owned()
                } else if w_rex.is_match(temp_min_height) {
                    temp_min_height_op
                } else {
                    temp_min_height_f
                },
                max_width: if temp_max_width == "auto" {
                    "none".to_owned()
                } else if w_rex.is_match(temp_max_width) {
                    temp_max_width_op
                } else {
                    tmep_max_width_f
                },
                max_height: if temp_max_height == "auto" {
                    "none".to_owned()
                } else if w_rex.is_match(temp_max_height) {
                    temp_max_height_op
                } else {
                    temp_max_height_f
                },
                box_shadow: box_css_p.shadow.clone(),
                position: box_css_p.position.get_name(),
                top: box_css_p.top.clone(),
                right: box_css_p.right.clone(),
                bottom: box_css_p.bottom.clone(),
                left: box_css_p.left.clone(),
                z_index: box_css_p.z_index.clone(),
                opacity: box_css_p.opacity.clone(),

                font_size: if w_rex.is_match(&box_css_p.font_size) {
                    box_css_p.font_size.to_owned()
                } else {
                    box_css_p.font_size.clone() + "px"
                },
                color: box_css_p.color.clone(),
                font_style: box_css_p.font_style.get_name(),
                font_weight: box_css_p.font_weight.get_name(),
                letter_spacing: if w_rex.is_match(&box_css_p.letter_spacing) {
                    box_css_p.letter_spacing.to_owned()
                } else {
                    box_css_p.letter_spacing.clone() + "px"
                },
                line_height: if w_rex.is_match(&box_css_p.line_height) {
                    box_css_p.line_height.to_owned()
                } else {
                    box_css_p.line_height.clone() + "px"
                },
                text_decoration: box_css_p.text_decoration.clone(),
                text_align: box_css_p.text_align.get_name(),
                word_break: box_css_p.word_break.get_name(),
                flex_shrink: box_css_p.flex_shrink.clone(),

                d: box_css_p.d.clone(),
                hover_bg_color: if box_css_p.h_bg_color == String::default() {
                    box_css_p.bg_color.clone()
                } else {
                    box_css_p.h_bg_color.clone()
                },
                hover_color: if box_css_p.h_color == String::default() {
                    box_css_p.color.clone()
                } else {
                    box_css_p.h_color.clone()
                },
                hover_shadow: if box_css_p.h_color == String::default() {
                    box_css_p.shadow.clone()
                } else {
                    box_css_p.h_shadow.clone()
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
            display: ${display};
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


            transition: all ${d}s;

            &:hover {
                background-color: ${hover_bg_color};
                color: ${hover_color};
                box-shadow: ${hover_shadow};
                width: ${hover_width};
                height: ${hover_height};
            }
        "#,
        display = box_css.display,
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
        d = box_css.d,
        hover_bg_color = box_css.hover_bg_color,
        hover_color = box_css.hover_color,
        hover_shadow = box_css.hover_shadow,
        hover_width = box_css.hover_width,
        hover_height = box_css.hover_height,
    );

    let do_click = {
        let click = props.onclick.clone();
        Callback::from(move |e: MouseEvent| click.emit(e))
    };

    html! {
        <div {class} onclick={do_click}>
        { for props.children.iter() }
        </div>
    }
}
