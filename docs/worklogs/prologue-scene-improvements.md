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

### Goal ativo — rua brasileira e evacuação completa

- Pedido: assets substituíveis/reutilizáveis, carros cotidianos diferentes,
  ônibus, ponto de ônibus, placas/boteco e reação coletiva à EP, sem trânsito
  reaparecendo depois da fuga.
- Ponto de retorno commitado antes da ampliação: **`c2dbbe4`**. O goal foi
  criado explicitamente, sem limite de tokens solicitado.
- [Escopo 31](../31-brazilian-street-evacuation.md) e
  [ADR 0025](../adr/0025-replaceable-street-pieces.md) registrados docs-first.
- Pendente: catálogo e composição separados; arte e props; estado de
  evacuação e áudio; integração/revisão; evidências e commits finais.
- Recuperação: conferir status/log e este diário, manter mudanças locais e
  procurar arte gerada já salva antes de repetir gerações. Não reverter para
  `c2dbbe4` sem novo pedido: ele é a referência de comparação e retorno.

Retomada após limite de uso:

- `096c5fc` registra o escopo antes da implementação. A versão de retorno
  `c2dbbe4` permanece íntegra.
- Estado puro de evacuação concluído: cinco veículos aceleram sem wrap;
  dois ciclistas freiam/desmontam/correm; bicicletas permanecem; ninguém
  retorna após seis segundos. 19 testes direcionados aprovados pelo agente.
- Veículos novos salvos em `assets/adventure/street/vehicles.*` com procedência.
  Props e ciclistas tiveram gerações RGBA válidas; última tentativa de corrigir
  espaçamento voltou RGB com xadrez e deve ser rejeitada. Fontes preservadas
  em `~/.codex/generated_images/01a08b8b-1def-74e1-baf1-7f46de868f95/`.
- `scenery.rs` e `engine/pieces.rs` implementam catálogo/composição separados;
  renderers já consomem peças e filtram atores evacuados. `cargo check` passa.
- Integração ainda pendente: completar arquivos de catálogo/cena e props,
  metadados/placas, telemetria/harness, empacotamento e mixer do novo áudio;
  depois validar/capturar e commitar as etapas.

### Continuidade posterior

Experimentar a primeira rodada e escolher a próxima cena nesta mesma branch.
Ada, Assembly e apresentação poderão receber propostas próprias; não há
implementação desses próximos ajustes pendente nesta solicitação. Manter
o experimento separado da main e da tag até pedido de integração/publicação.

### Integração após retomada

- Catálogo e cena externos integrados com todos os PNGs finais: seis veículos
  disponíveis, cinco no trânsito; poses de desmontagem/corrida; fachada, ponto,
  bicicletas e canto de mercearia. Duas instâncias reaproveitam `prop.corner`.
- Geração textual final do ciclista resolveu espaçamento e preservou alpha real.
  Metadados e procedência completos acompanham cada atlas; sem edição de pixels.
- Primeira inspeção nativa: `.git/street-chaos-review/inspect-1`, capturas de
  calma, aproximação, desmontagem, corrida e rua vazia. Sem faltas de assets;
  bicicleta e destroço persistem. Ajuste de áudio: queda soa no contato ao tick70.
- `cargo build --bin borrow-adventure` passou. Empacotamento:14 testes passaram,
  inclusive descoberta de PNGs por todos os frames e rejeição de caminho externo.
- Pendente: verificação nativa completa/áudio, matriz final, evidências e commits.

### Etapa implementada e verificada

- 22/22 checks nativos em `.git/street-chaos-review/native-1`: 1219 amostras
  sem wrap/respawn, fases completas dos dois ciclistas, via vazia do tick 361
  ao 1397, câmera direita, pausa, derrota/retry, skip e restart.
- Matriz em `.git/street-chaos-review/validation/verification.json`: Fmt e
  Clippy estrito passaram; 450 testes conjuntos, 56 aventura, 395 luta e 3 core.
  Dois testes preexistentes de dispositivo de áudio ignorados. 56 fixtures
  de fronteira e 40 testes do mixer aprovados. Python 3.8 não executa o checker;
  a tentativa com Python 3.13 passou. Fontes permaneceram estáveis na matriz.
- 14 testes de empacotamento; staging Linux real com 240 assets aprovado.
  Windows coberto por fixtures, sem executável Windows local disponível.
  Troca de PNG/reuso confirmados em árvore temporária, composição preservada.
- Prévia contínua gravada com áudio do próprio jogo em sink Pulse dedicado:
  `.git/street-chaos-review/preview`, 13,73 segundos/412 frames. Exportação e
  relatório do áudio em revisão final; sem cortes ou substituição de conteúdo.
- Código e assets prontos para commit desta etapa; resta finalizar documentação
  das evidências, checar links e registrar o fechamento.

### Goal concluído

- `65d826b` commita implementação, arte, contrato de catálogo/composição,
  evacuação, áudio e ferramentas. O retorno solicitado continua `c2dbbe4`.
- [Entrega com prévia e evidências](../evidence/brazilian-street-evacuation/README.md):
  vídeo contínuo de 13,73 segundos, som real do jogo, 22 checks nativos e
  matriz completa. Cinco cues confirmados no PCM; latência de 10–16 ms,
  sem clipping. A queda foi analisada em banda porque a buzina se sobrepõe
  aos primeiros 67 ms; nenhum filtro foi aplicado ao som entregue.
- Validação documental final: 19 YAMLs e 1430 links Markdown locais;
  `git diff --check` aprovado. CLI antigo encaminha à revisão atual;
  compilação Python e ajuda dos dois comandos aprovadas.
- Nenhuma etapa de implementação pendente neste goal. Avaliação do ritmo
  pelo usuário poderá orientar ajustes posteriores. Main e tag preservadas.
- Recuperação: usar esta branch e conferir `git status`/`git log` antes de
  novas alterações. Para comparação, abrir `c2dbbe4` numa branch própria;
  não fazer reset que descarte trabalho posterior.
