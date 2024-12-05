use leptos::prelude::*;
use tic_tac_toe::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let game = RwSignal::new(Game::new());

    let board = [Hor::Left, Hor::Mid, Hor::Right]
        .into_iter()
        .flat_map(|h| {
            [Vert::Top, Vert::Mid, Vert::Bottom]
                .into_iter()
                .map(move |v| view! { <Field game=game field=FieldName{v, h}/> })
        })
        .collect_view();

    view! { <div class="game">
        <div class="board"> {board} </div>
        <div class="state"> {move || game.get().state().to_string()} </div>
    </div> }
}

#[component]
fn Field(#[prop(into)] game: RwSignal<Game>, field: FieldName) -> impl IntoView {
    let class = move || match game.get().get(field).0 {
        Some(Side::X) => "cell side-X",
        Some(Side::O) => "cell side-O",
        None => "cell",
    };
    let action = move |_| {
        game.update(|g| {
            g.act(field);
        })
    };
    let value = move || game.get().get(field).to_string();

    view! { <div class=class on:click=action> {value} </div> }
}
