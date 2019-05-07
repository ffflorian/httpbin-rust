workflow "Test" {
  on = "push"
  resolves = "Test the project"
}

action "Don't skip CI" {
  uses = "ffflorian/actions/last_commit@master"
  args = "^(?:(?!\\[(ci skip|skip ci)\\]).)*$"
}

action "Test the project" {
  uses = "./.github/actions/rust-test"
  needs = "Don't skip CI"
}
