Issue using glutin and serde in emscripten:

when running on the web:
```
Successfully compiled asm.js code (total compilation time 2994ms; storage initialization failed (consider filing a bug); 6 functions compiled slowly: ql:8:8 (433ms), Rf:12:38706 (391ms), Fg:13:8 (409ms), hl:15:8 (661ms), Ci:9:8 (2079ms), pk:11:8 (2472ms)) spring-crabs.js
emscripten_webgl_create_context failed: explicitSwapControl is not supported, please rebuild with -s OFFSCREENCANVAS_SUPPORT=1 to enable targeting the experimental OffscreenCanvas specification! spring-crabs.js:1:471520

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: BackendCreationError(OsError("Error while calling emscripten_webgl_create_context: Internal error in the library (success detected as failure)"))', /checkout/src/libcore/result.rs:859 index.htm:141:13

note: Run with `RUST_BACKTRACE=1` for a backtrace. index.htm:141:13
```

if you delete CFG.window.fullscreen static ref call in the main then it works.

# versions

rust: `1.7` and `1.18.0-beta.1`

emsdk:
```
$ ~/emsdk_portable/emsdk list

The following precompiled tool packages are available for download:
           clang-nightly-e1.37.9-2017_03_30_17_42-64bit
           clang-nightly-e1.37.9-2017_03_30_19_17-64bit
           clang-e1.37.1-64bit      
           clang-e1.37.8-64bit      
     *     clang-e1.37.9-64bit      	INSTALLED
           node-4.1.1-32bit         
     *     node-4.1.1-64bit         	INSTALLED
           emscripten-1.30.0        
           emscripten-1.34.1        
           emscripten-1.35.0        
           emscripten-1.37.1        
           emscripten-1.37.8        
     *     emscripten-1.37.9        	INSTALLED
           emscripten-nightly-1.37.9-2017_03_30_17_42
           emscripten-nightly-1.37.9-2017_03_30_19_17

The following tools can be compiled from source:
           clang-tag-e1.37.8-32bit  
           clang-tag-e1.37.9-32bit  
           clang-tag-e1.37.8-64bit  
           clang-tag-e1.37.9-64bit  
           clang-incoming-32bit     
           clang-incoming-64bit     	INSTALLED
           clang-master-32bit       
           clang-master-64bit       	INSTALLED
           emscripten-tag-1.37.8-32bit
           emscripten-tag-1.37.9-32bit
           emscripten-tag-1.37.8-64bit
           emscripten-tag-1.37.9-64bit
           binaryen-tag-1.37.8-32bit
           binaryen-tag-1.37.9-32bit
           binaryen-tag-1.37.8-64bit
           binaryen-tag-1.37.9-64bit
           emscripten-incoming-32bit
           emscripten-master-32bit  
           emscripten-incoming-64bit	INSTALLED
           emscripten-master-64bit  
           binaryen-master-32bit    
           binaryen-master-64bit    

The following precompiled SDKs are available for download: (Run "./emsdk update" to pull in the latest list)
         sdk-nightly-1.37.9-2017_03_30_17_42-64bit
         sdk-nightly-1.37.9-2017_03_30_19_17-64bit
         sdk-1.37.8-64bit         
    *    sdk-1.37.9-64bit         	INSTALLED

The following SDKs can be compiled from source:
         sdk-incoming-32bit       
         sdk-incoming-64bit       	INSTALLED
         sdk-master-32bit         
         sdk-master-64bit         
         sdk-tag-1.37.8-32bit     
         sdk-tag-1.37.9-32bit     
         sdk-tag-1.37.8-64bit     
         sdk-tag-1.37.9-64bit     

Items marked with * are activated for the current user.

To access the historical archived versions, type 'emsdk list --old'
```
