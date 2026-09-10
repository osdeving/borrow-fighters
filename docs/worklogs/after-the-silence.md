# Diário — Depois do silêncio

## Goal e recuperação

Branch `feature/prologue-scene-improvements`; base limpa **`81c50e6`**.
[Escopo](../33-after-the-silence.md), [ADR](../adr/0027-chapter-spatial-direction.md).
Preservar alterações ao retomar; conferir status/log e este diário antes de
repetir geração, build ou captura. Não fazer merge/tag/publicação.

## Início

- Lidos README, visão/GDD/escopo, lore relevante, direção de arte, arquitetura,
  ADRs 0001/0003/0021/0023 e código de aplicação, menu, render e assets.
- Modelo escolhido: capítulo separado do prólogo; dados espaciais, fases de
  encenação e clips com sockets; composição via pedido opaco do menu.
- Arte em andamento: poses narrativas de Rust com mãos livres para celular
  separado; motorista em PNG independente. Reutilizar moradores e mercearia.
- Windows C: 4,7 GB livres; D: 12 GB. Inspeção/limpeza conservadora delegada.
  Relatório planejado `.git/story-chapter-review/disk-cleanup.json`.
- Pendente: modelo/integração/render/celular/save/áudio, validação nativa e
  testes, evidências, documentação e commits coerentes.

## Integração intermediária

- Base documental commitada em `8d4311f`; rollback visual continua `81c50e6`.
- Modelo puro: três espaços, limites/obstáculo/interações em `world.json`, rotas
  de aproximação/retorno, intro600ticks, conversas e telefone870ticks, checkpoints.
- `ChapterAssets`/renderer por mundo, atores, telefone e interface; extensões de
  clips com duração variável, retenção final e sockets. Arte: 8 poses de Rust,
  2 do motorista e travessa vazia; PNGs ~6MB, prompts e metadados preservados.
- Porta espera48ticks para moradora sair antes de fechar em90. Pulo conserva
  escala do personagem. Rua usa consequências reconstruídas da origem real da EP.
- Modelo, equivalência do ambiente, catálogo/sockets e dez cenários de áudio
  passaram nos testes focados. `cargo check --all-targets --all-features` passou.
- Host/menu/save e loop de capítulo em integração; captura nativa ainda pendente.
- Windows: removidos sete temporários próprios e instalador VSCode obsoleto,
  liberando542.793.728bytes (517,65MiB), C4,653→5,158GiB. VHDX/Codex/dados pessoais
  preservados. Relatório em `.git/story-chapter-review/disk-cleanup.json`.
- Não usar outro `CARGO_TARGET_DIR`; builds compartilham o cache atual. Não
  repetir geração de arte nem limpeza. Próximo: revisar imagens em execução,
  percurso real+pause/skip/retry/continue/menu, áudio e pacote; documentação final.

## Percurso nativo e refinamento

- O percurso funcional em janela X11 própria passou: 4.992 registros,
  vídeo137,63s, diálogo completo, telefone pausado/retomado sem input acumulado,
  salto/colisão, derrota real e R no checkpoint, vitória e save Complete.
  Evidência bruta: `.git/story-chapter-review/native-functional/`.
- Inspeção de screenshots mostrou dois refinamentos necessários: estender o
  asfalto ao mundo inteiro (não apenas1280px) e afastar Rust da moradora da
  travessa antes da conversa. `LaneApproach` reutiliza caminhos existentes e
  termina100px antes dela. Caixote reutiliza recorte da arte já existente.
- Interface extraída para `engine/chapter/ui.rs`. Submenu/pausa também aceitam
  mouse; setas de texto substituídas por "Setas", pois o font não possui glifos.
- `--review` usa perfil próprio dentro da evidência e inicia estado novo;
  prólogo automatizado não registra progresso pessoal. Validação dos clips
  verifica sockets em todos os870ticks nos quais o aparelho deve aparecer.
- O som final da porta agora considera a troca natural de fase no tick138;
  teste integrado cobre fechamento normal e descarte por skip.12testes de áudio.
- Links/YAML:19arquivos e1.579links válidos; fronteiras56testes; pacote19testes.
  Lib conjunta:159passaram,2 testes de dispositivo de áudio preexistentes ignorados.
- Captura do host já confirmou boot/skip, clique em História, checkpoint,
  continuar e revisão preservando progresso. Tentativas de automação foram
  refinadas para aguardar assets e enviar eventos locais de cursor/foco; uma
  verificação genérica de diferença de pixels foi substituída por marcadores
  reais da seleção Versus. Não foram falhas de gameplay; ver logs native-host-*.
- Próximos passos: matriz por features/rebuild final; host com cliques no
  submenu; prévia contínua sem F12 com áudio nativo; evidências e commits finais.
