Some short test code to demonstrate a bug using the `argmin` crate
in web assembly.


## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

You will likely get a very long error which looks something like this:

    Set timeout to 20 seconds...
    Running headless tests in Firefox on `http://127.0.0.1:33429/`
    Try find `webdriver.json` for configure browser's capabilities:
    Not found
    running 2 tests                                   

    test web::solve_test ... FAIL
    test web::pass ... ok

    failures:

    ---- web::solve_test output ----
        error output:
            panicked at 'time not implemented on this platform', library/std/src/sys/wasm/../unsupported/time.rs:13:9
            
            Stack:
            
            init/imports.wbg.__wbg_new_59cb74e423758ede<@http://127.0.0.1:38345/wasm-bindgen-test:591:19
            logError/<@http://127.0.0.1:38345/wasm-bindgen-test:154:22
            console_error_panic_hook::Error::new::hbdb0f39a7e26dfae@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[2833]:0xb2f5c
            console_error_panic_hook::hook_impl::h9694381c72587073@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[357]:0x5f40d
            console_error_panic_hook::hook::h3add71af1744884d@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3187]:0xb7cd4
            core::ops::function::Fn::call::h9516aefea7143c24@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[2628]:0xafcea
            std::panicking::rust_panic_with_hook::h123718ba3bf480af@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[695]:0x7806d
            std::panicking::begin_panic::{{closure}}::h80296a1cde52d2c6@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3488]:0xbb4a9
            std::sys_common::backtrace::__rust_end_short_backtrace::hd00e0322330ed085@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3360]:0xb9ed5
            std::panicking::begin_panic::h22f3dacc7e51aec9@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3486]:0xbb44a
            std::sys::wasm::time::Instant::now::h70764a6b710b7aac@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3869]:0xbe090
            std::time::Instant::now::h803a89b9ec1488b3@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3932]:0xbe3a0
            argmin::core::executor::Executor<O,S>::run::h174702a4f90445b1@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[46]:0x18012
            argmin_wasm_test::solve::h3acb1c8ed83f8f50@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[200]:0x4baad
            web::solve_test::hdad69cdf6b4c095d@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3929]:0xbe38a
            core::ops::function::FnOnce::call_once::h5559e00dea6aa5d5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3334]:0xb99ea
            wasm_bindgen_test::__rt::Context::execute_sync::{{closure}}::ha30c895b595ed339@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[966]:0x857f8
            <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h25403166e79259fe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[909]:0x82ead
            <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::{{closure}}::{{closure}}::h3cce7b7045ac5ea1@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1239]:0x8fe3b
            wasm_bindgen::convert::closures::invoke0_mut::hcd3c73d781d702c5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1011]:0x876c1
            __wbg_adapter_32@http://127.0.0.1:38345/wasm-bindgen-test:278:10
            cb0@http://127.0.0.1:38345/wasm-bindgen-test:430:28
            window.__wbg_test_invoke@http://127.0.0.1:38345/:37:38
            init/imports.wbg.__wbg_wbgtestinvoke_03a45ab326341b1b<@http://127.0.0.1:38345/wasm-bindgen-test:435:30
            handleError/<@http://127.0.0.1:38345/wasm-bindgen-test:284:22
            wasm_bindgen_test::__rt::__wbg_test_invoke::h2903f6672efdd336@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[469]:0x690a2
            <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::{{closure}}::h17f36c877856ba13@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1069]:0x89c42
            scoped_tls::ScopedKey<T>::set::h8d6f5f6394f3a73c@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[565]:0x6fff4
            <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::hcdb546f09503d276@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[125]:0x3c2bf
            <wasm_bindgen_test::__rt::ExecuteTests as core::future::future::Future>::poll::h34ac976759ab5841@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[55]:0x21669
            wasm_bindgen_test::__rt::Context::run::{{closure}}::h010da5efc2a264c7@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[286]:0x57897
            <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h2db12b417860998e@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[637]:0x74993
            wasm_bindgen_futures::future_to_promise::{{closure}}::{{closure}}::hb61ad9b432e8a595@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[149]:0x41bfe
            <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h8620f66e333f5b07@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[924]:0x839c0
            wasm_bindgen_futures::task::singlethread::Task::run::h8ada318985226ffe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[243]:0x5215c
            wasm_bindgen_futures::queue::QueueState::run_all::he45d6ddcbbfe65e5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[331]:0x5cab6
            wasm_bindgen_futures::queue::Queue::new::{{closure}}::h986232bc2c1399fe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1730]:0x9db6e
            <dyn core::ops::function::FnMut<(A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::h26819f27b977cd50@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[757]:0x7b66b
            __wbg_adapter_18@http://127.0.0.1:38345/wasm-bindgen-test:176:10
            real@http://127.0.0.1:38345/wasm-bindgen-test:136:20
            
            
            
        
        JS exception that was thrown:
            RuntimeError: unreachable executed
            __rust_start_panic@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3961]:0xbe487
            rust_panic@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3499]:0xbb656
            std::panicking::rust_panic_with_hook::h123718ba3bf480af@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[695]:0x78094
            std::panicking::begin_panic::{{closure}}::h80296a1cde52d2c6@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3488]:0xbb4a9
            std::sys_common::backtrace::__rust_end_short_backtrace::hd00e0322330ed085@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3360]:0xb9ed5
            std::panicking::begin_panic::h22f3dacc7e51aec9@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3486]:0xbb44a
            std::sys::wasm::time::Instant::now::h70764a6b710b7aac@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3869]:0xbe090
            std::time::Instant::now::h803a89b9ec1488b3@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3932]:0xbe3a0
            argmin::core::executor::Executor<O,S>::run::h174702a4f90445b1@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[46]:0x18012
            argmin_wasm_test::solve::h3acb1c8ed83f8f50@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[200]:0x4baad
            web::solve_test::hdad69cdf6b4c095d@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3929]:0xbe38a
            core::ops::function::FnOnce::call_once::h5559e00dea6aa5d5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3334]:0xb99ea
            wasm_bindgen_test::__rt::Context::execute_sync::{{closure}}::ha30c895b595ed339@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[966]:0x857f8
            <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h25403166e79259fe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[909]:0x82ead
            <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::{{closure}}::{{closure}}::h3cce7b7045ac5ea1@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1239]:0x8fe3b
            wasm_bindgen::convert::closures::invoke0_mut::hcd3c73d781d702c5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1011]:0x876c1
            __wbg_adapter_32@http://127.0.0.1:38345/wasm-bindgen-test:278:10
            cb0@http://127.0.0.1:38345/wasm-bindgen-test:430:28
            window.__wbg_test_invoke@http://127.0.0.1:38345/:37:38
            init/imports.wbg.__wbg_wbgtestinvoke_03a45ab326341b1b<@http://127.0.0.1:38345/wasm-bindgen-test:435:30
            handleError/<@http://127.0.0.1:38345/wasm-bindgen-test:284:22
            wasm_bindgen_test::__rt::__wbg_test_invoke::h2903f6672efdd336@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[469]:0x690a2
            <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::{{closure}}::h17f36c877856ba13@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1069]:0x89c42
            scoped_tls::ScopedKey<T>::set::h8d6f5f6394f3a73c@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[565]:0x6fff4
            <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::hcdb546f09503d276@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[125]:0x3c2bf
            <wasm_bindgen_test::__rt::ExecuteTests as core::future::future::Future>::poll::h34ac976759ab5841@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[55]:0x21669
            wasm_bindgen_test::__rt::Context::run::{{closure}}::h010da5efc2a264c7@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[286]:0x57897
            <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h2db12b417860998e@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[637]:0x74993
            wasm_bindgen_futures::future_to_promise::{{closure}}::{{closure}}::hb61ad9b432e8a595@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[149]:0x41bfe
            <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h8620f66e333f5b07@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[924]:0x839c0
            wasm_bindgen_futures::task::singlethread::Task::run::h8ada318985226ffe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[243]:0x5215c
            wasm_bindgen_futures::queue::QueueState::run_all::he45d6ddcbbfe65e5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[331]:0x5cab6
            wasm_bindgen_futures::queue::Queue::new::{{closure}}::h986232bc2c1399fe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1730]:0x9db6e
            <dyn core::ops::function::FnMut<(A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::h26819f27b977cd50@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[757]:0x7b66b
            __wbg_adapter_18@http://127.0.0.1:38345/wasm-bindgen-test:176:10
            real@http://127.0.0.1:38345/wasm-bindgen-test:136:20

    failures:

        web::solve_test

    test result: FAILED. 1 passed; 1 failed; 0 ignored
    console.log div contained:
        panicked at 'time not implemented on this platform', library/std/src/sys/wasm/../unsupported/time.rs:13:9
        
        Stack:
        
        init/imports.wbg.__wbg_new_59cb74e423758ede<@http://127.0.0.1:38345/wasm-bindgen-test:591:19
        logError/<@http://127.0.0.1:38345/wasm-bindgen-test:154:22
        console_error_panic_hook::Error::new::hbdb0f39a7e26dfae@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[2833]:0xb2f5c
        console_error_panic_hook::hook_impl::h9694381c72587073@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[357]:0x5f40d
        console_error_panic_hook::hook::h3add71af1744884d@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3187]:0xb7cd4
        core::ops::function::Fn::call::h9516aefea7143c24@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[2628]:0xafcea
        std::panicking::rust_panic_with_hook::h123718ba3bf480af@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[695]:0x7806d
        std::panicking::begin_panic::{{closure}}::h80296a1cde52d2c6@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3488]:0xbb4a9
        std::sys_common::backtrace::__rust_end_short_backtrace::hd00e0322330ed085@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3360]:0xb9ed5
        std::panicking::begin_panic::h22f3dacc7e51aec9@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3486]:0xbb44a
        std::sys::wasm::time::Instant::now::h70764a6b710b7aac@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3869]:0xbe090
        std::time::Instant::now::h803a89b9ec1488b3@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3932]:0xbe3a0
        argmin::core::executor::Executor<O,S>::run::h174702a4f90445b1@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[46]:0x18012
        argmin_wasm_test::solve::h3acb1c8ed83f8f50@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[200]:0x4baad
        web::solve_test::hdad69cdf6b4c095d@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3929]:0xbe38a
        core::ops::function::FnOnce::call_once::h5559e00dea6aa5d5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[3334]:0xb99ea
        wasm_bindgen_test::__rt::Context::execute_sync::{{closure}}::ha30c895b595ed339@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[966]:0x857f8
        <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h25403166e79259fe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[909]:0x82ead
        <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::{{closure}}::{{closure}}::h3cce7b7045ac5ea1@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1239]:0x8fe3b
        wasm_bindgen::convert::closures::invoke0_mut::hcd3c73d781d702c5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1011]:0x876c1
        __wbg_adapter_32@http://127.0.0.1:38345/wasm-bindgen-test:278:10
        cb0@http://127.0.0.1:38345/wasm-bindgen-test:430:28
        window.__wbg_test_invoke@http://127.0.0.1:38345/:37:38
        init/imports.wbg.__wbg_wbgtestinvoke_03a45ab326341b1b<@http://127.0.0.1:38345/wasm-bindgen-test:435:30
        handleError/<@http://127.0.0.1:38345/wasm-bindgen-test:284:22
        wasm_bindgen_test::__rt::__wbg_test_invoke::h2903f6672efdd336@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[469]:0x690a2
        <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::{{closure}}::h17f36c877856ba13@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1069]:0x89c42
        scoped_tls::ScopedKey<T>::set::h8d6f5f6394f3a73c@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[565]:0x6fff4
        <wasm_bindgen_test::__rt::TestFuture<F> as core::future::future::Future>::poll::hcdb546f09503d276@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[125]:0x3c2bf
        <wasm_bindgen_test::__rt::ExecuteTests as core::future::future::Future>::poll::h34ac976759ab5841@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[55]:0x21669
        wasm_bindgen_test::__rt::Context::run::{{closure}}::h010da5efc2a264c7@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[286]:0x57897
        <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h2db12b417860998e@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[637]:0x74993
        wasm_bindgen_futures::future_to_promise::{{closure}}::{{closure}}::hb61ad9b432e8a595@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[149]:0x41bfe
        <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h8620f66e333f5b07@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[924]:0x839c0
        wasm_bindgen_futures::task::singlethread::Task::run::h8ada318985226ffe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[243]:0x5215c
        wasm_bindgen_futures::queue::QueueState::run_all::he45d6ddcbbfe65e5@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[331]:0x5cab6
        wasm_bindgen_futures::queue::Queue::new::{{closure}}::h986232bc2c1399fe@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[1730]:0x9db6e
        <dyn core::ops::function::FnMut<(A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::h26819f27b977cd50@http://127.0.0.1:38345/wasm-bindgen-test_bg.wasm:wasm-function[757]:0x7b66b
        __wbg_adapter_18@http://127.0.0.1:38345/wasm-bindgen-test:176:10
        real@http://127.0.0.1:38345/wasm-bindgen-test:136:20

    Error: some tests failed                          
    error: test failed, to rerun pass '--test web'
    Error: Running Wasm tests with wasm-bindgen-test failed
    Caused by: failed to execute `cargo test`: exited with exit code: 1
      full command: "cargo" "test" "--target" "wasm32-unknown-unknown"