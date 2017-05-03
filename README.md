Issue using glutin and serde in emscripten:

when running on the web:
```
Successfully compiled asm.js code (total compilation time 2994ms; storage initialization failed (consider filing a bug); 6 functions compiled slowly: ql:8:8 (433ms), Rf:12:38706 (391ms), Fg:13:8 (409ms), hl:15:8 (661ms), Ci:9:8 (2079ms), pk:11:8 (2472ms)) spring-crabs.js
emscripten_webgl_create_context failed: explicitSwapControl is not supported, please rebuild with -s OFFSCREENCANVAS_SUPPORT=1 to enable targeting the experimental OffscreenCanvas specification! spring-crabs.js:1:471520

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: BackendCreationError(OsError("Error while calling emscripten_webgl_create_context: Internal error in the library (success detected as failure)"))', /checkout/src/libcore/result.rs:859 index.htm:141:13

note: Run with `RUST_BACKTRACE=1` for a backtrace. index.htm:141:13
```

if you delete CFG.window.fullscreen static ref call in the main then it works.
