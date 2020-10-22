use std::fmt::Display;

/// The kind of element to construct.
#[derive(Debug)]
pub enum ElementKind {
    // Document metadata
    /// An HTML
    /// [`<base>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
    /// element.
    Base,
    /// An HTML
    /// [`<head>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/head)
    /// element.
    Head,
    /// An HTML
    /// [`<link>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
    /// element.
    Link,
    /// An HTML
    /// [`<meta>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
    /// element.
    Meta,
    /// An HTML
    /// [`<style>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/style)
    /// element.
    Style,
    /// An HTML
    /// [`<title>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title)
    /// element.
    Title,

    // Sectioning root
    /// An HTML
    /// [`<body>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body)
    /// element.
    Body,

    // Content sectioning
    /// An HTML
    /// [`<address>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/address)
    /// element.
    Address,
    /// An HTML
    /// [`<article>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/article)
    /// element.
    Article,
    /// An HTML
    /// [`<aside>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/aside)
    /// element.
    Aside,
    /// An HTML
    /// [`<footer>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/footer)
    /// element.
    Footer,
    /// An HTML
    /// [`<header>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/header)
    /// element.
    Header,
    /// An HTML
    /// [`<h1>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h1)
    /// element.
    H1,
    /// An HTML
    /// [`<h2>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h2)
    /// element.
    H2,
    /// An HTML
    /// [`<h3>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h3)
    /// element.
    H3,
    /// An HTML
    /// [`<h4>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h4)
    /// element.
    H4,
    /// An HTML
    /// [`<h5>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h5)
    /// element.
    H5,
    /// An HTML
    /// [`<h6>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/h6)
    /// element.
    H6,
    /// An HTML
    /// [`<hGroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup)
    /// element.
    HGroup,
    /// An HTML
    /// [`<main>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/main)
    /// element.
    Main,
    /// An HTML
    /// [`<nav>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav)
    /// element.
    Nav,
    /// An HTML
    /// [`<section>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
    /// element.
    Section,

    // Text content
    /// An HTML
    /// [`<blockquote>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote)
    /// element.
    BlockQuote,
    /// An HTML
    /// [`<dd>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
    /// element.
    Dd,
    /// An HTML
    /// [`<div>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
    /// element.
    Div,
    /// An HTML
    /// [`<dl>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dl)
    /// element.
    Dl,
    /// An HTML
    /// [`<dt>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt)
    /// element.
    Dt,
    /// An HTML
    /// [`<figcaption>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
    /// element.
    FigCaption,
    /// An HTML
    /// [`<figure>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figure)
    /// element.
    Figure,
    /// An HTML
    /// [`<hr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr)
    /// element.
    Hr,
    /// An HTML
    /// [`<li>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li)
    /// element.
    Li,
    /// An HTML
    /// [`<ol>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol)
    /// element.
    Ol,
    /// An HTML
    /// [`<p>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
    /// element.
    P,
    /// An HTML
    /// [`<pre>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)
    /// element.
    Pre,
    /// An HTML
    /// [`<ul>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ul)
    /// element.
    Ul,

    // Inline text semantics
    /// An HTML
    /// [`<a>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
    /// element.
    A,
    /// An HTML
    /// [`<abbr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/abbr)
    /// element.
    Abbr,
    /// An HTML
    /// [`<b>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b)
    /// element.
    B,
    /// An HTML
    /// [`<bdi>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdi)
    /// element.
    Bdi,
    /// An HTML
    /// [`<bdo>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/bdo)
    /// element.
    Bdo,
    /// An HTML
    /// [`<br>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br)
    /// element.
    Br,
    /// An HTML
    /// [`<cite>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/cite)
    /// element.
    Cite,
    /// An HTML
    /// [`<code>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code)
    /// element.
    Code,
    /// An HTML
    /// [`<data>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data)
    /// element.
    Data,
    /// An HTML
    /// [`<dfn>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dfn)
    /// element.
    Dfn,
    /// An HTML
    /// [`<em>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/em)
    /// element.
    Em,
    /// An HTML
    /// [`<i>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/i)
    /// element.
    I,
    /// An HTML
    /// [`<kbd>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/kbd)
    /// element.
    Kbd,
    /// An HTML
    /// [`<mark>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/mark)
    /// element.
    Mark,
    /// An HTML
    /// [`<q>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/q)
    /// element.
    Q,
    /// An HTML
    /// [`<rb>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rb)
    /// element.
    Rb,
    /// An HTML
    /// [`<rp>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rp)
    /// element.
    Rp,
    /// An HTML
    /// [`<rt>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rt)
    /// element.
    Rt,
    /// An HTML
    /// [`<rtc>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/rtc)
    /// element.
    Rtc,
    /// An HTML
    /// [`<ruby>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ruby)
    /// element.
    Ruby,
    /// An HTML
    /// [`<s>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)
    /// element.
    S,
    /// An HTML
    /// [`<samp>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/samp)
    /// element.
    Samp,
    /// An HTML
    /// [`<small>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/small)
    /// element.
    Small,
    /// An HTML
    /// [`<span>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)
    /// element.
    Span,
    /// An HTML
    /// [`<strong>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/strong)
    /// element.
    Strong,
    /// An HTML
    /// [`<sub>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub)
    /// element.
    Sub,
    /// An HTML
    /// [`<sup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sup)
    /// element.
    Sup,
    /// An HTML
    /// [`<time>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time)
    /// element.
    Time,
    /// An HTML
    /// [`<u>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u)
    /// element.
    U,
    /// An HTML
    /// [`<var>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/var)
    /// element.
    Var,
    /// An HTML
    /// [`<wbr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/wbr)
    /// element.
    Wbr,

    // Image and multimedia
    /// An HTML
    /// [`<area>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
    /// element.
    Area,
    /// An HTML
    /// [`<audio>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
    /// element.
    Audio,
    /// An HTML
    /// [`<img>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
    /// element.
    Img,
    /// An HTML
    /// [`<map>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map)
    /// element.
    Map,
    /// An HTML
    /// [`<track>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track)
    /// element.
    Track,
    /// An HTML
    /// [`<video>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video)
    /// element.
    Video,

    // Embedded content
    /// An HTML
    /// [`<embed>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/embed)
    /// element.
    Embed,
    /// An HTML
    /// [`<iframe>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe)
    /// element.
    Iframe,
    /// An HTML
    /// [`<object>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/object)
    /// element.
    Object,
    /// An HTML
    /// [`<param>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/param)
    /// element.
    Param,
    /// An HTML
    /// [`<picture>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture)
    /// element.
    Picture,
    /// An HTML
    /// [`<source>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
    /// element.
    Source,

    // Scripting
    /// An HTML
    /// [`<canvas>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)
    /// element.
    Canvas,
    /// An HTML
    /// [`<noscript>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/noscript)
    /// element.
    Noscript,
    /// An HTML
    /// [`<script>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script)
    /// element.
    Script,

    // Demarcating edits
    /// An HTML
    /// [`<del>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
    /// element.
    Del,
    /// An HTML
    /// [`<ins>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ins)
    /// element.
    Ins,

    // Table content
    /// An HTML
    /// [`<caption>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/caption)
    /// element.
    Caption,
    /// An HTML
    /// [`<col>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col)
    /// element.
    Col,
    /// An HTML
    /// [`<colGroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup)
    /// element.
    ColGroup,
    /// An HTML
    /// [`<table>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table)
    /// element.
    Table,
    /// An HTML
    /// [`<tbody>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tbody)
    /// element.
    TBody,
    /// An HTML
    /// [`<td>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
    /// element.
    Td,
    /// An HTML
    /// [`<tfoot>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot)
    /// element.
    TFoot,
    /// An HTML
    /// [`<th>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/th)
    /// element.
    Th,
    /// An HTML
    /// [`<thead>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/thead)
    /// element.
    Thead,
    /// An HTML
    /// [`<tr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tr)
    /// element.
    Tr,

    // Forms
    /// An HTML
    /// [`<button>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
    /// element.
    Button,
    /// An HTML
    /// [`<datalist>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist)
    /// element.
    DataList,
    /// An HTML
    /// [`<fieldset>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
    /// element.
    FieldSet,
    /// An HTML
    /// [`<form>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/form)
    /// element.
    Form,
    /// An HTML
    /// [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
    /// element.
    Input,
    /// An HTML
    /// [`<label>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label)
    /// element.
    Label,
    /// An HTML
    /// [`<legend>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/legend)
    /// element.
    Legend,
    /// An HTML
    /// [`<meter>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter)
    /// element.
    Meter,
    /// An HTML
    /// [`<optGroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
    /// element.
    OptGroup,
    /// An HTML
    /// [`<option>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option)
    /// element.
    Option,
    /// An HTML
    /// [`<output>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/output)
    /// element.
    Output,
    /// An HTML
    /// [`<progress>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress)
    /// element.
    Progress,
    /// An HTML
    /// [`<select>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
    /// element.
    Select,
    /// An HTML
    /// [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    /// element.
    TextArea,

    // Interactive elements
    /// An HTML
    /// [`<details>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
    /// element.
    Details,
    /// An HTML
    /// [`<dialog>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
    /// element.
    Dialog,
    /// An HTML
    /// [`<menu>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menu)
    /// element.
    Menu,
    /// An HTML
    /// [`<menuitem>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menuitem)
    /// element.
    MenuItem,
    /// An HTML
    /// [`<summary>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/summary)
    /// element.
    Summary,

    // Web components
    /// An HTML
    /// [`<slot>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
    /// element.
    Slot,
    /// An HTML
    /// [`<template>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template)
    /// element.
    Template,
}

impl ElementKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Head => "head",
            Self::Link => "link",
            Self::Meta => "meta",
            Self::Style => "style",
            Self::Title => "title",
            Self::Body => "body",
            Self::Address => "address",
            Self::Article => "article",
            Self::Aside => "aside",
            Self::Footer => "footer",
            Self::Header => "header",
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::HGroup => "hgroup",
            Self::Main => "main",
            Self::Nav => "nav",
            Self::Section => "section",
            Self::BlockQuote => "blockquote",
            Self::Dd => "dd",
            Self::Div => "div",
            Self::Dl => "dl",
            Self::Dt => "dt",
            Self::FigCaption => "figcaption",
            Self::Figure => "figure",
            Self::Hr => "hr",
            Self::Li => "li",
            Self::Ol => "ol",
            Self::P => "p",
            Self::Pre => "pre",
            Self::Ul => "ul",
            Self::A => "a",
            Self::Abbr => "abbr",
            Self::B => "b",
            Self::Bdi => "bdi",
            Self::Bdo => "bdo",
            Self::Br => "br",
            Self::Cite => "cite",
            Self::Code => "code",
            Self::Data => "data",
            Self::Dfn => "dfn",
            Self::Em => "em",
            Self::I => "i",
            Self::Kbd => "kbd",
            Self::Mark => "mark",
            Self::Q => "q",
            Self::Rb => "rb",
            Self::Rp => "rp",
            Self::Rt => "rt",
            Self::Rtc => "rtc",
            Self::Ruby => "ruby",
            Self::S => "s",
            Self::Samp => "samp",
            Self::Small => "small",
            Self::Span => "span",
            Self::Strong => "strong",
            Self::Sub => "sub",
            Self::Sup => "sup",
            Self::Time => "time",
            Self::U => "u",
            Self::Var => "var",
            Self::Wbr => "wbr",
            Self::Area => "area",
            Self::Audio => "audio",
            Self::Img => "img",
            Self::Map => "map",
            Self::Track => "track",
            Self::Video => "video",
            Self::Embed => "embed",
            Self::Iframe => "iframe",
            Self::Object => "object",
            Self::Param => "param",
            Self::Picture => "picture",
            Self::Source => "source",
            Self::Canvas => "canvas",
            Self::Noscript => "noscript",
            Self::Script => "script",
            Self::Del => "del",
            Self::Ins => "ins",
            Self::Caption => "caption",
            Self::Col => "col",
            Self::ColGroup => "colgroup",
            Self::Table => "table",
            Self::TBody => "tbody",
            Self::Td => "td",
            Self::TFoot => "tfoot",
            Self::Th => "th",
            Self::Thead => "thead",
            Self::Tr => "tr",
            Self::Button => "button",
            Self::DataList => "datalist",
            Self::FieldSet => "fieldset",
            Self::Form => "form",
            Self::Input => "input",
            Self::Label => "label",
            Self::Legend => "legend",
            Self::Meter => "meter",
            Self::OptGroup => "optgroup",
            Self::Option => "option",
            Self::Output => "output",
            Self::Progress => "progress",
            Self::Select => "select",
            Self::TextArea => "textarea",
            Self::Details => "details",
            Self::Dialog => "dialog",
            Self::Menu => "menu",
            Self::MenuItem => "menuitem",
            Self::Summary => "summary",
            Self::Slot => "slot",
            Self::Template => "template",
        }
    }
}

impl Display for ElementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
