const invoke = window.__TAURI__.invoke
// console.info('__TAURI_IPC__ ', window.__TAURI_IPC__,)
// console.info('__TAURI_METADATA__ ', window.__TAURI_METADATA__,)
// console.info('__TAURI__ ', window.__TAURI__,)

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


