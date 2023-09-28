# Contributing to Sherlock Code

We welcome contributions to Sherlock Code!

## How to Contribute

If you would like to contribute to Sherlock Code, please follow these steps:

1. Fork the repository and clone it to your local machine.
2. Create a new branch for your changes.
3. Make your changes and commit them to your branch.
4. Push your branch to your fork on GitHub.
5. Submit a pull request to the main repository.

## Processes

We do not have PR templates or formal processes for approving PRs. But there
are a few guidelines that will make it a better experience for everyone:

- Run `cargo fmt` before submitting your code.
- Keep PRs limited to addressing one feature or one issue, in general. In some
  cases (e.g., “reduce allocations in the reactive system”) this may touch a number
  of different areas, but is still conceptually one thing.
- If it’s an unsolicited PR not linked to an open issue, please include a
  specific explanation for what it’s trying to achieve. For example: “When I
  was trying to deploy my app under _circumstances X_, I found that the way
  _function Z_ was implemented caused _issue Z_. This PR should fix that by
  _solution._”
- Our CI tests every PR against all the existing examples, sometimes requiring
  compilation for both server and client side, etc. It’s thorough but slow. If
  you want to run CI locally to reduce frustration, you can do that by installing
  `cargo-make` and using `cargo make check && cargo make test && cargo make
check-examples`.

## Before Submitting a PR

We have a fairly extensive CI setup that runs both lints (like `rustfmt` and `clippy`)
and tests on PRs. You can run most of these locally if you have `cargo-make` installed.

If you added an example, make sure to add it to the list in `examples/Makefile.toml`.

From the root directory of the repo, run
- `cargo +nightly fmt`
- `cargo +nightly make check`
- `cargo +nightly make test`
- `cargo +nightly make --profile=github-actions ci`

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md).

## License

By contributing to Sherlock Code, you agree to license your contributions under
the [LICENSE.md](./LICENSE) file in the root of the repository.
