# Commitalyzer

This is a simple commit linter written in Rust. The compiled binary can be executed with command line arguments to 
compare a commit message to a set of pre-defined rules. This is intended to be used in a git hook to lint commit messages
for software projects. This tool requires no dependencies to work as it is a single executable file.

To install, just take the binary at ./target/release/commitalyzer along with the /rules directory and copy them into your git hook folder. Rename "commitalyzer" to your preferred git hook like "commit" or "pre-commit" and you're good to go.

To add a custom rule, you can add a new yaml file to the rules directory and use the following format to define a rule:

    rule-name:
      pattern: "^regex_pattern$"
      message: "Custom failure message."

Alternatively, you can also copy the executable file and /rules directory to your preferred locations, then make a shell script in your git hook location name "commit" or "pre-commit" (Or whatever hook type you're using.), and call the executable file from that script. Be sure to give it the argument "$1" for it's first argument and the absolute path to the /rules directory location as the second argument.

A basic Conventional Commits ruleset is supplied by default to get started.