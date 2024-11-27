use sycamore::web::{
    tags::{h1, h2, h3, p, span},
    GlobalProps, HtmlGlobalAttributes, View,
};
use typewind::{
    typography::{
        FontSize, FontWeight, LineClamp, TextAlign, TextColor, TextOverflow, TextTransform,
    },
    ToClasses,
};

/// Possible HTML tags that can be used for texts.
pub enum TextTag {
    /// `<h1>`
    H1,
    /// `<h2>`
    H2,
    /// `<h3>`
    H3,
    /// `<p>`
    P,
    /// `<span>`
    Span,
}

/// Represents a text with configurable properties for text color, font size,
/// line clamp, text alignment, text transform and text overflow.
#[derive(ToClasses)]
pub struct Text {
    #[tw(skip)]
    text: String,
    #[tw(skip)]
    tag: TextTag,
    font_size: Option<FontSize>,
    font_weight: Option<FontWeight>,
    line_clamp: Option<LineClamp>,
    align: Option<TextAlign>,
    color: Option<TextColor>,
    transform: Option<TextTransform>,
    overflow: Option<TextOverflow>,
}

impl Text {
    /// Creates a new `Text` with the specified [`Signal<String>`] text.
    ///
    /// By default the text is created with the `P` tag.
    pub fn new(text: String) -> Self {
        Self {
            text,
            tag: TextTag::P,
            font_size: None,
            font_weight: None,
            line_clamp: None,
            align: None,
            color: None,
            transform: None,
            overflow: None,
        }
    }

    /// Creates a new large title `Text` with the specified text.
    ///
    /// By default the large title text is created with the `H1` tag and `TextOverflow::Truncate`,
    /// `FontSize::_3xl` and `FontWeight::Bold` properties.
    ///
    /// # Example
    /// ```
    /// use nero_components::Text;
    ///
    /// let text = Text::large_title("Large title!".to_owned());
    /// ```
    pub fn large_title(text: String) -> Self {
        Self::new(text)
            .tag(TextTag::H1)
            .overflow(TextOverflow::Truncate)
            .font_size(FontSize::_3xl)
            .font_weight(FontWeight::Bold)
    }

    /// Creates a new medium title `Text` with the specified text.
    ///
    /// By default the medium title text is created with the `H2` tag and `TextOverflow::Truncate`,
    /// `FontSize::_2xl` and `FontWeight::Semibold` properties.
    ///
    /// # Example
    /// ```
    /// use nero_components::Text;
    ///
    /// let text = Text::medium_title("Medium title!".to_owned());
    /// ```
    pub fn medium_title(text: String) -> Self {
        Self::new(text)
            .tag(TextTag::H2)
            .overflow(TextOverflow::Truncate)
            .font_size(FontSize::_2xl)
            .font_weight(FontWeight::Semibold)
    }

    /// Sets the HTML tag to be used for the text (defaults to `P`).
    pub fn tag(mut self, tag: TextTag) -> Self {
        self.tag = tag;
        self
    }

    /// Sets the `font-size` property of the text.
    pub fn font_size(mut self, font_size: FontSize) -> Self {
        self.font_size = Some(font_size);
        self
    }

    /// Sets the `font-weight` property of the text.
    pub fn font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = Some(font_weight);
        self
    }

    /// Sets the line clamp of the text.
    pub fn line_clamp(mut self, line_clamp: LineClamp) -> Self {
        self.line_clamp = Some(line_clamp);
        self
    }

    /// Sets the text alignment of the text.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Sets the text color of the text.
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the `text-transform` property of the text.
    pub fn transform(mut self, transform: TextTransform) -> Self {
        self.transform = Some(transform);
        self
    }

    /// Sets the `text-overflow` property of the text.
    pub fn overflow(mut self, overflow: TextOverflow) -> Self {
        self.overflow = Some(overflow);
        self
    }
}

impl From<Text> for View {
    fn from(value: Text) -> Self {
        let classes = value.classes();
        match value.tag {
            TextTag::H1 => h1().class(classes).children(value.text).into(),
            TextTag::H2 => h2().class(classes).children(value.text).into(),
            TextTag::H3 => h3().class(classes).children(value.text).into(),
            TextTag::P => p().class(classes).children(value.text).into(),
            TextTag::Span => span().class(classes).children(value.text).into(),
        }
    }
}
