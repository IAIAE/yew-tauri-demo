const invoke = window.__TAURI__.invoke
// console.info('the invoke is ', invoke)

export async function invokeHello(cb) {
    return new Promise((done)=>{
        setTimeout(()=>{
            done(cb())
        }, 2000)
    })
}

export async function getUser() {
    return invoke('getUser');
}

let yewCb = null
export function jsSetYewCb(cb){
    console.info('yew & js 通讯渠道建立成功')
    yewCb = cb
}

export function jsRemoveYewCb(){
    yewCb = null
}



setTimeout(()=>{
    console.info('after 10s')
    if(yewCb == null) {
        console.info('yewcb not ready, no calll')
    }else{
        console.info('yewcb ready, calling....')
        yewCb({
            type: 'hello',
            data: {
                name: "ricch",
                age: 123,
            }
        })
    }
}, 10000)
