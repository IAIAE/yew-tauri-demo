use std::{sync::Arc, borrow::{Borrow, BorrowMut}};
use gloo::{
    console::{self, Timer, console},
    timers::callback::{Interval, Timeout},
};
use futures::{join, future::join, channel::mpsc::Sender};
use wasm_bindgen::{prelude::*, convert::OptionIntoWasmAbi, JsCast, closure::Closure};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Window};
use yew::prelude::*;

mod binding;
use binding::{model::{self as TauriModel, User}};

pub enum Msg {
    SetWelcom(String),
    SetUser(TauriModel::User),
    SetErrTip(String),
    AddCount,
    JSEvt(JsValue),
    None,
}

pub struct Model {
    welcom: String,
    name: String,
    user: Option<TauriModel::User>,
    err_tip: Option<String>,
    count: i32,
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
    
    fn getDocument(&self) -> web_sys::Document {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        document
    }

    fn renderUploadInput(&self, ctx: &Context<Self>) -> Html {
        let document = self.getDocument();
        let cb = ctx.link().callback(move |e:InputEvent| {
            let dom = document.get_element_by_id("fileinput").expect("no fileinput dom");
            console::info!("e is ", &dom);
            let t = dom.get_attribute("files");
            Msg::None
        });
        html!(
            <div style="margin-top: 20px;">
                <input type="file" id="fileinput" oninput={cb} />
            </div>
        )
    }

    /**
     * 在js中注入一个和yew通信的回调
     */
    fn jsSetYewCb(&self, ctx: &Context<Self>) {
        let linkCallback = ctx.link().callback(|e|  Msg::JSEvt(e));
        let yewcb = Closure::<dyn Fn(JsValue)>::new(move |evt:JsValue| {
            linkCallback.emit(evt);
        }).into_js_value();  // 用into_js_value，将回调函数的所有权交给js的gc。这个方法好棒！
        binding::jsSetYewCb(yewcb);
    }

    /**
     * 撤销js中注入的回调
     */
    fn jsRemoveYewCb(&self) {
        binding::jsRemoveYewCb();
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
            count: 0,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let message = self.welcom.clone();
        html!(<>
            <div>
                <h2 class="heading">{message}</h2>
                <div>{"下面的信息由yew -> js -> tauri调用获得，支持async。(我设置了一定概率会失败，可以右键reload一下)"}</div>
                {if let Some(user) = &self.user {
                    self.renderUser(&user)
                }else {html!()}}
                {if let Some(msg) = &self.err_tip {
                    let tip = msg.clone(); 
                    self.renderErrtip(tip)
                }else{html!()}}
                <div style="margin-top: 20px;"><button class="btn" onclick={ctx.link().callback(|_| Msg::AddCount)}>{"+1"}</button><span style="display: inline-block; vertical-align: middle; margin-left: 20px;">{self.count}</span></div>
                <div style="margin-top: 20px;">{"而从tauri -> js -> yew 的调用过程，点击顶部菜单栏编辑/复制，即可在console中看到yew的响应。"}</div>
                {self.renderUploadInput(ctx)}
            </div>
        </>)
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        self.jsRemoveYewCb();
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // 首先设置js的回调通道。将closure的所有权交给js，意味着，这些回调将在整个页面的生命周期中存在
            self.jsSetYewCb(ctx);

            let link = ctx.link();
            let name = self.name.clone();
            let sendWelcom = link.callback(Msg::SetWelcom);
            let sendUser = link.callback(Msg::SetUser);
            let sendErr = link.callback(Msg::SetErrTip);
            
            spawn_local(async move {
                let (res1, res2) = join!(
                    binding::getUser(), 
                    binding::hello(Closure::once_into_js(|| "Hello Yew and Tauri!".to_string()))
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
                        // console::info!("ok ", &msg);
                        sendWelcom.emit(msg.as_string().unwrap());
                    },
                    Err(err) => {
                        console::info!("error ", err);
                    }
                }
                
            });
        }

    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetWelcom(newtop) => {
                self.welcom = newtop;
                true
            },
            Msg::SetErrTip(msg) => {
                self.err_tip = Some(msg);
                true
            },
            Msg::SetUser(user) => {
                self.user = Some(user);
                true
            },
            Msg::AddCount => {
                self.count += 1;
                true
            },
            Msg::JSEvt(evt) => {
                console::info!("tauri event is ", &evt);
                if !evt.is_instance_of::<js_sys::Object>() {
                    return false    
                }
                false
            },
            _=>{ false }
        }
    }
    
}


fn main() {
    yew::start_app::<Model>();
}