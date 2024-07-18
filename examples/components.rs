use htmlrs::{component, html, view, IntoView};

#[component]
fn Text(text: String) -> impl IntoView {
    view! {<span>{text}</span>}
}

#[component]
fn TextInto(#[prop(into)] text: String) -> impl IntoView {
    view! {<span>{text}</span>}
}

fn main() {
    let page = html! {
        <div>
            <Text text="hello".to_string() />
            <TextInto text="world" />
        </div>
    };
    println!("{}", page);
    assert_eq!(page, "<div><span>hello</span><span>world</span></div>")
}
