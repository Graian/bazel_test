# Repository Title
   
**bazel_test/sv_parser**

## Cargo run command 

* Cargo version
  * 1.71.0

```
$ pwd
~/bazel_test/sv_parser

$ cargo run -p main 2>&1 | tee cargo_run.log
...
```

## Bazel run command

* Bazel version
  * 6.2.1
* Bazel release
  * https://github.com/bazelbuild/bazel/releases/tag/6.2.1

```
$ pwd
~/bazel_test/sv_parser

$ bazel run //main:run_main --experimental_ui_max_stdouterr_bytes=3200000 2>&1 | tee bazel_run.log
...
```
