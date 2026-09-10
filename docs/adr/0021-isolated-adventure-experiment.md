# ADR 0021 — Experimento de aventura isolado do jogo de luta

## Status

Aceito para o experimento em 10 de setembro de 2026, por pedido explícito do
usuário. O corte está no [episódio inicial](../27-rust-story-adventure.md).

Atualização de 10/09/2026: a [ADR 0023](0023-story-to-terminal-menu.md#entrada-padrão--atualização-de-10092026)
promove `borrow-story` e ambas as features a padrão de `cargo run`, por pedido
do usuário. Substitui a escolha de entrada/default descrita abaixo; mantém
o isolamento de domínios, assets e compilações independentes desta decisão.

## Contexto

O usuário quer experimentar duas animações e um combate de aventura com Rust,
podendo descartar o experimento ou evoluir os jogos em paralelo. Uma flag de menu
que continue compilando e carregando ambos os domínios não atende ao isolamento.

## Decisão

Manter um pacote Cargo com duas features e dois executáveis:

- `fighting`, padrão: executável `borrow-fighters`, com os módulos existentes
  de luta, ferramentas, cenas, áudio e apresentação.
- `adventure`, optativa: executável `borrow-adventure`, com domínio, cenas,
  combate, input, áudio e desenho próprios em `src/adventure/`.
- `math` e `runtime_paths`: core neutro existente, compartilhando geometria e
  localização de arquivos, sem regras específicas de qualquer jogo.

Cada domínio importa apenas seu próprio código, core neutro e bibliotecas
externas. A aventura não importa `World`, `Fighter`, `CharacterSpec`, sprites,
seleção, energia ou áudio da luta. A luta não importa `adventure`. O core não
importa nenhum domínio. Raylib fica na borda `adventure/engine` e da aplicação;
o domínio de aventura é testável sem janela.

Binários, testes e exemplos específicos declaram `required-features`, permitindo
compilar/testar cada jogo sem incluir os módulos do outro. `default-run` preserva
`cargo run` como entrada da luta. Uma verificação de fronteiras fiscaliza imports
mesmo quando ambas as features estão ligadas.

Assets ficam em `assets/adventure/`. Arte de Rust pode ser copiada com procedência
como base de identidade; a aventura usa seu catálogo visual próprio e nunca
carrega manifestos de combate/seleção da luta. Capturas e configurações usam
uma subpasta exclusiva nos dados do usuário.

```sh
cargo run --bin borrow-fighters --no-default-features --features fighting
cargo run --bin borrow-adventure --no-default-features --features adventure
cargo test --all-targets --all-features
```

## Alternativas consideradas

- Cena dentro de `App`/`World`: acopla regras e ciclo de vida; rejeitada pelo pedido.
- Workspace com três crates: limites mais fortes com ambos ligados, mas exigiria
  mover grande parte do projeto. Gates e verificações são o menor corte agora.
- Duplicar todo o motor: duplicaria utilidades neutras desnecessariamente.

## Consequências e verificação

Mudanças no core ainda podem afetar ambos e exigem a matriz completa. O isolamento
evita dependências específicas cruzadas; bibliotecas, hardware e core continuam
compartilhados. Verificar cada feature, ambas juntas e core sem features.

Validar na janela real prólogo, manhã, combate, derrota/retry e desfecho, além de
inicializar a luta sem assets da aventura e a aventura sem assets da luta.

## Critério de revisão

Reavaliar crates separados se surgirem imports cruzados recorrentes, dependências
exclusivas pesadas, releases independentes ou core difícil de delimitar.
