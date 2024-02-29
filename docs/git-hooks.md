# Git Hooks

You must install the Git hooks provided by this repository and located in `.githooks`:

```shell
git config core.hooksPath .githooks
```

Available hooks:

- **commit-msg**: Checks the format of the commit message against the conventional commits specification.
- **pre-commit**: Check that dotenv is not commited and rustfmt doesn't return any difference.
- **post-checkout**: Update submodules when switching branch.

> **Note**
> Conventional commits message specification can be found [here][0].

> **Note**
> More information about Git hooks can be found [here][1].

[0]: https://www.conventionalcommits.org/en/v1.0.0
[1]: https://git-scm.com/docs/githooks
