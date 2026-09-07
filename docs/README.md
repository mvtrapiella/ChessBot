# Documentation

Architecture documentation for Atlas, following the [arc42](https://arc42.org/) template
(see `index.adoc` for the section list, and `src/` for one file per section).

## Requirements

We use [AsciiDoc](https://asciidoc.org/) and [PlantUML](https://plantuml.com) to write the
documentation, and [Asciidoctor](https://asciidoctor.org/) (with asciidoctor-diagram) to
render it to HTML. This requires Ruby (Gem) and Java (JRE).

```shell
gem install asciidoctor asciidoctor-diagram
```

Then install the Node dependencies inside this `docs` directory:

```shell
cd docs
npm install
```

## Building

```shell
npm run build
```

The rendered site is generated under `docs/build`.

## Deploying

```shell
npm run deploy
```

This pushes `docs/build` to the repository's `gh-pages` branch via the `gh-pages` package,
making it available on GitHub Pages. Only the rendered documentation should live on that
branch — never the build output on `main`.
