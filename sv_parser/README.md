# Repository Title
   
**bazel_test/sv_parser**

## Cargo run command 
```
$ pwd
~/bazel_test/sv_parser

$ cargo run -p main 2>&1 | tee cargo_run.log
...
```

## Bazel run command
```
$ pwd
~/bazel_test/sv_parser

$ bazel run //main:run_main --experimental_ui_max_stdouterr_bytes=3200000 2>&1 | tee bazel_run.log
...
```
