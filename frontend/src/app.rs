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
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

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
    // #[at("/secure")]
    // Secure,
    #[at("/sign_up")]
    SignUp,
    #[at("/sign_in")]
    SignIn,
    #[at("/stream_view/:id")]
    LiveNum {id:String},
    #[at("/playlist")]
    PlayList,
    #[at("/stream_info")]
    LiveInfo,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// const URL:&str = "https://jsonplaceholder.typicode.com/todos/1";



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
        <div class="home">
            {link_sign_in_button}
            {link_sign_up_button}
        </div>
    }

}

// #[function_component(Secure)]
// pub fn secure() -> Html {
//     let navigator = use_navigator().unwrap();

//     let onclick = Callback::from(move |_| navigator.push(&Route::Home));
//     html! {
//         <div>
//             <h1>{ "Secure" }</h1>
//             <button {onclick}>{ "Go Home" }</button>
//         </div>
//     }
// }

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
                let res = client.post("http://megalo.pigeons.house/api/signin")
                // .body(serde_json::to_string(&authorization))
                // .form(&authorization)
                // .json(&serde_json::to_string(&authorization).unwrap())
                .json(&post_data)
                .send()
                .await
                .unwrap()
                .text()
                .await;
                LocalStorage::set("jwt", res.ok());
                // match res {
                //     Ok(r) => LocalStorage::set("yew", r.text().await.unwrap()).ok(),
                //     Err(err) => (),
                // }
            });
        });
            html!{
                <button class="primary-button" {onclick}>{"サインイン"}</button>
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
        <div class="signin">
            <h1>{ "サインイン" }</h1>
            <div>
            <p>{"Eメール"}</p>
            {input_email}
            </div>
            <div>
            <p>{"パスワード"}</p>
            {input_password}
            </div>
            {button_post_auth_sign_in}

            <button class="secondary-button" {onclick}>{ "新規登録の方はこちら" }</button>
            
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
                let res = client.post("http://megalo.pigeons.house/api/signup")
                // .body(serde_json::to_string(&authorization))
                // .form(&authorization)
                // .json(&serde_json::to_string(&authorization).unwrap())
                .json(&post_data)
                .send()
                .await
                .unwrap()
                .text()
                .await;
                LocalStorage::set("jwt", res.ok());

            });
        });
            html!{
                <button class="primary-button" {onclick}>{"サインアップ"}</button>
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
        <div class="signup">
            <h1>{ "サインアップ" }</h1>
            <div>
              <p>{"ユーザーネーム"}</p>
              {input_name}
            </div>
            <div>
              <p>{"Eメール"}</p>
              {input_email}
            </div>
            <div>
              <p>{"パスワード"}</p>
              {input_password}
            </div>
            {button_post_auth_sign_up}

            <button class="secondary-button" {onclick}>{ "サインインはこちら" }</button>
            
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
    use_effect_with_deps(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            test();
        });
        || ()
    }, ());


    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::PlayList));

    html! {
        <div class="stream-view">
            <div class="video-zone">
                <button class="primary-button" {onclick}>{ "一覧表示へ" }</button>
                <video controls={true} id="video"></video>
                <h1>{"配信やってみた✌"}</h1>
                <div class="user-info">
                    <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                    <span>{"User1"}</span>
                </div>
            </div>
            <div class="comment-zone">
                <p class="comment-column-title">{"コメント"}</p>
                <div class="comment-list">
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                </div>
                <textarea placeholder="コメント"> </textarea>
                <button>{"送信"}</button>
            </div>
        </div>
    }
}








#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: String,
}
#[function_component(LiveNum)]
pub fn live_num(props:&Props) -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();
    let user_id = props.id.clone();
    let testop = || {
        #[wasm_bindgen(module="/src/test.js")]
        extern "C"{
            fn testjs();
        }

        #[wasm_bindgen]
        pub fn temp00 (user_id:String){
            testjs();
        }
        temp00(user_id); 
    };
    // fn test() {

    //     #[wasm_bindgen(module="/src/test.js")]
    //     extern "C"{
    //         fn testjs(user_id:String);
    //     }

    //     #[wasm_bindgen]
    //     pub fn temp00 (user_id:&str){
    //         testjs(user_id.to_string());
    //     }
    //     temp00(user_id);
    // }
    use_effect_with_deps(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            testop();
        });
        || ()
    }, ());


    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::PlayList));

    html! {
        <div class="stream-view">
            <div class="video-zone">
                <button class="primary-button" {onclick}>{ "一覧表示へ" }</button>
                <video controls={true} id="video"></video>
                <h1>{"配信やってみた✌"}</h1>
                <div class="user-info">
                    <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                    <span>{"User1"}</span>
                </div>
            </div>
            <div class="comment-zone">
                <p class="comment-column-title">{"コメント"}</p>
                <div class="comment-list">
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                    <div class="comment-component">
                        <div class="comment-user-info">
                            <img src="https://1.bp.blogspot.com/-GqIjU--SM-k/X9lJl-pkCjI/AAAAAAABc68/hEhMB_uG-xEPzhgaRjBhgX24-niyVZUnwCNcBGAsYHQ/s637/pose_reiwa_woman.png" alt="user1"/>
                            <span>{"User1"}</span>
                        </div>
                        <p>{"面白いですね！www"}</p>
                    </div>
                </div>
                <textarea placeholder="コメント"> </textarea>
                <button>{"送信"}</button>
            </div>
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
    let navigator = use_navigator().unwrap();
    let play_list = use_state(|| vec![]);
    let mut demo:Vec<PlayListProps> =  vec![];
    demo.push(PlayListProps {
        id:"1".to_string(),
        streamed_by: "t3mp".to_string(),
        title : "半導体作ってみた！".to_string(),
        description : "半導体不足で大変ですよね．そんな時は自作半導体！みなさ…".to_string(),
        created_at : "".to_string(),
        updated_at : "".to_string(),
        is_streaming : false,
    });
    demo.push(PlayListProps {
        id:"2".to_string(),
        streamed_by: "鳩屋敷".to_string(),
        title : "配信やってみた✌".to_string(),
        description : "えと…あの…そ，その……はじまm，はじめまして．鳩屋敷で…".to_string(),
        created_at : "".to_string(),
        updated_at : "".to_string(),
        is_streaming : true,
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
                let fetched_videos:Vec<PlayListProps> = Request::get("http://megalo.pigeons.house/api")
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
        <h1 class="live-list-title">
        // {format!("{}", play_list[0].id)}
        {"今熱いライブ"}
        </h1>
        <div>
        // {for x in demo{
        //     format!("{}", x.id)
        // }}
        { for demo.into_iter().map(|e| {
            let navigator = navigator.clone();
            let onclick = Callback::from(move |eve:MouseEvent| navigator.push(&Route::LiveNum{id:e.streamed_by.to_string()}));

            html!{
                <div {onclick} class="stream-card">
                    <div class="upper-card">
                        <span class="stream-title">{e.title.to_string()}</span>
                        // <span class="streamer-name">{e.streamed_by.to_string()}</span>
                    </div>
                    <div class="lower-card">
                        <span>
                            {e.description.to_string()}
                        </span>
                    </div>
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

// #[function_component(Test)]
// fn app() -> Html {
//    let videos = use_state(|| vec![]);
//    {
//        let videos = videos.clone();
//        use_effect_with_deps(move |_| {
//            let videos = videos.clone();
//            wasm_bindgen_futures::spawn_local(async move {
//                let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
//                    .send()
//                    .await
//                    .unwrap()
//                    .json()
//                    .await
//                    .unwrap();
//                videos.set(fetched_videos);
//            });
//            || ()
//        }, ());
//    }
//    let selected_video = use_state(|| None);
//        let on_video_select = {
//             let selected_video = selected_video.clone();
//             Callback::from(move |video: Video| {
//                 selected_video.set(Some(video))
//             })
//         };
    
//         // let details = selected_video.as_ref().map(|video| html! {
//         //     <VideoDetails video={video.clone()} />
//         // });

//     html! {
//         <>
//             <h1>{ "RustConf Explorer" }</h1>
//             <div>
//                 <h3>{"Videos to watch"}</h3>
//                 <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
//             </div>
//             // { for details }
//         </>
//     }
// }


#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct LiveInfoProps {
    title: String,
    description: String,
}

#[function_component(LiveInfo)]
fn live_info()-> Html{
    let live_info = use_state(|| LiveInfoProps{title: String::from(""), description: String::from("")});
    let input_title = 
    {
        let live_info = live_info.clone();
        let onchange = Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
    
            if let Some(input) = input {
                live_info.set(LiveInfoProps { title: input.value(), description: String::from(live_info.description.clone())});
            };
        }); 
        html!{
            <input {onchange}/>
        }
    };
    let input_description = 
    {
        let live_info = live_info.clone();
        let onchange = Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
    
            if let Some(input) = input {
                live_info.set(LiveInfoProps { title: String::from(live_info.description.clone()), description:input.value() });
            };
        }); 
        html!{
            <input {onchange}/>
        }
    };

    let button_post_live_info = {
        let onclick = Callback::from(move |e: MouseEvent| {
            let live_info = live_info.clone();
    
            wasm_bindgen_futures::spawn_local(async move {
                let post_data = LiveInfoProps {title: String::from(live_info.title.clone()), description: String::from(live_info.description.clone())};
    
                let client = reqwest::Client::new();
                let res = client.post("/api/streams")
                .json(&post_data)
                .send()
                .await
                .unwrap()
                .text()
                .await;
            });
        });
            html!{
                <button {onclick}>{"ライブ開始！"}</button>
            }
    };

    html!{
        <div class="live-info">
            <h1>{"ライブを始める"}</h1>
            <div>{"タイトル"}</div>
            {input_title}
            <div>{"説明"}</div>
            {input_description}
            <div>
                {button_post_live_info}
            </div>
        </div>
    }
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home /> 
        },
        // Route::Secure => html! {
        //     <Secure />
        // },
        Route::SignIn => html! {
            <SignIn />
        },
        Route::SignUp => html! {
            <SignUp />
        },
        Route::LiveNum{id} => html!{
            <LiveNum id={id} />
        },
        Route::PlayList => html! {
            <PlayList />
        },
        Route::LiveInfo => html! {
            <LiveInfo />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(CustomHeader)]
pub fn custom_header() -> Html {
    html!{
        <div class="header">{"PieceLive"}</div>
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