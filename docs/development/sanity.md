# 🩺 Sanity

## Access

Sanity dashboard is automatically built and configured for debug build and
`development` environment. If one of these is missing, it's not built.

It's available in the application by reaching the URL `/sanity`.

## Data sources

The dashboard loads some report files located in `target/sanity/` but you have
to manually generate them by calling:

```shell
makers sanity
```
