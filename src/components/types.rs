#[derive(Clone, PartialEq)]
pub enum BoxSizing {
    BorderBox,
    ContentBox,
}
impl BoxSizing {
    pub fn get_name(&self) -> String {
        match &self {
            &BoxSizing::BorderBox => "border-box".to_owned(),
            &BoxSizing::ContentBox => "content-box".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum BorderStyle {
    No,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}
impl BorderStyle {
    pub fn get_name(&self) -> String {
        match &self {
            &BorderStyle::No => "none".to_owned(),
            &BorderStyle::Hidden => "hidden".to_owned(),
            &BorderStyle::Dotted => "dotted".to_owned(),
            &BorderStyle::Dashed => "dashed".to_owned(),
            &BorderStyle::Solid => "solid".to_owned(),
            &BorderStyle::Double => "double".to_owned(),
            &BorderStyle::Groove => "groove".to_owned(),
            &BorderStyle::Ridge => "ridge".to_owned(),
            &BorderStyle::Inset => "inset".to_owned(),
            &BorderStyle::Outset => "outset".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Clip,
    Scroll,
    Auto,
    VisibleHidden,
    VisibleScroll,
    HiddenVisible,
    HiddenScroll,
    ScrollVisible,
    ScrollHidden,
}
impl Overflow {
    pub fn get_name(&self) -> String {
        match &self {
            &Overflow::Visible => "visible".to_owned(),
            &Overflow::Hidden => "hidden".to_owned(),
            &Overflow::Clip => "clip".to_owned(),
            &Overflow::Scroll => "scroll".to_owned(),
            &Overflow::Auto => "auto".to_owned(),
            &Overflow::VisibleHidden => "visible hidden".to_owned(),
            &Overflow::VisibleScroll => "visible scroll".to_owned(),
            &Overflow::HiddenVisible => "hidden visible".to_owned(),
            &Overflow::HiddenScroll => "hidden scroll".to_owned(),
            &Overflow::ScrollVisible => "scroll visible".to_owned(),
            &Overflow::ScrollHidden => "scroll hidden".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ImageMode {
    Auto,
    AspectFit,
    AspectFill,
}
impl ImageMode {
    pub fn get_name(&self) -> String {
        match &self {
            &ImageMode::Auto => "auto".to_owned(),
            &ImageMode::AspectFit => "contain".to_owned(),
            &ImageMode::AspectFill => "cover".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Cursor {
    Unset,
    Auto,
    Pointer,
    Help,
    Wait,
    Crosshair,
    NotAllowed,
    ZoomIn,
    ZoomOut,
    Grab,
}
impl Cursor {
    pub fn get_name(&self) -> String {
        match &self {
            &Cursor::Unset => "unset".to_owned(),
            &Cursor::Auto => "auto".to_owned(),
            &Cursor::Pointer => "pointer".to_owned(),
            &Cursor::Help => "help".to_owned(),
            &Cursor::Wait => "wait".to_owned(),
            &Cursor::Crosshair => "crosshair".to_owned(),
            &Cursor::NotAllowed => "not-allowed".to_owned(),
            &Cursor::ZoomIn => "zoom-in".to_owned(),
            &Cursor::ZoomOut => "zoom-out".to_owned(),
            &Cursor::Grab => "grab".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum WhiteSpace {
    Normal,
    Nowrap,
    Pre,
    PreWrap,
    PreLine,
    BreakSpaces,
}
impl WhiteSpace {
    pub fn get_name(&self) -> String {
        match &self {
            &WhiteSpace::Normal => "normal".to_owned(),
            &WhiteSpace::Nowrap => "nowrap".to_owned(),
            &WhiteSpace::Pre => "pre".to_owned(),
            &WhiteSpace::PreWrap => "pre-wrap".to_owned(),
            &WhiteSpace::PreLine => "pre-line".to_owned(),
            &WhiteSpace::BreakSpaces => "break-spaces".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Grid,
    InlineGrid,
    FlowRoot,
    None,
    Contents,
}
impl Display {
    pub fn get_name(&self) -> String {
        match &self {
            &Display::Block => "block".to_owned(),
            &Display::Inline => "inline".to_owned(),
            &Display::InlineBlock => "inline-block".to_owned(),
            &Display::Grid => "grid".to_owned(),
            &Display::InlineGrid => "inline-grid".to_owned(),
            &Display::FlowRoot => "flow-root".to_owned(),
            &Display::None => "none".to_owned(),
            &Display::Contents => "contents".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Position {
    Absolute,
    Fixed,
    Relative,
    Static,
    Sticky,
    Inherit,
}
impl Position {
    pub fn get_name(&self) -> String {
        match &self {
            &Position::Absolute => "absolute".to_owned(),
            &Position::Fixed => "fixed".to_owned(),
            &Position::Relative => "relative".to_owned(),
            &Position::Static => "static".to_owned(),
            &Position::Sticky => "sticky".to_owned(),
            &Position::Inherit => "inherit".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
    Inherit,
}
impl FontStyle {
    pub fn get_name(&self) -> String {
        match &self {
            &FontStyle::Normal => "normal".to_owned(),
            &FontStyle::Italic => "italic".to_owned(),
            &FontStyle::Oblique => "oblique".to_owned(),
            &FontStyle::Inherit => "inherit".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    Initial,
    Inherit,
    Num900,
    Num800,
    Num700,
    Num600,
    Num500,
    Num400,
    Num300,
    Num200,
    Num100,
}
impl FontWeight {
    pub fn get_name(&self) -> String {
        match &self {
            &FontWeight::Normal => "normal".to_owned(),
            &FontWeight::Bold => "bold".to_owned(),
            &FontWeight::Bolder => "bolder".to_owned(),
            &FontWeight::Lighter => "lighter".to_owned(),
            &FontWeight::Initial => "initial".to_owned(),
            &FontWeight::Inherit => "inherit".to_owned(),
            &FontWeight::Num900 => "900".to_owned(),
            &FontWeight::Num800 => "800".to_owned(),
            &FontWeight::Num700 => "700".to_owned(),
            &FontWeight::Num600 => "600".to_owned(),
            &FontWeight::Num500 => "500".to_owned(),
            &FontWeight::Num400 => "400".to_owned(),
            &FontWeight::Num300 => "300".to_owned(),
            &FontWeight::Num200 => "200".to_owned(),
            &FontWeight::Num100 => "100".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Start,
    End,
    Justify,
}
impl TextAlign {
    pub fn get_name(&self) -> String {
        match &self {
            &TextAlign::Left => "left".to_owned(),
            &TextAlign::Right => "right".to_owned(),
            &TextAlign::Center => "center".to_owned(),
            &TextAlign::Start => "start".to_owned(),
            &TextAlign::End => "end".to_owned(),
            &TextAlign::Justify => "justify".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum WordBreak {
    BreakAll,
    Normal,
    KeepAll,
    BreakWord,
}
impl WordBreak {
    pub fn get_name(&self) -> String {
        match &self {
            &WordBreak::BreakAll => "break-all".to_owned(),
            &WordBreak::Normal => "normal".to_owned(),
            &WordBreak::KeepAll => "keep-all".to_owned(),
            &WordBreak::BreakWord => "break-word".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum PointerEvents {
    Auto,
    None,
}
impl PointerEvents {
    pub fn get_name(&self) -> String {
        match &self {
            &PointerEvents::Auto => "auto".to_owned(),
            &PointerEvents::None => "none".to_owned(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum SafeType {
    None,
    Top,
    Bottom,
}

#[derive(Clone, PartialEq)]
pub enum FlexWay {
    Fraa,
    Frab,
    Frac,
    Frae,
    Fras,

    Frba,
    Frbb,
    Frbc,
    Frbe,
    Frbs,

    Frca,
    Frcb,
    Frcc,
    Frce,
    Frcs,

    Frea,
    Freb,
    Frec,
    Free,
    Fres,

    Frsa,
    Frsb,
    Frsc,
    Frse,
    Frss,

    Fcaa,
    Fcab,
    Fcac,
    Fcae,
    Fcas,

    Fcba,
    Fcbb,
    Fcbc,
    Fcbe,
    Fcbs,

    Fcca,
    Fccb,
    Fccc,
    Fcce,
    Fccs,

    Fcea,
    Fceb,
    Fcec,
    Fcee,
    Fces,

    Fcsa,
    Fcsb,
    Fcsc,
    Fcse,
    Fcss,
}
impl FlexWay {
    pub fn get_name(&self) -> String {
        match &self {
            &FlexWay::Fraa => "fraa".to_owned(),
            &FlexWay::Frab => "frab".to_owned(),
            &FlexWay::Frac => "frac".to_owned(),
            &FlexWay::Frae => "frae".to_owned(),
            &FlexWay::Fras => "fras".to_owned(),

            &FlexWay::Frba => "frba".to_owned(),
            &FlexWay::Frbb => "frbb".to_owned(),
            &FlexWay::Frbc => "frbc".to_owned(),
            &FlexWay::Frbe => "frbe".to_owned(),
            &FlexWay::Frbs => "frbs".to_owned(),

            &FlexWay::Frca => "frca".to_owned(),
            &FlexWay::Frcb => "frcb".to_owned(),
            &FlexWay::Frcc => "frcc".to_owned(),
            &FlexWay::Frce => "frce".to_owned(),
            &FlexWay::Frcs => "frcs".to_owned(),

            &FlexWay::Frea => "frea".to_owned(),
            &FlexWay::Freb => "freb".to_owned(),
            &FlexWay::Frec => "frec".to_owned(),
            &FlexWay::Free => "free".to_owned(),
            &FlexWay::Fres => "fres".to_owned(),

            &FlexWay::Frsa => "frsa".to_owned(),
            &FlexWay::Frsb => "frsb".to_owned(),
            &FlexWay::Frsc => "frsc".to_owned(),
            &FlexWay::Frse => "frse".to_owned(),
            &FlexWay::Frss => "frss".to_owned(),

            &FlexWay::Fcaa => "fcaa".to_owned(),
            &FlexWay::Fcab => "fcab".to_owned(),
            &FlexWay::Fcac => "fcac".to_owned(),
            &FlexWay::Fcae => "fcae".to_owned(),
            &FlexWay::Fcas => "fcas".to_owned(),

            &FlexWay::Fcba => "fcba".to_owned(),
            &FlexWay::Fcbb => "fcbb".to_owned(),
            &FlexWay::Fcbc => "fcbc".to_owned(),
            &FlexWay::Fcbe => "fcbe".to_owned(),
            &FlexWay::Fcbs => "fcbs".to_owned(),

            &FlexWay::Fcca => "fcca".to_owned(),
            &FlexWay::Fccb => "fccb".to_owned(),
            &FlexWay::Fccc => "fccc".to_owned(),
            &FlexWay::Fcce => "fcce".to_owned(),
            &FlexWay::Fccs => "fccs".to_owned(),

            &FlexWay::Fcea => "fcea".to_owned(),
            &FlexWay::Fceb => "fceb".to_owned(),
            &FlexWay::Fcec => "fcec".to_owned(),
            &FlexWay::Fcee => "fcee".to_owned(),
            &FlexWay::Fces => "fces".to_owned(),

            &FlexWay::Fcsa => "fcsa".to_owned(),
            &FlexWay::Fcsb => "fcsb".to_owned(),
            &FlexWay::Fcsc => "fcsc".to_owned(),
            &FlexWay::Fcse => "fcse".to_owned(),
            &FlexWay::Fcss => "fcss".to_owned(),
        }
    }
}
