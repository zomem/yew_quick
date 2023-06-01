use yew::{function_component, html, use_memo, Callback, Children, Html, MouseEvent, Properties};

use regex::Regex;
use stylist::css;

use crate::{Cursor, FontStyle, FontWeight, TextAlign, WhiteSpace, WordBreak};

#[derive(Clone, PartialEq)]
struct TextEllipsisCss {
    line: String,
    width: String,
    height: String,
    padding: String,
    margin: String,

    background_image: String,
    cursor: String,
    white_space: String,
    min_width: String,
    min_height: String,
    max_width: String,
    max_height: String,

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

    d: String,
    hover_color: String,
    hover_width: String,
    hover_height: String,
}

#[derive(Clone, PartialEq)]
struct TextEllipsisCssProps {
    line: String,
    size: String,
    padding: String,
    margin: String,

    bg_image: String,
    cursor: Cursor,
    white_space: WhiteSpace,
    min_size: String,
    max_size: String,

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

    d: String,
    h_color: String,
    h_size: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TextEllipsisProps {
    pub line: String,
    #[prop_or(String::from("auto auto"))]
    pub size: String,
    #[prop_or(String::from("0"))]
    pub padding: String,
    #[prop_or(String::from("0"))]
    pub margin: String,

    #[prop_or(String::from("0"))]
    pub bg_image: String,

    #[prop_or(Cursor::Unset)]
    pub cursor: Cursor,
    #[prop_or(WhiteSpace::Normal)]
    pub white_space: WhiteSpace,
    #[prop_or(String::from("auto auto"))]
    pub min_size: String,
    #[prop_or(String::from("auto auto"))]
    pub max_size: String,

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

    #[prop_or(String::from("0"))]
    pub d: String,
    #[prop_or(String::from(""))]
    pub h_color: String,
    #[prop_or(String::from(""))]
    pub h_size: String,

    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// ### 使用示例
///
///```
/// line: String, // "2"  多少行之后，显示省略号
/// size: String,
/// padding: String,
/// margin: String,
/// bg_image: String,  // linear-gradient, 文字渐变
/// cursor: Cursor,
/// white_space: WhiteSpace,
/// min_size: String,
/// max_size: String,
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
/// d: String,
/// h_color: String, // hover 时的文字颜色
/// h_size: String,  // hover 时的长宽
///```
#[function_component]
pub fn TextEllipsis(props: &TextEllipsisProps) -> Html {
    let box_css_p: TextEllipsisCssProps = TextEllipsisCssProps {
        line: props.line.clone(),
        size: props.size.clone(),
        padding: props.padding.clone(),
        margin: props.margin.clone(),

        bg_image: props.bg_image.clone(),

        cursor: props.cursor.clone(),
        white_space: props.white_space.clone(),
        min_size: props.min_size.clone(),
        max_size: props.max_size.clone(),

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

        d: props.d.clone(),
        h_color: props.h_color.clone(),
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

            TextEllipsisCss {
                line: box_css_p.line.clone(),
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

                d: box_css_p.d.clone(),
                hover_color: if box_css_p.h_color == String::default() {
                    box_css_p.color.clone()
                } else {
                    box_css_p.h_color.clone()
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

            background-image: ${bg_img};
            cursor: ${cursor};
            white-space: ${white_space};
            min-width: ${min_width};
            min-height: ${min_height};
            max-width: ${max_width};
            max-height: ${max_height};
            z-index: ${z_index};
            opacity: ${opacity};

           	overflow: hidden;
           	text-overflow: ellipsis;
           	word-wrap: break-word;
           	white-space: normal !important;
           	-webkit-line-clamp: ${line};
           	-webkit-box-orient: vertical;

            font-size: ${font_size};
            color: ${color};
            font-style: ${font_style};
            font-weight: ${font_weight};
            letter-spacing: ${letter_spacing};
            line-height: ${line_height};
            text-decoration: ${text_decoration};
            text-align: ${text_align};
            word-break: ${word_break};

            transition: all ${d}s;
            &:hover {
                color: ${hover_color};
                width: ${hover_width};
                height: ${hover_height};
            }
        "#,
        line = box_css.line,
        width = box_css.width,
        height = box_css.height,
        padding = box_css.padding,
        margin = box_css.margin,
        bg_img = box_css.background_image,
        cursor = box_css.cursor,
        white_space = box_css.white_space,
        min_width = box_css.min_width,
        min_height = box_css.min_height,
        max_width = box_css.max_width,
        max_height = box_css.max_height,
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
        d = box_css.d,
        hover_color = box_css.hover_color,
        hover_width = box_css.hover_width,
        hover_height = box_css.hover_height,
    );

    let do_click = {
        let click = props.onclick.clone();
        Callback::from(move |e: MouseEvent| click.emit(e))
    };

    html! {
        <span {class} onclick={do_click}>
        { for props.children.iter() }
        </span>
    }
}