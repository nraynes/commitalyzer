# Commitalyzer

This is a simple commit linter written in Rust. The compiled binary can be executed with command line arguments to 
compare a commit message to a set of pre-defined rules. This is intended to be used in a git hook to lint commit messages
for software projects. This tool requires no dependencies to work as it is a single executable file.

To install, just copy the binary at ./target/release/commitalyzer into your git hook folder (Default is "project_root/.git/hooks"), rename the commitalyzer file to "commit-msg" so that it runs as the commit-msg hook, and also copy the /rules directory to the root of your project.

To add a custom rule, you can add a new yaml file to the rules directory and use the following format to define a rule:

    rule-name:
      pattern: "^regex_pattern$"
      message: "Custom failure message."

Alternatively, you can also copy the executable file and /rules directory to your preferred locations, then make a shell script in your git hook location named "commit-msg", and call the executable file from that script. Be sure to give it the argument "$1" for it's first argument and the absolute path to the /rules directory location as the second argument.

A basic Conventional Commits ruleset is supplied by default to get started.