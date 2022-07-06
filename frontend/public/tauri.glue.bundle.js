// tauri.ts
/** @ignore */
function uid() {
    return window.crypto.getRandomValues(new Uint32Array(1))[0];
}
/**
 * Transforms a callback function to a string identifier that can be passed to the backend.
 * The backend uses the identifier to `eval()` the callback.
 *
 * @return A unique identifier associated with the callback function.
 */
function transformCallback(callback, once = false) {
    const identifier = uid();
    const prop = `_${identifier}`;
    Object.defineProperty(window, prop, {
        value: (result) => {
            if (once) {
                Reflect.deleteProperty(window, prop);
            }
            return callback === null || callback === void 0 ? void 0 : callback(result);
        },
        writable: false,
        configurable: true
    });
    return identifier;
}
/**
 * Sends a message to the backend.
 * @example
 * ```typescript
 * import { invoke } from '@tauri-apps/api/tauri';
 * await invoke('login', { user: 'tauri', password: 'poiwe3h4r5ip3yrhtew9ty' });
 * ```
 *
 * @param cmd The command name.
 * @param args The optional arguments to pass to the command.
 * @return A promise resolving or rejecting to the backend response.
 */
async function invoke(cmd, args = {}) {
    return new Promise((resolve, reject) => {
        const callback = transformCallback((e) => {
            resolve(e);
            Reflect.deleteProperty(window, `_${error}`);
        }, true);
        const error = transformCallback((e) => {
            reject(e);
            Reflect.deleteProperty(window, `_${callback}`);
        }, true);
        window.__TAURI_IPC__(Object.assign({ cmd,
            callback,
            error }, args));
    });
}
/**
 * Convert a device file path to an URL that can be loaded by the webview.
 * Note that `asset:` and `https://asset.localhost` must be allowed on the `csp` value configured on `tauri.conf.json > tauri > security`.
 * Example CSP value: `"csp": "default-src 'self'; img-src 'self' asset: https://asset.localhost"` to use the asset protocol on image sources.
 *
 * Additionally, the `asset` must be allowlisted under `tauri.conf.json > tauri > allowlist > protocol`,
 * and its access scope must be defined on the `assetScope` array on the same `protocol` object.
 *
 * @param  filePath The file path.
 * @param  protocol The protocol to use. Defaults to `asset`. You only need to set this when using a custom protocol.
 * @example
 * ```typescript
 * import { appDir, join } from '@tauri-apps/api/path';
 * import { convertFileSrc } from '@tauri-apps/api/tauri';
 * const appDirPath = await appDir();
 * const filePath = await join(appDir, 'assets/video.mp4');
 * const assetUrl = convertFileSrc(filePath);
 *
 * const video = document.getElementById('my-video');
 * const source = document.createElement('source');
 * source.type = 'video/mp4';
 * source.src = assetUrl;
 * video.appendChild(source);
 * video.load();
 * ```
 *
 * @return the URL that can be used as source on the webview.
 */
function convertFileSrc(filePath, protocol = 'asset') {
    const path = encodeURIComponent(filePath);
    return navigator.userAgent.includes('Windows')
        ? `https://${protocol}.localhost/${path}`
        : `${protocol}://${path}`;
}
async function invokeTauriCommand(command) {
    return invoke('tauri', command);
}
/**
 * Unregister the event listener associated with the given name and id.
 *
 * @ignore
 * @param event The event name
 * @param eventId Event identifier
 * @returns
 */
async function _unlisten(event, eventId) {
    return invokeTauriCommand({
        __tauriModule: 'Event',
        message: {
            cmd: 'unlisten',
            event,
            eventId
        }
    });
}
/**
 * Emits an event to the backend.
 *
 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
 * @param [windowLabel] The label of the window to which the event is sent, if null/undefined the event will be sent to all windows
 * @param [payload] Event payload
 * @returns
 */
async function eventEmit(event, windowLabel, payload) {
    await invokeTauriCommand({
        __tauriModule: 'Event',
        message: {
            cmd: 'emit',
            event,
            windowLabel,
            payload: typeof payload === 'string' ? payload : JSON.stringify(payload)
        }
    });
}
/**
 * Listen to an event from the backend.
 *
 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
 * @param handler Event handler callback.
 * @return A promise resolving to a function to unlisten to the event.
 */
async function eventListen(event, windowLabel, handler) {
    return invokeTauriCommand({
        __tauriModule: 'Event',
        message: {
            cmd: 'listen',
            event,
            windowLabel,
            handler: transformCallback(handler)
        }
    }).then((eventId) => {
        return async () => _unlisten(event, eventId);
    });
}
/**
 * Listen to an one-off event from the backend.
 *
 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
 * @param handler Event handler callback.
 * @returns A promise resolving to a function to unlisten to the event.
 */
async function eventOnce(event, windowLabel, handler) {
    return eventListen(event, windowLabel, (eventData) => {
        handler(eventData);
        _unlisten(event, eventData.id).catch(() => { });
    });
}
const eventApi = { emit: eventEmit, listen: eventListen, once: eventOnce };
// event.ts
/**
 * Listen to an event from the backend.
 * @example Listen to the `error` event expecting a string payload
 * ```typescript
 * import { listen } from '@tauri-apps/api/event';
 * const unlisten = await listen<string>('error', (event) => {
 *   console.log(`Got error in window ${event.windowLabel}, payload: ${payload}`);
 * });
 *
 * // removes the listener later
 * await unlisten();
 * ```
 *
 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
 * @param handler Event handler callback.
 * @return A promise resolving to a function to unlisten to the event.
 */
async function listen(event, handler) {
    return eventApi.listen(event, null, handler);
}
/**
 * Listen to an one-off event from the backend.
 * @example Listen to the `loaded` event that is only triggered once
 * ```typescript
 * import { once } from '@tauri-apps/api/event';
 * interface LoadedPayload {
 *   loggedIn: boolean,
 *   token: string
 * }
 * const unlisten = await once<LoadedPayload>('loaded', (event) => {
 *   console.log(`App is loaded, logggedIn: ${event.payload.loggedIn}, token: ${event.payload.token}`);
 * });
 * ```
 *
 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
 * @param handler Event handler callback.
 * @returns A promise resolving to a function to unlisten to the event.
 */
async function once(event, handler) {
    return eventApi.once(event, null, handler);
}
/**
 * Emits an event to the backend.
 * @example Emits the `frontend-loaded` event with the given payload
 * ```typescript
 * import { emit } from '@tauri-apps/api/event';
 * await emit('frontend-loaded', { loggedIn: true, token: 'authToken' });
 * ```
 *
 * @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
 * @param [payload] Event payload
 * @returns
 */
async function emit(event, payload) {
    return eventApi.emit(event, undefined, payload);
}
// export { listen, once, emit }
let yewCb = null;
let unlistenTauri = null;
export async function jsSetYewCb(cb) {
    try {
        const unlisten = await listen('tauri-event', event => {
            // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
            // event.payload is the payload object
            // console.info('tauri-event ', event)
            if (yewCb == null)
                return;
            yewCb(event.payload);
        });
        unlistenTauri = unlisten;
        yewCb = cb;
        console.info('tauri -> js -> yew 通讯渠道建立成功');
    }
    catch (e) {
        console.info('tauri -> js 通讯渠道建立失败', e);
    }
}
export function jsRemoveYewCb() {
    yewCb = null;
    unlistenTauri && unlistenTauri();
}
// 测试
// setTimeout(()=>{
//     console.info('after 10s')
//     if(yewCb == null) {
//         console.info('yewcb not ready, no calll')
//     }else{
//         console.info('yewcb ready, calling....')
//         yewCb({
//             type: 'hello',
//             data: {
//                 name: "ricch",
//                 age: 123,
//             }
//         })
//     }
// }, 10000)
// 这个js运行时，我们hold一个对tauri的全局listen，再通过yewCb传递给yew
