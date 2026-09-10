# Melhorias das cenas do prólogo

## Pedido e retomada

- Branch: `feature/prologue-scene-improvements`.
- Base: `main`, `ac05e4b`, código da `v0.1.0-prototype.4` mais docs finais.
- Pedido: experimentar melhorias das cenas do prólogo, começando pela casa
  tecnológica de Rust, ciclistas ao fundo e garoto soltando pipa que foge
  quando a EP aparece.
- [Escopo](../30-prologue-scene-improvements.md) e
  [decisão](../adr/0024-prologue-background-life.md).
- A release publicada permanece na tag existente; esta é uma branch de trabalho.

Primeira rodada implementada e verificada. Prévia: `cargo run -- --start morning`
ou `cargo run -- --start encounter`. [Evidências](../evidence/prologue-scene-improvements/README.md).

## Etapas

- [x] Conferir main limpa, ler contexto e criar branch de experimento.
- [x] Registrar primeira rodada e fronteira entre encenação e combate.
- [x] Produzir e integrar ambiente e atores de fundo.
- [x] Implementar animação, fuga, pausa e reinício coerentes.
- [x] Validar testes, fronteiras, pacotes e documentação.
- [x] Capturar e revisar quarto, rua e reação à errática.

## Recuperação

Antes de retomar, ler este diário, `git status` e `git log`. Preservar mudanças
locais e consultar evidências/gerações já feitas antes de repetir trabalho.
Não mover a tag prototype.4 nem integrar o experimento à main como parte
desta rodada.

## Checkpoints

- Rodada concluída: 437 testes conjuntos, 43 aventura isolada, 395 luta isolada,
  3 core; Fmt, Clippy estrito e 56 fixtures de fronteira. Dez testes de
  empacotamento e staging com 227 assets aprovados; launcher abriu RustMorning
  fora do checkout. YAML e 1320 destinos Markdown locais verificados.
  Logs e staging locais em `.git/prologue-review/`.
- Capturas nativas: 15 checks da sequência, seis da encenação, cinco de
  entrada direta/câmera e cinco do quarto final. Pausa, retry, restart, skip,
  soltura e fuga conferidos. Vídeo e quadros selecionados preservados nas
  evidências; a última calibração do pôster aparece nos PNGs 090/390/650.
- Arte integrada: `prologue-environments.png` preserva cama e cria setup,
  pôsteres e ciclovia; `street-life.png` tem 12 poses com alpha medido em JSON.
  Origem, SHA e prompts registrados em `assets/adventure/ART-PROVENANCE.md`.
  Os 76 valores de texto existentes permaneceram idênticos; quatro chaves
  novas permitem editar os textos dos pôsteres e do computador.
- Encenação pertence a `Story.ambient`; `engine/street.rs` observa o relógio
  e desenha os três planos. EP e susto aparecem no mesmo sinal; nenhum ator
  decorativo participa da física. Raios das rodas, pedais, cursor, ventiladores,
  gestos, linha, pipa e fuga receberam revisão na janela.
- Quarto e rua atuais inspecionados; os apoios do quarto estão calibrados
  em `engine/morning.rs`. A rua usa câmera própria e tem exploração antes
  de `combat.enemy_awake`; a reação decorativa deve acompanhar esse sinal.

## Continuação

Experimentar a primeira rodada e escolher a próxima cena nesta mesma branch.
Ada, Assembly e apresentação poderão receber propostas próprias; não há
implementação desses próximos ajustes pendente nesta solicitação. Manter
o experimento separado da main e da tag até pedido de integração/publicação.
