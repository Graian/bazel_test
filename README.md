# Repository Title
   
**bazel_test**

## Cargo run command 
```
$ pwd
~/bazel_test

$ cargo run -p mimic -- -i ./mimic/input.txt -o ./mimic/output.txt
...
```

## Bazel build/run command
```
$ pwd
~/bazel_test

$ bazel run //mimic:run_mimic -- --input_file ./mimic/input.txt --output_file ./mimic/output.txt
...
```
