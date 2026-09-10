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
  aumentado para18 px. Ônibus de440×129px, com silhueta original preservada.
- `cargo test --lib adventure::story`:14/14; `adventure::arrival`:1/1.
  Testes antigos ajustados para passar pela chegada antes de exercer combate.
- Build de borrow-adventure aprovado. Captura `.git/neighbourhood-review/arrival-1`
  inspecionada: início na pipa, descida pela mercearia, revelação de Rust,
  entrega de controle e ônibus inteiro. Texto legível, sem salto de enquadramento.
- Próximos passos: integrar moradores/cão/porta e áudio, validar entrada/oclusão
  e fechamento, matriz final e vídeos com som nativo.
