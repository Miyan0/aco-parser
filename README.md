# aco-parser

[Adobe Spec](https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577411_pgfId-1055819)

Reads an `.aco` file and exports it to web colors in these formats: (`css`, `scss` and `css-variable`)

## Background

I'm doing web development and don't have any Adobe products (I use Affinity's). When I look for inspiration I often go to [Dribbble](https://dribbble.com/). Many designers on Dribbble include a color pallette in `.aco` format. Since this format is not supported by Affinity, I thought it could be fun to make a converter in **Rust**.

## Notes

This is my first real **Rust** project so, I'm surely doing many things the wrong way... It works for me though and I learned a lot doing it.
