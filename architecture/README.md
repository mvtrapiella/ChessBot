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

Then install the Node dependencies inside this `architecture` directory:

```shell
cd architecture
npm install
```

## Building

```shell
npm run build
```

This renders the site straight into `/docs` at the repository root (overwriting whatever
was there). Commit and push that folder to `main` and it's live.

## Publishing to GitHub Pages

In the repo settings, under **Pages**, set the source to the `main` branch, `/docs` folder.
From then on, every push to `main` that updates `/docs` (via `npm run build` + commit)
updates the published site — no separate branch or deploy step needed.
