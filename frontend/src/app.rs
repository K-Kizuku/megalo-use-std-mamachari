use yew_router::prelude::*;
use yew::prelude::*;
// use wasm_bindgen::prelude::wasm_bindgen;
// use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
// use wasm_bindgen;
use wasm_bindgen::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/sign_up")]
    SignUp,
    #[at("/sign_in")]
    SignIn,
    #[at("/live")]
    Live,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let link_sign_in_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::SignIn));
        html! {
            <button {onclick}>{"SignIn"}</button>
        }
    };
    let link_sign_up_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::SignUp));
        html! {
            <button {onclick}>{"SignUp"}</button>
        }
    };
    // let link_sign_in = Callback::from(move |_| navigator.push(&Route::SignIn));
    // let link_sign_up = Callback::from(move |_| navigator.push(&Route::SignUp));
    html! {
        <div>
            {link_sign_in_button}
            {link_sign_up_button}
        </div>
    }

}

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::SignUp));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "新規登録の方はこちら" }</button>
            
        </div>
    }
}

// extern "C" {
//     fn alert(s: &str);
// }
#[function_component(SignUp)]
pub fn sign_up() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();
    fn test() {

        #[wasm_bindgen(module="/src/test.js")]
        extern "C"{
            fn testjs();
        }

        #[wasm_bindgen]
        pub fn temp (){
            testjs()
        }
        temp();
    }
    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: MouseEvent| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            test();
            // let target: Option<EventTarget> = e.target();
            // // Events can bubble so this listener might catch events from child
            // // elements which are not of type HtmlInputElement
            // let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            // if let Some(input) = input {
            //     input_value_handle.set(input.value());
            // }
        })
    };


    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <input />
            <h1>{"Secure" }</h1>
            <video controls={true} id="test"></video>
            <button {onclick}>{ "Go Home" }</button>
            <button onclick={on_cautious_change}>{"click"}</button>
            // <input onchange={on_cautious_change}
            //         id="cautious-input"
            //         type="text"
            //         value={input_value.clone()}
            //     />
            <p id="kaka">{"test"}</p>
        </div>
    }
}



fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home /> 
        },
        Route::Secure => html! {
            <Secure />
        },
        Route::SignIn => html! {
            <SignIn />
        },
        Route::SignUp => html! {
            <SignUp />
        },
        Route::Live => html!{
            <SignIn />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}