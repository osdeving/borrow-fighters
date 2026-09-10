# 29 — Da abertura ao menu de terminal

## Pedido e escopo

O fim da apresentação leva ao menu principal, com identidade coerente com o logo
da aventura. A janela de terminal traz cursor de bloco piscante e números que
se transformam nas opções. `Modo História` ocupa a primeira linha, sem ação por
enquanto; `Versus Setup` conserva a seleção, e os demais destinos permanecem.
Lutas, seleção do elenco e livro de lore não recebem reformulação nesta rodada.
Go fica fora da apresentação.

## Execução prevista

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

## Verificação e retomada

Implementação em andamento. Registrar resultados concretos e capturas antes
de concluir. O [diário](worklogs/rust-adventure-prologue.md) guarda checkpoints.
