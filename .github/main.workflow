workflow "Test projects" {
  on = "push"
  resolves = "Test all projects"
}

action "Don't skip CI" {
  uses = "ffflorian/actions/last_commit@master"
  args = "^(?:(?!\\[(ci skip|skip ci)\\]).)*$"
}

action "Test all projects" {
  uses = "./.github/actions/rust-test"
  needs = "Don't skip CI"
}
