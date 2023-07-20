# Repository Title
   
**bazel_test/copy_and_append**

## Cargo command 
```
$ pwd
~/bazel_test/copy_and_append

$ cargo run -p main -- -i ./main/input.txt -o ./main/output.txt
...
```

## Bazel command
```
$ pwd
~/bazel_test/copy_and_append

$ bazel run //main:run_main -- -i ./main/input.txt -o ./main/output.txt
...
```
