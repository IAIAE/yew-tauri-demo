const invoke = window.__TAURI__.invoke
console.info('the invoke is ', invoke)
export async function invokeHello(name) {
    return invoke("hello", {name: name});
}

export async function getUser() {
    return invoke('getUser');
}
