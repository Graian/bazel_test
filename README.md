# Repository Title
   
**bazel_test**

## Cargo run command 
```
$ pwd
~/bazel_test

$ cargo run -p mimic -- -i ./mimic/input.txt -o ./output.txt
...
```

## Bazel build/run command
```
$ pwd
~/bazel_test

$ bazel run //mimic:run_mimic -- --input_file input.txt --output_file output.txt
...
```
