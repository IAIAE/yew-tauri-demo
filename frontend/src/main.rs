use std::{sync::Arc};
use gloo::{
    console::{self, Timer},
    timers::callback::{Interval, Timeout},
};
use futures::{join, future::join, channel::mpsc::Sender};
use wasm_bindgen::{prelude::*, convert::OptionIntoWasmAbi, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Window};
use yew::prelude::*;

mod binding;
use binding::{model::{self as TauriModel, User}};

pub enum Msg {
    SetWelcom(String),
    SetUser(TauriModel::User),
    setErrTip(String),
    None,
}

pub struct Model {
    welcom: String,
    name: String,
    user: Option<TauriModel::User>,
    err_tip: Option<String>,
}

impl Model {
    fn renderUser(&self, user: &TauriModel::User) -> Html {
        let name = user.name.clone();
        let age = user.age.clone();
        let address = user.address.clone();
        html!(
            <>
            <div style="margin-top: 20px; padding: 20px; border: 1px solid #ddd; border-radius: 4px;">
                <span style="display: inline-block;">{"姓名"}</span>
                <span style="display: inline-block; margin-left: 10px;">{name}</span>
                <span style="display: inline-block; margin-left: 20px;">{"年龄"}</span>
                <span style="display: inline-block; margin-left: 10px;">{age}</span>
                {
                    if let Some(home) = address {
                        html!(<>
                            <span style="display: inline-block; margin-left: 20px;">{"地址"}</span>
                            <span style="display: inline-block; margin-left: 10px;">{home.desc}</span>
                        </>)
                    }else { html!()}
                }
            </div>
            </>
        )
    }
    fn renderErrtip(&self, msg: String) -> Html {
        html!(
            <div style="margin-top: 20px; padding: 20px; border: 1px solid #ddd; border-radius: 4px;">{format!("调用失败：{}", msg)}</div>
        )
    }
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            welcom: "".to_owned(),
            name: "world".to_owned(),
            user: Option::None,
            err_tip: Option::None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let message = self.welcom.clone();
        html!(<>
            <div>
                <h2 class="heading">{message}</h2>
                <div>{"下面的信息由yew调用js，然后js再调用tauri宿主，再原路返回而得，支持async。(我设置了一定概率会失败，可以右键reload一下)"}</div>
                {if let Some(user) = &self.user {
                    self.renderUser(&user)
                }else {html!()}}
                {if let Some(msg) = &self.err_tip {
                    let tip = msg.clone(); 
                    self.renderErrtip(tip)
                }else{html!()}}
            </div>
        </>)
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link();
            let name = self.name.clone();
            let sendWelcom = link.callback(Msg::SetWelcom);
            let sendUser = link.callback(Msg::SetUser);
            let sendErr = link.callback(Msg::setErrTip);
            spawn_local(async move {
                console::info!("rendered ====> inner invoke the tauri");
                let (res1, res2) = join!(
                    binding::getUser(), 
                    binding::hello(name)
                );
                match res1 {
                    Ok(user) => {
                        let t = user.into_serde::<TauriModel::User>().unwrap();
                        sendUser.emit(t);
                    },
                    Err(msg) => {
                        let err_tip = msg.as_string().unwrap();
                        sendErr.emit(err_tip); 
                    }
                }
                match res2 {
                    Ok(msg) => {
                        console::info!("ok ", &msg);
                        sendWelcom.emit(msg.as_string().unwrap());
                    },
                    Err(err) => {
                        console::info!("error ", err);
                    }
                }
                
            });
            console::info!("rendered ===> ");
        }

    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetWelcom(newtop) => {
                self.welcom = newtop;
                true
            },
            Msg::setErrTip(msg) => {
                self.err_tip = Some(msg);
                true
            },
            Msg::SetUser(user) => {
                self.user = Some(user);
                true
            }
            _=>{
                false
            }
        }
    }
    
}


fn main() {
    yew::start_app::<Model>();
}