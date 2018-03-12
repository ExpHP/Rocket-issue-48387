# Looks like Team Rocket has blasted off again

<!-- 嫌な感じー！！ --->

This is `rocket_codegen` minimized down to 14 exported macros that just `loop { }`, and the `plugin_registrar` function that registers them.

```
$ git clone https://github.com/ExpHP/Rocket-issue-48387
$ cd Rocket-issue-48387
$ cargo build --example=issue-48387
   Compiling rocket_codegen v0.4.0-dev (file:///home/lampam/asd/clone/Rocket)
error: internal compiler error: Error constructed but not emitted

thread 'rustc' panicked at 'explicit panic', librustc_errors/diagnostic_builder.rs:242:13
```

A function that just loops surely cannot be responsible for dropping a `DiagnosticBuilder`, so the `Error constructed but not emitted` must be a bug in rustc.
