# Fontes da interface

As fontes acompanham o executável via `include_bytes!`; menu, HUD e livro não
dependem de uma instalação de fontes no computador do jogador.

| Arquivo | Uso | Fonte e licença |
|---|---|---|
| `BarlowCondensed-SemiBold.ttf` | Menu, títulos e HUD | [Google Fonts / Barlow Condensed](https://github.com/google/fonts/tree/main/ofl/barlowcondensed), [SIL OFL 1.1](BARLOW-OFL.txt) |
| `Barlow-Regular.ttf` | Corpo do livro e leitura longa | [Google Fonts / Barlow](https://github.com/google/fonts/tree/main/ofl/barlow), [SIL OFL 1.1](BARLOW-OFL.txt) |
| `Lora-Variable.ttf` | Títulos do livro, peso padrão regular | [Google Fonts / Lora](https://github.com/google/fonts/tree/main/ofl/lora), [SIL OFL 1.1](LORA-OFL.txt) |

Baixadas dos repositórios acima em 2026-09-08, sem modificação do conteúdo.
O nome local de Lora corresponde ao arquivo upstream `Lora[wght].ttf`.

O carregador rasteriza cada atlas a 96 px, gera mipmaps e aplica filtro trilinear
(bilinear entre pixels e interpolação entre níveis) antes de reduzir ao tamanho
de tela. A filtragem bilinear isolada ainda serrilhava as letras pequenas durante
a revisão visual; os mipmaps suavizam a redução de 96 px para rodapés a 14 px.
O conjunto inclui ASCII, Latin-1, aspas, travessões e marcadores para preservar
português e instruções de navegação.
Layout e medição usam a mesma fonte e espaçamento; um erro de GPU mantém o
fallback de desenho padrão do Raylib.

Para revisar as cinco páginas, a barra de vida e o resultado de luta:

```sh
cargo run --example capture_ui_review -- /tmp/borrow-ui-review
```

A captura usa uma janela oculta e exige um contexto gráfico disponível. Também
recusa fontes sem glifos de português, mesmo se o atlas ASCII carregar.
