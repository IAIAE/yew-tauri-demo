# this

## 背景
由于使用tauri，再js层面需要使用`@tauri-apps/api`才能保证tauri能够和js通信（进而才能使得tauri透过js和yew通信）。

而对于yew引用的js snippet来说，不允许import，只能是一个简单js文件。所以，如果和yew通信的js文件中希望引用第三方库`import xxx from 'xxxxx'`，目前是不支持的。

所以作为tauri和yew中间桥梁的js文件，我们必须把相关的代码打包成一个单一的、没有外部引用的`tauri.glue.bundle.js`才行，这就需要我们自己动手，将`@tauri-apps/api`的源码搞下来自己编译打包。

## 方法

由于目前的打包工具，都是依赖amd、es-module、commonjs之类的模块引入规范的。会依赖环境的require\define等等变量。wasm-binggen本身也没有提供。所以没办法自动的用打包器打包，只能手动的合并代码。好在代码都是一些接口调用，并不用复杂，工作量并不多。

手工合并在了`tauri.glue.bundle.ts`中，然后再用typescript编译一次。生成`.js`文件。

注意ts的target设置`es2017`以上即可，因为wasm-bindgen原生支持`async/await`。

懂我的意思ma?
