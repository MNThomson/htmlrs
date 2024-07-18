use criterion::{black_box, criterion_group, criterion_main, Criterion};
use htmlrs::{component, html, view, Children, IntoView};
use hypertext::{html_elements, rsx, rsx_move, Renderable};
use maud::{html as mhtml, Markup};

#[component]
fn Hellos(num: usize) -> impl IntoView {
    view! {
        <div>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    </div>
    }
}

fn h_hellos(num: usize) -> impl Renderable {
    rsx_move! {
        <div>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    <p>Hello {num}</p>
    </div>
    }
}

fn m_hellos(num: usize) -> Markup {
    mhtml! {
        div {
            p {"Hello " (num)}
            p {"Hello " (num)}
            p {"Hello " (num)}
            p {"Hello " (num)}
            p {"Hello " (num)}
            p {"Hello " (num)}
            p {"Hello " (num)}
            p {"Hello " (num)}
            p {"Hello " (num)}
            p {"Hello " (num)}
        }
    }
}

#[component]
fn HelloBuilder(num: usize, children: Children) -> impl IntoView {
    view! {
        {children()}
        <Hellos num={num}/>
        <Hellos num={num}/>
        <Hellos num={num}/>
        <Hellos num={num}/>
        <Hellos num={num}/>
        <Hellos num={num}/>
        <Hellos num={num}/>
        <Hellos num={num}/>
        <Hellos num={num}/>
    }
}

fn h_hellobuilder(num: usize, children: impl Renderable) -> impl Renderable {
    rsx_move! {
        {children}
        {h_hellos(num)}
        {h_hellos(num)}
        {h_hellos(num)}
        {h_hellos(num)}
        {h_hellos(num)}
        {h_hellos(num)}
        {h_hellos(num)}
        {h_hellos(num)}
        {h_hellos(num)}
    }
}

fn m_hellobuilder(num: usize, children: Markup) -> Markup {
    mhtml! {
            (children)
            (m_hellos(num))
            (m_hellos(num))
            (m_hellos(num))
            (m_hellos(num))
            (m_hellos(num))
            (m_hellos(num))
            (m_hellos(num))
            (m_hellos(num))
            (m_hellos(num))
    }
}

#[component]
fn MehG(num: usize) -> impl IntoView {
    view! {
        <div>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
        <HelloBuilder num={num}><Hellos num={num}/></HelloBuilder>
    </div>
    }
}

fn h_mehg(num: usize) -> impl Renderable {
    rsx_move! {
        <div>
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        {h_hellobuilder(num, h_hellos(num))}
        </div>
    }
}

fn m_mehg(num: usize) -> Markup {
    mhtml! {
        div {
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
            (m_hellobuilder(num, m_hellos(num)))
    }
    }
}

#[component]
fn MiddleG(num: usize) -> impl IntoView {
    view! {
        <div>
        <MehG num={num}/>
        <MehG num={num}/>
        <MehG num={num}/>
        <MehG num={num}/>
        <MehG num={num}/>
        <MehG num={num}/>
        <MehG num={num}/>
        <MehG num={num}/>
        <MehG num={num}/>
        <MehG num={num}/>
    </div>
    }
}

fn h_middleg(num: usize) -> impl Renderable {
    rsx_move! {
        <div>
            {h_mehg(num)}
            {h_mehg(num)}
            {h_mehg(num)}
            {h_mehg(num)}
            {h_mehg(num)}
            {h_mehg(num)}
            {h_mehg(num)}
            {h_mehg(num)}
            {h_mehg(num)}
            {h_mehg(num)}
        </div>
    }
}

fn m_middleg(num: usize) -> Markup {
    mhtml! {
        div {
            (m_mehg(num))
            (m_mehg(num))
            (m_mehg(num))
            (m_mehg(num))
            (m_mehg(num))
            (m_mehg(num))
            (m_mehg(num))
            (m_mehg(num))
            (m_mehg(num))
            (m_mehg(num))
    }
    }
}

#[component]
fn BottomG(num: usize) -> impl IntoView {
    view! {
        <div>
        <MiddleG num={num}/>
        <MiddleG num={num}/>
        <MiddleG num={num}/>
        <MiddleG num={num}/>
        <MiddleG num={num}/>
        <MiddleG num={num}/>
        <MiddleG num={num}/>
    </div>
    }
}

fn h_bottomg(num: usize) -> impl Renderable {
    rsx_move! {
        <div>
        {h_middleg(num)}
        {h_middleg(num)}
        {h_middleg(num)}
        {h_middleg(num)}
        {h_middleg(num)}
        {h_middleg(num)}
        {h_middleg(num)}
        </div>
    }
}

fn m_bottomg(num: usize) -> Markup {
    mhtml! {
        div {
            (m_middleg(num))
            (m_middleg(num))
            (m_middleg(num))
            (m_middleg(num))
            (m_middleg(num))
            (m_middleg(num))
            (m_middleg(num))
    }
    }
}

pub fn compare_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("rendering");
    group.bench_function("htmlrs", |b| {
        b.iter(|| {
            let page = html! {
                <html>
                    <head>
                        <title>Large HTML page</title>
                    </head>
                    <body>
                        <BottomG num=black_box(1)/>
                    </body>
                </html>
            };
            page.len()
            //eprintln!("{:.1}kb", page.len() as f32 / 1024.0);
        })
    });
    group.bench_function("maud", |b| {
        b.iter(|| {
            let page: String = mhtml! {
            (m_bottomg(black_box(1)))
            }
            .into();
            page.len()
            //eprintln!("{:.1}kb", page.len() as f32 / 1024.0);
        })
    });
    group.bench_function("hypertext", |b| {
        b.iter(|| {
            let page: String = rsx! {
                {h_bottomg(black_box(1))}
            }
            .render()
            .into();
            page.len()
            //eprintln!("{:.1}kb", page.len() as f32 / 1024.0);
        })
    });
}

criterion_group!(benches, compare_bench);
criterion_main!(benches);
