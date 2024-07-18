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
