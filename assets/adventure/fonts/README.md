# Fontes da aventura

As fontes são cópias independentes dos arquivos existentes em `assets/fonts/`.
A aventura carrega somente esta pasta e não depende de fontes instaladas no host.

| Arquivo | Uso | Fonte e licença |
|---|---|---|
| `Barlow-Regular.ttf` | Mensagem, legendas, comandos e HUD | [Google Fonts / Barlow](https://github.com/google/fonts/tree/main/ofl/barlow), [SIL OFL 1.1](BARLOW-OFL.txt) |
| `Lora-Variable.ttf` | Títulos narrativos | [Google Fonts / Lora](https://github.com/google/fonts/tree/main/ofl/lora), [SIL OFL 1.1](LORA-OFL.txt) |
| `BarlowCondensed-SemiBold.ttf` | Manchetes, nomes e logo da apresentação | Cópia do catálogo local, [SIL OFL 1.1](BARLOW-OFL.txt) |

Baixadas dos repositórios acima em 2026-09-08, sem modificação do conteúdo.
O nome local de Lora corresponde ao arquivo upstream `Lora[wght].ttf`.

O carregador próprio rasteriza Barlow a 48 px, Lora a 64 px e Barlow Condensed a
120 px, com glifos latinos e pontuação tipográfica para português. Para revisar
as cenas da aventura:

```sh
cargo run --no-default-features --features adventure --bin borrow-adventure -- --review /tmp/borrow-adventure-review
```

A captura usa uma janela oculta e exige contexto gráfico e FFmpeg. Jogar não
exige FFmpeg.
