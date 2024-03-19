# ⚙️ Configuration

## Environments

Here's the list of available environment:

- `development`: settings applied for local development and development
                 platform.
- `staging`: settings applied for pre-production platform.
- `production`: settings applied for production platform.
- `testing`: settings applied for running unit-tests.

The application expect the environment variable `ENVIRONMENT` to be set or
fallbacks to the development value.

## Static configuration

Configuration files are located in `<root>/crates/server/config` directory.
Each environment has a dedicated [YAML][0] configuration:

- `development.yml`
- `staging.yml`
- `production.yml`
- `testing.yml`

A base file is loaded (`base.yml`) by the application and all settings are
available unless they are overrided by the environment configuration.

## Dotenv configuration

Some configurations are made by environment variables. They can be defined in a
`.env` file placed at the root folder of the repository and are loaded during
development process (i.e. not for productions environments).

A sample file is provided and can be used as is: `.env.sample`.

> **Note**
> Get more information about [dotenv](dotenv.md).

## Dynamic overrides

Note that it's possible, but not recommended, to override dynamically the
configuration values by using environment variables.

All variables must start with the prefix `OVERRIDE_` followed by the scope
build from category name and key name separated by underscores.

E.g. the following variable `OVERRIDE_APPLICATION_PORT` will override the
following configuration:

```yaml
application:
  port: 8080
```

[0]: https://yaml.org/spec
