use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter_state = use_state(|| 0);

    let incr_counter = {
        let counter_state = counter_state.clone();
        Callback::from(move |_| counter_state.set(*counter_state + 1))
    };

    let decr_counter = {
        let counter_state = counter_state.clone();
        Callback::from(move |_| counter_state.set(*counter_state - 1))
    };

    // Rust Yewのhtml!マクロでランディングページを作りたい。
    html! {
        <>
            <main>
                <header>
                    <div class="logo">
                        <a href="/">{ "Shibasou HP" }</a>
                    </div>
                    <nav>
                        <ul>
                            <li><a href="/">{ "ホーム" }</a></li>
                            <li><a href="/products">{ "商品一覧" }</a></li>
                            <li><a href="/services">{ "サービス" }</a></li>
                            <li><a href="/about">{ "会社情報" }</a></li>
                            <li><a href="/contact">{ "お問い合わせ" }</a></li>
                        </ul>
                    </nav>
                </header>
                <section class="main-visual">
                    <h1>{ "あなたのビジネスを支えるShibasouのBtoBソリューション" }</h1>
                    <img src="path/to/image.jpg" alt="ビジネスシーンのイメージ画像" />
                </section>
                <div>
                     <p>{ "current count: " }{ *counter_state }</p>
                    <button onclick={incr_counter}>{ "+" }</button>
                    <button onclick={decr_counter}>{ "-" }</button>
                </div>
            </main>
        </>
    }
}
