# 30 — Melhorias das cenas do prólogo

Experimento autorizado em 10 de setembro de 2026 na branch
`feature/prologue-scene-improvements`, a partir da prototype.4 em `main`.
A branch reúne melhorias das cenas do prólogo; esta primeira rodada trata
do quarto de Rust e da rua durante a manhã, conforme pedido do usuário.

**Estado:** primeira rodada implementada e verificada na branch. São 437 testes
conjuntos e 31 verificações nativas/de encenação, com matriz isolada, Fmt, Clippy,
fronteiras e empacotamento aprovados. [Vídeo, imagens e resultados](evidence/prologue-scene-improvements/README.md).
O próximo passo é experimentar no jogo e escolher os ajustes das próximas cenas.

## Primeira rodada

- Quarto com setup de computador e pôsteres ligados a Rust, preservando a
  luz da manhã, o caráter de casa brasileira e os apoios da animação na cama.
- Bicicletas em movimento numa ciclovia ao fundo, separada visualmente da
  faixa de Rust. São atores decorativos, sem colisão ou hitbox.
- Garoto brincando com pipa antes da ameaça. Ao despertar da errática,
  reage, larga a linha e foge; a pipa continua pelo vento. Não reaparece
  tranquilamente durante o mesmo combate.
- Movimento de pedais, rodas, corpo, braços, corrida e cauda da pipa;
  passagem pelo cenário deve comunicar vida, sem esconder ações jogáveis.
- Pausa congela a encenação. Retry reinicia seu estado de forma coerente;
  avanço por trecho e skip total continuam funcionando.

Os detalhes e a animação ficam no domínio `adventure`, conforme a
[ADR 0024](adr/0024-prologue-background-life.md). O cenário de fundo fica
separado em profundidade; não se cria navegação, física ou colisão de NPCs.
Melhorias de Ada, Assembly e demais cenas poderão continuar nesta branch
em rodadas próprias. Esta solicitação não publica outra release.

## Verificação

Conferir os apoios de Rust nas poses da manhã, ciclistas em posições distintas,
garoto antes da ameaça e a sequência de soltar a pipa e fugir. Testar pausa,
retomada, reinício e separação das regras de combate. Capturar a execução real
e conferir também os pontos de entrada `--start morning` e `--start encounter`.

Status, comandos e evidências ficam no
[diário da branch](worklogs/prologue-scene-improvements.md).
