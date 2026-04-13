# Commitalyzer

This is a simple commit linter written in Rust. The compiled binary can be executed with command line arguments to 
compare a commit message to a set of pre-defined rules. This is intended to be used in a git hook to lint commit messages
for software projects. This tool requires no dependencies to work as it is a single executable file.

## Installation

To install, just copy the binary file named "commit-msg" in the ./bin directory into your git hook folder (Default is "<project_root>/.git/hooks"), then also copy the ./rules directory to the root of your project.

Alternatively, you can also copy the executable file and /rules directory to your preferred locations, then make a shell script in your git hook location named "commit-msg", and call the executable file from that script. Be sure to give it the argument "$1" for it's first argument and the absolute path to the /rules directory location as the second argument.

### Example

***commit-msg (shell script)***    

    /absolute/path/to/commitalyzer/binary $1 /absolute/path/to/rules/directory

## Custom Rulesets

A yaml file that contains many rules tailored to enforcing a specific git commit format can be used as a ruleset to be applied to all commit messages in your project. All rulesets in the ./rules directory in the root of your project (or wherever you configured your rules directory to be if using a different location) will be applied when the commit-msg hook runs, and will not allow the commit to complete if any one of the rule checks within any of the rulesets fails.

### Custom Rules

To add a custom rule, you can add a new yaml file to the rules directory and use the following format to define a rule:

    rule-name:
      pattern: "^regex_pattern$"
      message: "Custom failure message."



A basic Conventional Commits ruleset is supplied by default to get started.

## Building a New Release Binary

After running "cargo build --release" to build a new release binary, run the ./post-build-release.sh script from the projects root directory. This script will copy the generated release binary to the ./bin directory within the projects root directory and rename it to "commit-msg". This is to make it easier for someone to just grab that binary to put in their hooks directory without having to perform the steps of finding it in the target directory and then renaming it. If using Windows, just copy the file manually from ./target/release/commitalyzer into ./bin and rename the file from "commitalyzer" to "commit-msg".
