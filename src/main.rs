use leptos::*;
use tic_tac_toe::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (game, game_mut) = create_signal(Game::new());

    view! { <div class="board"> {
        [Hor::Left, Hor::Mid, Hor::Right]
            .into_iter()
            .flat_map(|h| {
                [Vert::Top, Vert::Mid, Vert::Bottom]
                    .into_iter()
                    .map(move |v|
             view! {<Field
                 value=move || game.get().get(FieldName{v,h})
                 on:click=move |_| game_mut.update(|g| {g.act(FieldName {v, h});})
                 />})})
            .collect_view()
    } </div> }
}

#[component]
fn Field(#[prop(into)] value: Signal<FieldState>) -> impl IntoView {
    view! {
    <div class="cell">
        {move || value.get().to_string() }
        </div>
    }
}
