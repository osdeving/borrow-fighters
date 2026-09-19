# Workflow enxuto e Augusta 2D

- Branch: `feat/lean-workflow-augusta-sprites`, base `27ca4bb` (Augusta 2D).
- Preservado: `feat/augusta-blender-actors` em `453fcce`; nenhum reset/remoção.
- Pedido: primeiro reduzir workflow conforme anexo; depois corrigir caminhada
  e corrida dos NPCs com sprites completos, preservando a cena 2D.

## Checkpoint — workflow concluído

- Regras/skills encurtadas, docs só quando necessárias, fast/domínio/full via
  `./dev`; CI ampla preservada. Viewer retirado; Studio multi-atlas corrigido.
- Builder existente consolidado em `tools/art/art.py`, receitas Python/C++,
  fontes preservadas; candidatos em assets-generated, publicação explícita.
- Contrato Rust gera schema/TS; sem novas dependências do jogo.
- `./dev check fighting`: fmt, Clippy e testes passaram (log local
  `/tmp/borrow-workflow-fighting.log`). Testes focais Viewer 93; Studio build,
  Clippy e 3 testes; builder 7 testes e comparação de 176 recortes passaram.
- `./dev schema check`, `./dev art validate --all`, fronteiras (Python3.13),
  validação das 4 skills e links/YAML passaram. Nenhuma matriz repetida.

## Próximo — Augusta 2D

Substituir deformação de locomoção em painted_crowd::draw_person (8 figurinos,
5 caminhadas/8 corridas) e interceptação frame_motion em actors.rs (Julia,
broker, security). Sprites inteiros compartilham cinema/lateral, sem mexer
câmera/cenário/trajetórias/carros. C++ tem rig próprio e fica fora desta correção
específica dos NPCs. Gerar/revisar lote pequeno antes de ampliar o elenco.

Retomada: status/log + este checkpoint; preservar alterações dos agentes.
