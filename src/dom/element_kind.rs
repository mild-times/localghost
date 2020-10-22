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
    /// [`<hgroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hgroup)
    /// element.
    Hgroup,
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
    Blockquote,
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
    Figcaption,
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
    /// [`<colgroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/colgroup)
    /// element.
    Colgroup,
    /// An HTML
    /// [`<table>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table)
    /// element.
    Table,
    /// An HTML
    /// [`<tbody>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tbody)
    /// element.
    Tbody,
    /// An HTML
    /// [`<td>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td)
    /// element.
    Td,
    /// An HTML
    /// [`<tfoot>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tfoot)
    /// element.
    Tfoot,
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
    Datalist,
    /// An HTML
    /// [`<fieldset>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
    /// element.
    Fieldset,
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
    /// [`<optgroup>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
    /// element.
    Optgroup,
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
    Textarea,

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
    Menuitem,
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
