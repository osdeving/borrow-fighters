# 29 — Da abertura ao menu de terminal

## Pedido e escopo

O fim da apresentação leva ao menu principal, com identidade coerente com o logo
da aventura. A janela de terminal traz cursor de bloco piscante e números que
se transformam nas opções. `Modo História` ocupa a primeira linha, sem ação por
enquanto; `Versus Setup` conserva a seleção, e os demais destinos permanecem.
Lutas, seleção do elenco e livro de lore não recebem reformulação nesta rodada.
Go fica fora da apresentação.

## Execução

```sh
# Ada → manhã → encontro jogável → pesar → apresentação → menu.
cargo run --features adventure --bin borrow-story

# Rever apenas apresentação → menu.
cargo run --features adventure --bin borrow-story -- --start opening

# Abrir diretamente o menu renovado.
cargo run --features adventure --bin borrow-story -- --menu
```

Os comandos independentes do README continuam disponíveis. O novo executável
é uma composição das APIs de aplicação, conforme [ADR 0023](adr/0023-story-to-terminal-menu.md).

Os gráficos do menu/luta são preparados antes do prólogo. O menu guarda seus
próprios recursos; a aventura não os acessa. Isso evita carregar os atlas entre
o título e o menu. Os dispositivos de áudio são abertos em sequência.
As flags `--hidden`/`--review` se aplicam à aventura; a conclusão torna o menu
visível. Atingir `--frames N` antes da conclusão encerra a sessão sem abrir menu.

## Continuidade da lore

O [TODO-016](03-backlog.md#backlog-de-to-do) reserva a atualização do livro de
lore para incorporar imagens e conteúdo já apresentados pela aventura.
Os arquivos e o desenho atuais de `Lore / Roster` foram preservados.

## Verificação e retomada

A matriz passou com 421 testes de ambos os modos, 395 somente luta,
27 aventura/core e 3 core, além de Fmt, Clippy e 54 fixtures de fronteira.
As [capturas e relatórios](evidence/story-terminal-menu/README.md) reúnem desenho,
transição e navegação. O [diário](worklogs/rust-adventure-prologue.md) guarda checkpoints.
