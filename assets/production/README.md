# Produção de sprites por ação

Arte raster gerada com a ferramenta integrada `imagegen`, com referência fixa por
personagem. Rust, Duke/Java, Go, C, Python e C++ concluíram o refinamento e a verificação
visual/funcional: **120 clips, 378 quadros selecionados e dez golpes por personagem**.
Os [laudos e vídeos atuais](../candidates/README.md) delimitam as evidências e as
ressalvas de acabamento. Os placeholders originais permanecem em `assets/placeholder/`.

O **novo Go semirrealista foi concluído e verificado**, com 20 clips / 60 quadros.
O [novo master](go/reference/master.png) e seu [contrato de identidade](go/reference/README.md)
substituem a identidade rejeitada pelo usuário. A pasta `go-cartoon-archive/`
preserva integralmente a produção anterior. O [laudo final](go/finalization-review.md)
inclui a correção de anatomia do salto e sua recaptura no jogo e no Studio.

Os recortes `reference/master-existing.png` dos outros cinco personagens são
**reaproveitados** das referências originais, não são nova arte.

Cada personagem usa:

- `reference/master-existing.png`: referência permanente de identidade; o novo Go usa `reference/master.png`;
- `<ação>/source-vN.png` e `prompt*.txt`: geração original e prompt, inclusive
  variantes rejeitadas quando ajudam a explicar a escolha;
- `<ação>/action.json`: seleção explícita dos desenhos, retângulos, pivôs e tempos;
- `<ação>/keyed*.png`: preparação do alpha, separada da fonte original;
- `review-plan.json`: escala uniforme por ação e decisão de revisão;
- `<ação>/prepared.png`: folha da ação na resolução de runtime;
- `production.json`: mapeamento explícito consumido pelo exportador.

Os PNGs de trabalho são editáveis como raster, sem camadas de Aseprite inventadas.
Uma ação pode precisar de nova geração; a ordem de poses nunca é inferida pela
posição na folha. Rust reaproveita a engrenagem após limpeza de alpha, enquanto C e
C++ conservam os seus projéteis. Duke recebeu [novos grãos](duke/projectile/README.md)
e Python recebeu um novo data stream, pois o PNG anterior continha um recorte da
personagem caída; sua fonte e preparação ficam em `python/projectile/`. Origem,
colisão e regras foram preservadas. Go reaproveita o canal cyan separado, validado na emissão do primeiro quadro e nas duas orientações.

## Alpha e escala

As primeiras gerações do Rust devolveram RGB com xadrez/branco pintado, mesmo
quando foi pedido alpha. A tentativa de extração pela ferramenta também retornou
RGB. Após autorização explícita do usuário, a preparação passou a remover esse
fundo localmente. As gerações seguintes usam magenta sólido e passam por remoção
da cor e de sua contaminação nas bordas. O runtime recebe PNG RGBA; não há shader
de chroma key nem alteração do renderer para remover fundo.

`remove_generated_checkerboard.py` remove o fundo claro conectado à borda e sua
franja neutra. `remove_generated_magenta.py` remove o matte magenta e descontamina
as bordas; `--rust-contour` é específico à paleta laranja do Rust e **não deve ser
usado em Duke, Go ou outros personagens com vermelho/azul legítimo**. Inspeção em
fundos claros e escuros continua obrigatória: alpha presente não garante recorte
bom. Não se deve passar esses filtros cegamente sobre os placeholders.

As fontes ficam na resolução gerada. A preparação aplica uma escala uniforme
declarada por ação e preserva o ponto de apoio de cada desenho. O atlas candidato
usa `scale = 1.0`, com humanoides perto de 268 px de altura em idle, evitando atlas
enormes e escalas físicas alteradas para compensar proporções. Corpo, dano,
alcance, startup, atividade, recovery e origem de projétil continuam no baseline.

## Reconstrução

Requer Python, Pillow e NumPy. Depois de preparar o alpha e revisar as ações:

```bash
python3 tools/art/prepare_reviewed_actions.py assets/production/rust/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/rust/production.json
python3 tools/art/render_sprite_review.py assets/candidates/rust/rust-fighter.sprite.json
```

O exportador entrega um PNG+JSON por personagem em `assets/candidates/`, os
quadros individuais em `<ação>/frames/` e um registro de proveniência. Os GIFs de
review respeitam escala/pivô/duração do runtime, exibem ambas as orientações e
acrescentam uma pausa de 400 ms ao fim dos clips não loopáveis apenas para exame.
Essa pausa não faz parte dos dados do jogo. Os fundos e textos dos previews nunca
entram no PNG runtime.

O jogo carrega os conjuntos revisados completos por padrão; `BORROW_FIGHTERS_SPRITE_CANDIDATES=1` continua sendo uma seleção explícita e `0` permite comparar os placeholders.
Candidatos parciais podem ser inspecionados pelo Viewer/Studio; em partidas, o
carregador exige os 20 nomes de clips implementados e mantém o placeholder se faltar algum.
`combat_manifest` conserva o comportamento anterior mesmo quando o desenho muda.

A cobertura e as limitações verificadas ficam em
[docs/19-sprite-production-coverage.md](../../docs/19-sprite-production-coverage.md).
