# 30 — Melhorias das cenas do prólogo

Experimento autorizado em 10 de setembro de 2026 na branch
`feature/prologue-scene-improvements`, a partir da prototype.4 em `main`.
A branch reúne melhorias das cenas do prólogo; esta primeira rodada trata
do quarto de Rust e da rua durante a manhã, conforme pedido do usuário.

**Estado:** duas rodadas implementadas e verificadas na branch. A segunda tem
445 testes conjuntos e 20 checks nativos, com matriz isolada, Fmt, Clippy,
fronteiras, áudio e empacotamento aprovados.
[Quarto/ciclovia/pipa](evidence/prologue-scene-improvements/README.md) e
[trânsito/acidente com som](evidence/prologue-traffic/README.md).

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

## Segunda rodada — trânsito e acidente

Pedido adicional: intensificar a vida da rua com automóveis e a reação à EP
com buzina, frenagem, colisão em um poste e deformação visível do carro.

- Abrir uma faixa de asfalto ao fundo, mantendo ciclovia, calçada do garoto
  e piso jogável claramente separados. Carros circulam antes da ameaça.
- Quando `enemy_awake` disparar, encenar uma única aproximação com buzina,
  frenagem e impacto no poste. O capô amassado, uma breve nuvem de poeira e
  fumaça leve sustentam a consequência visual no restante do encontro.
- Compartilhar o relógio da encenação entre imagem e cues de áudio. Pausa,
  retry/restart e skip devem congelar, restaurar ou descartar a sequência
  de forma coerente, sem repetir sons por frame renderizado.
- A colisão é encenação de fundo: não altera vida, hitboxes, movimento ou
  resultado do combate. Preservar quarto, ciclistas e fuga da pipa.

Estado: implementada. Tráfego contínuo, contato com o poste, carro amassado
persistente, câmera, pausa e fluxo conferidos na execução nativa.
Prévia e provas na [revisão da segunda rodada](evidence/prologue-traffic/README.md).

## Verificação das rodadas

Conferir os apoios de Rust nas poses da manhã, ciclistas em posições distintas,
garoto antes da ameaça e a sequência de soltar a pipa e fugir. Testar pausa,
retomada, reinício e separação das regras de combate. Capturar a execução real
e conferir também os pontos de entrada `--start morning` e `--start encounter`.

Status, comandos e evidências ficam no
[diário da branch](worklogs/prologue-scene-improvements.md).
