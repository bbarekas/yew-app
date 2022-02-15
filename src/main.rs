mod app;

//use yew::prelude::*;

/*
#[derive(Properties, PartialEq)]
struct TodoProps {
    state: UseStateHandle<u64>,
}

#[function_component(Counter)]
fn counter(props: &TodoProps) -> Html {
    let increment = {
        let state = props.state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    let decrement = {
        let state = props.state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    let reset = {
        let state = props.state.clone();
        Callback::from(move |_| state.set(0))
    };

    html! {
        <div class="uk-position-center uk-text-center">
            <button
                onclick={increment}
                class="uk-button uk-button-primary uk-button-large"
            >
                { "+1" }
            </button>
            <button
                onclick={reset}
                class="uk-button uk-button-primary uk-button-large"
            >
                { "0" }
            </button>
            <button
                onclick={decrement}
                class="uk-button uk-button-primary uk-button-large"
            >
                { "-1" }
        </button>
            <p>{ *props.state }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| 0 as u64);
    html! {
        <Counter {state} />
    }
}
fn main0() {
    yew::start_app::<App>();
}
*/

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<crate::app::DropPhoto>();
}
