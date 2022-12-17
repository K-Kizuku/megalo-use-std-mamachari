use yew_router::prelude::*;
use yew::prelude::*;
// use wasm_bindgen::prelude::wasm_bindgen;
// use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
// use wasm_bindgen;
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;
use reqwest;
// use gloo_net::http::Response::json;
use serde::{Deserialize,Serialize};
use chrono::{DateTime,Utc};
use serde_json::json;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Tsemp {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}

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
    #[at("/playlist")]
    PlayList,
    #[not_found]
    #[at("/404")]
    NotFound,
}

const URL:&str = "https://jsonplaceholder.typicode.com/todos/1";

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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SignInProps {
    email: String,
    password: String,
}

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let authorization: UseStateHandle<SignInProps> = use_state(|| SignInProps { email: String::from(""), password: String::from("") });
    // let e_mail = use_state(String::default);
    // let password = use_state(String::default);
    let navigator = use_navigator().unwrap();
    
    let onclick = Callback::from(move |_| navigator.push(&Route::SignUp));

    let authorization = authorization.clone();

    let button_post_auth_sign_in = {
        // let test = SignInProps { email: "".to_string(), password: "".to_string() };
        let authorization = authorization.clone();
        let onclick = Callback::from(move |e: MouseEvent| {
            let authorization = authorization.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // let authorization = authorization.clone();
                // let temp = &authorization;
                // let auth_json = serde_json::to_string(&authorization.serialize(serializer)).unwrap();
                // let post_data = json!(authorization);
                // Request::post("http://httpbin.org/post").json(&authorization).await;
                let post_data = SignInProps { email: String::from(authorization.email.clone()), password: String::from(authorization.password.clone()) };

                let client = reqwest::Client::new();
                let res = client.post("/api/signin")
                // .body(serde_json::to_string(&authorization))
                // .form(&authorization)
                // .json(&serde_json::to_string(&authorization).unwrap())
                .json(&post_data)
                .send()
                .await
                .unwrap()
                .text()
                .await;
            });
        });
            html!{
                <button {onclick}>{"サインイン"}</button>
            }
        };
    let input_email = {
        let authorization = authorization.clone();
        let onchange = Callback::from(move |e: Event| {

            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                authorization.set(SignInProps {email:input.value(), password:String::from(authorization.password.clone())});
            }
        });

        // Callback::from(move |e: Event| {
        //     let input = e.target_dyn_into::<HtmlInputElement>();

        //     if let Some(input) = input {
        //         authorization.set(SignInProps {email:input.value(), password:authorization.password});
        //     }
        // });
        html!{
            <input {onchange}/>
        }
    };
    let input_password = 
    {
        let authorization = authorization.clone();
        let onchange = Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
    
            if let Some(input) = input {
                authorization.set(SignInProps {email:String::from(authorization.email.clone()), password:input.value()});
            };
        }); 
    
    html!{
        <input {onchange}/>
    }
};


    html! {
        <div>
            <h1>{ "サインイン" }</h1>
            <div>{"Eメール"}</div>
            {input_email}
            <div>{"パスワード"}</div>
            {input_password}
            {button_post_auth_sign_in}

            <button {onclick}>{ "新規登録の方はこちら" }</button>
            
        </div>
    }
}


// extern "C" {
//     fn alert(s: &str);
// }

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SignUpProps {
    email: String,
    password: String,
    name: String,
}

#[function_component(SignUp)]
pub fn sign_up() -> Html {
    let authorization: UseStateHandle<SignUpProps> = use_state(|| SignUpProps { name: String::from(""),email: String::from(""), password: String::from("") });
    // let e_mail = use_state(String::default);
    // let password = use_state(String::default);
    let navigator = use_navigator().unwrap();
    
    let onclick = Callback::from(move |_| navigator.push(&Route::SignUp));

    let authorization = authorization.clone();

    let button_post_auth_sign_up = {
        // let test = SignInProps { email: "".to_string(), password: "".to_string() };
        let authorization = authorization.clone();
        let onclick = Callback::from(move |e: MouseEvent| {
            let authorization = authorization.clone();

            wasm_bindgen_futures::spawn_local(async move {
                // let authorization = authorization.clone();
                // let temp = &authorization;
                // let auth_json = serde_json::to_string(&authorization.serialize(serializer)).unwrap();
                // let post_data = json!(authorization);
                // Request::post("http://httpbin.org/post").json(&authorization).await;
                let post_data = SignUpProps { name: String::from(authorization.name.clone()), email: String::from(authorization.email.clone()), password: String::from(authorization.password.clone()) };

                let client = reqwest::Client::new();
                let res = client.post("/api/signup")
                // .body(serde_json::to_string(&authorization))
                // .form(&authorization)
                // .json(&serde_json::to_string(&authorization).unwrap())
                .json(&post_data)
                .send()
                .await
                .unwrap()
                .text()
                .await;
            });
        });
            html!{
                <button {onclick}>{"サインアップ"}</button>
            }
        };

    let input_name = {
        let authorization = authorization.clone();
        let onchange = Callback::from(move |e: Event| {

            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                authorization.set(SignUpProps {name: input.value(), email:String::from(authorization.email.clone()), password:String::from(authorization.password.clone())});
            }
        });

        html!{
            <input {onchange}/>
        }
    };
        
    let input_email = {
        let authorization = authorization.clone();
        let onchange = Callback::from(move |e: Event| {

            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                authorization.set(SignUpProps {name:String::from(authorization.name.clone()), email:input.value(), password:String::from(authorization.password.clone())});
            }
        });

        html!{
            <input {onchange}/>
        }
    };
    let input_password = 
    {
        let authorization = authorization.clone();
        let onchange = Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
    
            if let Some(input) = input {
                authorization.set(SignUpProps {name:String::from(authorization.name.clone()),email:String::from(authorization.email.clone()), password:input.value()});
            };
        }); 
    
    html!{
        <input {onchange}/>
    }
};


    html! {
        <div>
            <h1>{ "サインアップ" }</h1>
            <div>{"ユーザーネーム"}</div>
            {input_name}
            <div>{"Eメール"}</div>
            {input_email}
            <div>{"パスワード"}</div>
            {input_password}
            {button_post_auth_sign_up}

            <button {onclick}>{ "サインインはこちら" }</button>
            
        </div>
    }
}


#[function_component(Live)]
pub fn live() -> Html {
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

#[derive(Clone, PartialEq, Deserialize)]
struct PlayListProps {
        id : String,
        streamed_by: String,
        title : String,
        description : String,
        created_at : String,
        updated_at : String,
        is_streaming : bool,
}


#[function_component(PlayList)]
pub fn playlist() -> Html {
    let play_list = use_state(|| vec![]);
    let mut demo:Vec<PlayListProps> =  vec![];
    demo.push(PlayListProps {
        id:"1".to_string(),
        streamed_by: "".to_string(),
        title : "hoge".to_string(),
        description : "".to_string(),
        created_at : "".to_string(),
        updated_at : "".to_string(),
        is_streaming : false,
    });
    demo.push(PlayListProps {
        id:"2".to_string(),
        streamed_by: "".to_string(),
        title : "fuga".to_string(),
        description : "".to_string(),
        created_at : "".to_string(),
        updated_at : "".to_string(),
        is_streaming : false,
    });
    // PlayListPops {
    //     id:"",
    //     streamed_by: "",
    //     title : "",
    //     description : "",
    //     created_at : "",
    //     updated_at : "",
    //     is_streaming : false,
    // }
    {
        let play_list = play_list.clone();
        use_effect_with_deps(move |_| {
            let play_list = play_list.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos:Vec<PlayListProps> = Request::get(&URL)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    play_list.set(fetched_videos);
            });
            || ()
        }, ());
    }
    html!{
        <div>
        <h1>
        // {format!("{}", play_list[0].id)}
        </h1>
        <div>
        // {for x in demo{
        //     format!("{}", x.id)
        // }}
        { for demo.iter().map(|e| {
            html!{
                <div>
                    {e.id.to_string()}
                    {e.title.to_string()}
                </div>
            }
            
        }) }
        </div>
        // {test[]}
        </div>
    
    }
}

#[function_component(List)]
pub fn list()-> Html {

    let test = use_state(|| Tsemp {
        userId: 0,
        id: 1,
        title: "delectus aut autem".to_string(),
        completed: false
      });
    // #[wasm_bindgen]
    // let  res = reqwest::Client::new()
    //     .get("https://www.rust-lang.org")
    //     .header("Accept", "application/vnd.github.v3+json")
    //     .send()
    //     .await;

    // let text = res.expect("REASON").text().await;
    // let yamato = Callback::from(move |_: _| async { 
    //     // let text = text.clone();
    //     match text {
    //         Ok(n)  => test.set(&n),
    //         Err(e) => println!("Error: {}", e), 
    //     }
    //     // test.set(*text)
    // });
    // let run =|| async {
    //     let res = reqwest::Client::new()
    //         .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master")
    //         .header("Accept", "application/vnd.github.v3+json")
    //         .send()
    //         .await;
    
    //     let text = res.expect("REASON").text().await;
    //     // let branch_info: Branch = serde_json::from_str(&text).unwrap();
    //     // Callback::from(move |_: _| async {
    //         // let text = text.clone();
    //         match text {
    //             Ok(n)  => test.set(n),
    //             Err(e) => println!("Error: {}", e), 
    //         }
    //     // });
    //     // Ok(JsValue::from_serde(&branch_info).unwrap())
    // };
    // use_effect(move || {
        // let run =|| async {
        //     let res = reqwest::Client::new()
        //         .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master")
        //         .header("Accept", "application/vnd.github.v3+json")
        //         .send()
        //         .await;
        
        //     let text = res.expect("REASON").text().await;
        //     // let branch_info: Branch = serde_json::from_str(&text).unwrap();
        //     // Callback::from(move |_: _| async {
        //         // let text = text.clone();
        //         match text {
        //             Ok(n)  => test.set(n),
        //             Err(e) => println!("Error: {}", e), 
        //         }
        //     // });
        //     // Ok(JsValue::from_serde(&branch_info).unwrap())
        // };
        // let  codeed = async{
        //     run().await;
        //     html!{
        //         <div>{"a"}</div>
        //     }
        // };
    // });
    {
        let test = test.clone();
        use_effect_with_deps(move |_| {
            let test = test.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos = Request::get("https://jsonplaceholder.typicode.com/todos/1")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                test.set(fetched_videos);
            });
            || ()
        }, ());
    }

    
    // let body = reqwest::get("https://www.rust-lang.org").await?.text().await?;
html!{
    <div>
    <h1>
    {format!("{}", test.userId)}
    </h1>
    // {test[]}
    </div>

}

}

// #[function_component(Live)]
// pub fn live() -> Html{
//     fn test() {

//         #[wasm_bindgen(module="/src/test.js")]
//         extern "C"{
//             fn testjs();
//         }

//         #[wasm_bindgen]
//         pub fn temp (){
//             testjs()
//         }
//         temp();
//     }
//     let on_cautious_change = {
//         // let input_value_handle = input_value_handle.clone();

//         Callback::from(move |e: MouseEvent| {
//             test();
//         })
//     };
//     // use_effect(move || {
//     //     // Make a call to DOM API after component is rendered
//     //     gloo::utils::document().set_title(&format!("You clicked {} times", *counter_one));

//     //     // Perform the cleanup
//     //     || gloo::utils::document().set_title(&format!("You clicked 0 times"))
//     // });
//     html! {
//         <div>
//             <input />
//             <h1>{"Live" }</h1>
//             <video controls={true} id="test"></video>
//             // <button {onclick}>{ "Go Home" }</button>
//             <button onclick={on_cautious_change}>{"click"}</button>
//             <p id="kaka">{"test"}</p>
//         </div>
//     }
// }
// #[derive(Clone, PartialEq)]
// struct Video {
//     id: usize,
//     title: String,
//     speaker: String,
//     url: String,
// }
#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>
}
#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };
        html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}

#[function_component(Test)]
fn app() -> Html {
   let videos = use_state(|| vec![]);
   {
       let videos = videos.clone();
       use_effect_with_deps(move |_| {
           let videos = videos.clone();
           wasm_bindgen_futures::spawn_local(async move {
               let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
                   .send()
                   .await
                   .unwrap()
                   .json()
                   .await
                   .unwrap();
               videos.set(fetched_videos);
           });
           || ()
       }, ());
   }
   let selected_video = use_state(|| None);
       let on_video_select = {
            let selected_video = selected_video.clone();
            Callback::from(move |video: Video| {
                selected_video.set(Some(video))
            })
        };
    
        // let details = selected_video.as_ref().map(|video| html! {
        //     <VideoDetails video={video.clone()} />
        // });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            // { for details }
        </>
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
            <Live />
        },
        Route::PlayList => html! {
            <PlayList />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(CustomHeader)]
pub fn custom_header() -> Html {
    html!{
        <div>{"ヘッダーだよ"}</div>
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <CustomHeader />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </>
    }
}