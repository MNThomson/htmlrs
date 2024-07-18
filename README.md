# htmlrs

[<img alt="github" src="https://img.shields.io/badge/github-MNThomson/htmlrs-bc3f48?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MNThomson/htmlrs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/htmlrs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/htmlrs)


## Usage

```console
cargo add htmlrs
```

Take a look at the [`examples`](./examples/) folder!

```rust
use htmlrs::{component, html, view, Children, IntoView};

#[component]
fn Text(text: String) -> impl IntoView {
    view! {<span>{text}</span>}
}

#[component]
fn TextInto(#[prop(into)] text: String) -> impl IntoView {
    view! {<span>{text}</span>}
}

#[component]
fn Page(children: Children) -> impl IntoView {
    view! {<div>{children()}</div>}
}

fn main() {
    let page = html! {
        <Page>
            <Text text="hello".to_string() />
            <TextInto text="world" />
        </Page>
    };
    println!("{}", page);
    assert_eq!(page, "<div><span>hello</span><span>world</span></div>")
}
```
