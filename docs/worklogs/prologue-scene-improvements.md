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

### Segunda rodada concluída — automóveis e acidente

- Pedido: mais movimento na rua e, na entrada da EP, buzina e automóvel
  batendo/amassando no poste junto da fuga do garoto.
- Base conferida: `3f6d9da`, branch limpa antes desta rodada.
- Decisão: ampliar a encenação da ADR 0024 com faixa de asfalto, tráfego,
  sequência única de frenagem/impacto e cues de áudio próprios da aventura.
- Arte dos carros, estado, integração de áudio/render, testes e revisão
  nativa concluídos. Não integrar/publicar sem novo pedido.

Checkpoint de implementação:

- Arte e áudio integrados; estado puro contém carros contínuos e acidente
  nos ticks 38/78/112 da reação à EP. Renderer conserva ciclovia e chão,
  abre asfalto e desenha carro amassado, poste e efeitos locais.
- `cargo fmt`, Clippy estrito e matriz Rust passaram: 445 testes conjuntos,
  51 aventura, 395 luta, 3 core. Fronteiras: 56 fixtures; mixer de revisão:
  38 testes; empacotamento: dez testes, incluindo novos assets.
- Primeira captura nativa confirmou rua, frenagem/pausa, contato e permanência;
  a automação morreu ao atravessar a EP antes de atingir a câmera direita.
  Ajustar somente os comandos da revisão para completar câmera/retry/skip.
  Capturas e logs ficam em `.git/traffic-review/native-1`; testes no diretório
  pai. Áudio capturado em sink PulseAudio exclusivo, sem som do desktop.

Checkpoint final:

- `native-2/native-checks.json`: 20/20 checks nativos aprovados, incluindo
  extremo direito, derrota/retry, pulo durante frenagem e restart sem destroços.
  A revisão usou defesa durante as capturas e avanço nos intervalos de
  recuperação da EP; regras/saúde do jogo não foram alteradas.
- Prévia contínua separada em `.git/traffic-review/preview`, sem F12 ou pausas
  durante a cena, com som real gravado em `preview-audio.mka`. A captura
  funcional conserva seu vídeo e telemetria próprios em `native-2`.
- [Evidências selecionadas](../evidence/prologue-traffic/README.md) incluem
  vídeo com som, imagens, verificações e procedência. Atlas e prompts em
  `assets/adventure/street-traffic.*` e `assets/adventure/prompts/street-traffic*`;
  WAVs originais e gerador em `assets/adventure/audio/`.
- Comandos de verificação: `cargo fmt --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all-targets --all-features` e as três combinações isoladas
  de features. Python: `tools/check_domain_boundaries.py` e unittest discovery
  de `test_check_domain_boundaries.py`, `test_mix_adventure_review_audio.py`
  e `test_package.py`. Logs Rust em `.git/traffic-review/`.
- Rodada em alterações locais revisáveis nesta branch; base continua
  `3f6d9da`. Ao retomar, preservar o trabalho local antes de qualquer checkout.
- Prévia final: 10,27 segundos/308 frames, vídeo original preservado e áudio
  nativo alinhado. Buzina, pneus e batida confirmados por correlação com a
  captura; sem clipping. Validação final: 19 YAMLs e 1351 links Markdown
  locais aprovados, além de `git diff --check` e compilação do harness Python.

### Após as rodadas

Experimentar a primeira rodada e escolher a próxima cena nesta mesma branch.
Ada, Assembly e apresentação poderão receber propostas próprias; não há
implementação desses próximos ajustes pendente nesta solicitação. Manter
o experimento separado da main e da tag até pedido de integração/publicação.
