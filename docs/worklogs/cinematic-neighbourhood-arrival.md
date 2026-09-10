# Diário — chegada cinematográfica e vizinhança

## Goal e recuperação

Branch: `feature/prologue-scene-improvements`. Versão aprovada de referência:
**`bdd9ae6`**. Workspace limpo no início; não é necessário commit vazio.
Pedido e critérios: [escopo 32](../32-cinematic-neighbourhood-arrival.md).
Decisão: [ADR 0026](../adr/0026-cinematic-arrival-and-neighbours.md).

Manter trabalho local ao retomar, conferir status/log antes de repetir geração
ou captura. Arte final deve estar em `assets/adventure/street/`; fontes geradas
ficam em `~/.codex/generated_images/` até seleção. Sem integração à main/tag.

## Início

- Docs-first: README, arquitetura, direção de arte, ADRs e guia de peças lidos.
- Raiz: letreiro, ônibus, câmera/controle, integração e revisão nativa.
- Arte: moradores, caramelo e porta, com PNGs transparentes e metadados.
- Áudio: manhã, exterior, fade do trânsito e cues de cão/porta.
- Estado da vizinhança/render próprio será delegado assim que houver slot.
- Pendente: implementação, artes, integração, verificações, evidências e commits.

## Câmera, controle e leitura

- Câmera pura de 360 ticks: pipa em primeiro plano, descida, Rust e encaixe
  exato no enquadramento jogável. Combate e ações ficam retidos; ambiente
  avança. Avançar na tomada libera exploração; retry acordado dispensa câmera.
- Letreiro usa Barlow Condensed SemiBold, mipmaps e filtro trilinear; nome
  aumentado para 18 px. Ônibus de 440×129 px, com silhueta original preservada.
- `cargo test --lib adventure::story`: 14/14; `adventure::arrival`: 1/1.
  Testes antigos ajustados para passar pela chegada antes de exercer combate.
- Build de borrow-adventure aprovado. Captura `.git/neighbourhood-review/arrival-1`
  inspecionada: início na pipa, descida pela mercearia, revelação de Rust,
  entrega de controle e ônibus inteiro. Texto legível, sem salto de enquadramento.
- Próximos passos: integrar moradores/cão/porta e áudio, validar entrada/oclusão
  e fechamento, matriz final e vídeos com som nativo.

## Integração da vizinhança e do som

- `f90a0d1` preserva a primeira etapa (câmera/controle/ônibus/letreiro).
- Quatro PNGs novos com alpha real e recortes independentes: moradores,
  lojista, caramelo e porta; três identidades humanas em quatro instâncias.
  Catálogo conserva escala uniforme por identidade e pivôs em todos os frames.
- Neighborhood deriva tudo de AmbientState: visitantes entram até tick 251;
  lojista puxa no 270 e porta toca o chão no 330. Cão late no 8 e foge no 24.
  Entrada por recorte geométrico; painel desce sem esticar. Tirante liga
  o gesto das mãos à borda enquanto o lojista é coberto.
- A primeira inspeção `.git/neighbourhood-review/shelter-1` confirmou fuga,
  entrada e porta. Ajustada uma fresta à direita: portal final (603,355),
  97×122, catalogado sem regravar o PNG. Limite do teste de continuidade
  atualizado para a velocidade de entrada; três testes do modelo passaram.
- Letreiro final usa uma linha completa a 18 px: BAR E MERCEARIA CASA NOSSA.
- Áudio: seis WAVs originais; ar/pássaros da manhã, ar e trânsito externo,
  latido, rolo e contato da porta. 17 testes de áudio e 47 do mixer aprovados;
  gerador reproduz exatamente o PCM, mix offline sem clipping.
- Prévia contínua final `.git/neighbourhood-review/preview`: manhã natural,
  saída, câmera, exploração, EP e abrigo. 30,73 s/922 frames, cinco checks
  aprovados, som nativo em sink Pulse dedicado. Sem F12, pausas ou cortes.
- Staging Linux: 248 assets verificados; 14 testes de pacote passaram.
  Relatório em `.git/cinematic-neighbourhood-review/street-linux-stage-check.json`.
- Captura funcional `.git/neighbourhood-review/native-1`: 24/24 checks,
  2258 registros, 14 imagens e vídeo de 62 s. Sem falha ou ajuste no harness.
  Câmera contínua em 370 amostras; trajetos em 1077; cena evacuada e porta
  fechada em 1311 amostras entre os ticks de reação 361–2039. Conferidos
  letreiro, escala do ônibus, entrada atrás do batente e porta sem fresta.
- Matriz final: fmt e Clippy estrito aprovados; 461 testes conjuntos,
  395 de luta, 67 de aventura e três de core. Dois testes preexistentes de
  dispositivo de áudio ignorados. Fronteiras: 56; pacote: 14; mixer: 47.
  Logs e relatório: `.git/neighbourhood-review/validation/verification.json`.
  Código/assets e hash do binário de captura permaneceram iguais na matriz.
- Áudio nativo conferido: oito cues presentes, pássaros/manhã e trânsito
  calmo correlacionados, trânsito ausente depois da evacuação. Pico 0,238,
  sem clipping. Prévia com 922 frames preservados, 30,73 s, som do sink
  dedicado convertido para AAC, sem ajuste de ganho ou remontagem dos sons.
- Revisão independente do diff não encontrou bugs materiais de integração.
- Em andamento: consolidação das evidências, links e commits finais.
