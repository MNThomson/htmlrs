use htmlrs::{html, view};

fn main() {
    let page = html! {
        <div>
            <p>hello world</p>
        </div>
    };
    println!("{}", page);
    assert_eq!(page, "<div><p>hello world</p></div>")
}
