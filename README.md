# Test case for cargo tarpaulin and file access
## cargo test
Running `cargo test` succeeds ... both project **one** and **two** have test cases that can use a relative path to find and read files from the **files** folder.


```bash
$ cargo test
   Compiling one v0.1.0 (/tarpaulin-test/one)
   Compiling two v0.1.0 (/tarpaulin-test/two)
    Finished test [unoptimized + debuginfo] target(s) in 0.49s
     Running target/debug/deps/one-d4f73933cf4de450

running 1 test
test test::test_file_access ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/two-c15d8277a5ee5802

running 1 test
test test::test_file_access ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

## cargo tarpaulin
Running tarpaulin from a container fails:

```bash
$ sudo docker create --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin:latest bash -c "echo 'sleep 60m; echo bye' > /tmp/keep_alive.sh; chmod 777 /tmp/keep_alive.sh; /tmp/keep_alive.sh" > container_id.txt

$ sudo docker start $(cat container_id.txt)
3e28b7f2c26e0fcccf802f2f427f3ae08058ce2c3e2c97bee145e053a4fa1ba3

$ sudo docker exec $(cat container_id.txt) sh -c "RUST_LOG=trace cargo tarpaulin -v --out Xml"
[INFO tarpaulin] Creating config
[INFO tarpaulin] Running Tarpaulin
[INFO tarpaulin] Building project
   Compiling one v0.1.0 (/volume/one)
   Compiling two v0.1.0 (/volume/two)
    Finished test [unoptimized + debuginfo] target(s) in 0.43s
[INFO tarpaulin] Launching test
[INFO tarpaulin] running /volume/target/debug/two-16aba7f3436ab4af

running 1 test
test test::test_file_access ... FAILED

failures:

---- test::test_file_access stdout ----
thread 'test::test_file_access' panicked at 'unable to find test file: ../files/contents.txt: Os { code: 2, kind: NotFound, message: "No such file or directory" }', two/src/main.rs:11:25
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1052
   5: std::io::Write::write_fmt
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libstd/io/mod.rs:1426
   6: std::io::impls::<impl std::io::Write for alloc::boxed::Box<W>>::write_fmt
             at src/libstd/io/impls.rs:156
   7: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   8: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   9: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
  10: std::panicking::default_hook
             at src/libstd/panicking.rs:221
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:472
  12: rust_begin_unwind
             at src/libstd/panicking.rs:380
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  14: core::option::expect_none_failed
             at src/libcore/option.rs:1199
  15: core::result::Result<T,E>::expect
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libcore/result.rs:991
  16: two::test::test_file_access
             at two/src/main.rs:11
  17: two::test::test_file_access::{{closure}}
             at two/src/main.rs:9
  18: core::ops::function::FnOnce::call_once
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libcore/ops/function.rs:232
  19: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/liballoc/boxed.rs:1015
  20: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  21: std::panicking::try
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libstd/panicking.rs:281
  22: std::panic::catch_unwind
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libstd/panic.rs:394
  23: test::run_test_in_process
             at src/libtest/lib.rs:539
  24: test::run_test::run_test_inner::{{closure}}
             at src/libtest/lib.rs:452
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.


failures:
    test::test_file_access

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

[INFO tarpaulin] Launching test
[INFO tarpaulin] running /volume/target/debug/one-3a019f29a35048d3

running 1 test
test test::test_file_access ... FAILED

failures:

---- test::test_file_access stdout ----
thread 'test::test_file_access' panicked at 'unable to find test file: ../files/contents.txt: Os { code: 2, kind: NotFound, message: "No such file or directory" }', one/src/main.rs:11:25
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1052
   5: std::io::Write::write_fmt
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libstd/io/mod.rs:1426
   6: std::io::impls::<impl std::io::Write for alloc::boxed::Box<W>>::write_fmt
             at src/libstd/io/impls.rs:156
   7: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   8: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   9: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
  10: std::panicking::default_hook
             at src/libstd/panicking.rs:221
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:472
  12: rust_begin_unwind
             at src/libstd/panicking.rs:380
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  14: core::option::expect_none_failed
             at src/libcore/option.rs:1199
  15: core::result::Result<T,E>::expect
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libcore/result.rs:991
  16: one::test::test_file_access
             at one/src/main.rs:11
  17: one::test::test_file_access::{{closure}}
             at one/src/main.rs:9
  18: core::ops::function::FnOnce::call_once
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libcore/ops/function.rs:232
  19: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/liballoc/boxed.rs:1015
  20: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  21: std::panicking::try
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libstd/panicking.rs:281
  22: std::panic::catch_unwind
             at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447/src/libstd/panic.rs:394
  23: test::run_test_in_process
             at src/libtest/lib.rs:539
  24: test::run_test::run_test_inner::{{closure}}
             at src/libtest/lib.rs:452
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.


failures:
    test::test_file_access

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

Error: "Test failed during run"
```
