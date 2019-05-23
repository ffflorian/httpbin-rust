workflow "Test" {
  on = "push"
  resolves = "Test the project"
}

action "Don't skip CI" {
  uses = "ffflorian/actions/skip-ci-check@v1.0.0"
}

action "Test the project" {
  uses = "./.github/actions/rust-test"
  needs = "Don't skip CI"
}
