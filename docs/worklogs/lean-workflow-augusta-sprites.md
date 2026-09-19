# Workflow enxuto e Augusta 2D

- Branch: `feat/lean-workflow-augusta-sprites`, base `27ca4bb` (Augusta 2D).
- Preservado: `feat/augusta-blender-actors` em `453fcce`; nenhum reset/remoção.
- Pedido: primeiro reduzir workflow conforme anexo; depois corrigir caminhada
  e corrida dos NPCs com sprites completos, preservando a cena 2D.

## Workflow concluído — `08bcf9f`

- Regras/skills encurtadas; docs só quando necessárias; fast/domínio/full via
  `./dev`. CI ampla preservada. Viewer retirado; Studio multi-atlas corrigido.
- Builder existente consolidado em `tools/art/art.py`, receitas Python/C++,
  fontes preservadas; candidatos em assets-generated, integração explícita.
- Contrato Rust gera schema/TS; sem novas dependências do jogo.
- Check fighting passou (368 testes, 2 ignorados); Studio build/Clippy/3 testes;
  builder 7 testes e comparação de 176 recortes passaram. Schema, fronteiras,
  skills e links/YAML passaram. Log: `/tmp/borrow-workflow-fighting.log`.

## Augusta concluída — `ff23ee1`

- 82 quadros inteiros da multidão (5 caminhadas e 8 corridas), mais 18 quadros
  de Julia/cafetão/seguranças. Originais em `assets-src/adventure/augusta/`.
- `motion_entries` seleciona sprites por distância; `frame_motion.rs` removido.
  Nenhum `draft-motion-*` permanece. Pivots/escala fixos, sem deformar membros.
- Câmera, carros, cenário, trajetórias e diálogo preservados. C++ conserva seu
  rig e as atuações estacionárias/contato mantêm os assets anteriores.
- `./dev check adventure`: fmt, Clippy e 213 testes passaram; log local
  `/tmp/borrow-augusta-adventure.log`. Builder: 7 testes; 17 receitas validadas.
- Galeria nativa concluída: 360 quadros/12 s, oito figurinos nos dois sentidos;
  inspecionados escala, alpha, apoios e fases. Vídeo local:
  `assets-generated/reviews/augusta-sprites-review/crowd-motion.mp4`.
  O primeiro enquadramento da galeria cortava os pés nas bordas; somente sua
  escala de apresentação foi corrigida, sem mudar os sprites do jogo.
- NPCs: folhas de poses e pilotos de 120 quadros em
  `assets-generated/reviews/{julia,broker,security}-run/` revisados.
- Docs 38/39 atualizados; validação dos dois documentos passou.

## Revisão final / retomada

- Captura nativa única terminou sem erro: 8.238 amostras, estado Complete no
  frame 8.058, checkpoint complete no final 8.237. Vídeo 1280×720/30 fps,
  137,3 segundos e 41 snapshots; corrida dos três NPCs registrada.
- Evidências locais em `assets-generated/reviews/augusta-scene-final/` e na
  galeria acima. Fontes e runtime versionados; capturas/intermediários ficam
  locais para evitar adicionar relatórios e vídeos a cada ajuste de arte.
- Comando usado: `./dev story --start augusta --hidden --review assets-generated/reviews/augusta-scene-final --frames 24000`.
- Para jogar: `./dev story --start augusta`.
- Não houve push, merge ou publicação. Ao retomar, consultar status/log e este
  checkpoint; não repetir verificações aprovadas sem outra alteração relevante.
