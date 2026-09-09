# C — acabamento e verificação de 07/09/2026

O conjunto mantém **20 ações, 63 quadros e os dez golpes**. Nesta continuação foram refinados caminhada, soco forte com livro e rasteira. Os três refinamentos foram aceitos pelo agente principal após a revisão dos contatos e da partida nativa. As outras ações foram reaproveitadas da produção anterior; seus registros permanecem no [histórico de revisão](review.md).

| Ação | Seleção e resultado | Tempos preservados |
| --- | --- | --- |
| walk | [v3](walk/source-v3.png), quatro corpos completos com contato próximo, saída do pé distante, contato oposto e passagem próxima. A sobreposição das coxas alterna, o joelho deixou de marchar alto e o livro permanece no mesmo braço. [Laudo da ação](walk/review-v3.md). | 100/100/100/100 ms; escala 268/630. |
| punch_heavy | [v3](punch_heavy/source-v3.png), avanço longo com o livro seguro por duas mãos, retração ainda em avanço e volta à guarda. A borda das páginas alcança a região ativa. | 200/150/140/143 ms; recuperação 283 ms dividida em duas poses, escala 268/520. |
| sweep | [v3](sweep/source-v3.png), apoio baixo com livro contínuo, sola ativa elevada, recolhimento da perna e meia subida. A recuperação evita a troca direta de ajoelhado para idle. | 200/150/125/125 ms; recuperação 250 ms dividida em duas poses, escala 268/630. |

As [medidas de contato](../../candidates/c/review/refinement-2026-09-07/contact-review.json) usam pontos manuais na fonte, os pivôs explícitos e uma escala uniforme por ação. O centro da borda das páginas mapeia aproximadamente para (+148,4;−140,2); o centro da sola da rasteira para (+151,0;−117,8). Ambos estão nas caixas originais. A borda superior do livro avança cerca de 11 px acima da caixa e a ponta elevada do tênis cerca de 18 px; o contorno inteiro não foi declarado contido. As capturas confirmam contato legível do objeto/sola, em vez de depender apenas de braço/canela. A caixa da rasteira mantém sua faixa elevada original; não foi deslocada para o chão.

## Fontes e identidade

A [referência permanente](reference/master-existing.png), o idle polido e a ação anterior orientaram as gerações pela ferramenta integrada imagegen. Foram preservados homem idoso esguio, cabelo branco/cinza, bigode, jeans claro/escuro, camiseta branca, cinto e tênis claros, além do único livro azul com C. Letras continuam espelhadas no lado esquerdo conforme o renderer existente.

Walk v2 repetia a passada; v4/v5 alteraram o apoio e v6 produziu outra pose. Sweep v2 posicionou a sola baixa demais; v3 corrigiu a direção da perna. Heavy v4/v5 voltaram ao contato baixo; v6 elevou o livro novamente. Todas as fontes e todos os prompts permanecem por versão. Cada folha é arte real gerada, sem montagem de membros ou desenho de poses por código.

Os [metadados anteriores](review-plan-before-finalization.json) e os `action-before-finalization.json` de cada ação preservam o mapeamento anterior. O [índice de fontes e hashes](../../candidates/c/review/refinement-2026-09-07/art-source-index.json) identifica as três seleções. Recortes, pivôs, fases e tempos estão nos `action.json`; nenhum campo de hitbox, hurtbox, física ou origem de projétil foi acrescentado. O projétil binário anterior foi reaproveitado.

O [helper local de alpha](prepare-finalization-alpha.py) chama o keyer genérico sem `--rust-contour` e remove magenta escuro residual, cor ausente da identidade de C. Fontes RGB intactas. Foram inspecionados todos os quadros novos sobre fundos claro/escuro e nas duas orientações no [painel de alpha](../../candidates/c/review/refinement-2026-09-07/alpha-two-facings.png).

## Execução nativa

O [Lab filtrado](../../candidates/c/review/refinement-2026-09-07/lab/capture-report.json) concluiu 33 PNGs de HP, sweep e idle; o manifesto carregado é idêntico ao candidato. Foram conferidos início, quadros ativos 12/18, as duas recuperações e volta ao idle. O [painel de transições](../../candidates/c/review/refinement-2026-09-07/lab/native-transition-crops.png) contém recortes dos PNGs do renderer real sem alteração de pixels. O Lab usa a orientação direita; não é prova das duas orientações sozinho.

O [World completo](../../candidates/c/review/refinement-2026-09-07/world/capture-summary.json) concluiu 625 capturas e 1845 estados. Ambos os jogadores executaram os dez golpes nas orientações opostas e voltaram apoiados ao idle após cada golpe. Foram inspecionados avanço/recuo com os quatro desenhos, retorno ao idle, transições de HP/sweep e dos dois golpes aéreos, e os dez contatos/emissão. O [índice](../../candidates/c/review/refinement-2026-09-07/world/evidence-index.json) identifica 38 PNGs selecionados e oito painéis. A vida final 96×96 resulta dos projéteis normais; não houve reposição de HP. Manifesto e atlas permaneceram idênticos durante a captura.

O [vídeo](../../candidates/c/review/refinement-2026-09-07/world/runtime-motion.mp4) tem 1845 quadros a 60 fps, duração 30,750s validada por ffprobe. É uma montagem das capturas GPU nos tempos de simulação, sem interpolação nem pausa extra. A amostragem conserva a imagem anterior entre ticks sem novo PNG; os 1845 estados não representam 1845 capturas distintas. As sequências e os tempos foram revisados; isso não substitui teste interativo de teclado.

O agente principal também abriu uma [partida nativa C×Python](../../candidates/c/review/refinement-2026-09-07/native-match-result.png), sem variável de opt-in, terminando 2×0. Vitória/derrota ficaram apoiadas e o livro permaneceu inteiro junto da borda. [Hashes e comando](../../candidates/c/review/refinement-2026-09-07/native-match-assets.json) identificam os candidatos usados. O Sprite Studio foi exercitado no piloto Rust; não se declara uma sessão adicional de C no editor.

## Reprodução

```sh
python3 tools/art/prepare_reviewed_actions.py assets/production/c/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/c/production.json
python3 tools/art/render_sprite_review.py assets/candidates/c/c-fighter.sprite.json
env -u WAYLAND_DISPLAY BORROW_FIGHTERS_SPRITE_CANDIDATES=1 target/debug/examples/capture_sprite_review c target/art/c-finalization-lab-new sweep,punch_heavy
env -u WAYLAND_DISPLAY BORROW_FIGHTERS_SPRITE_CANDIDATES=1 target/debug/examples/capture_motion_review target/art/c-finalization-world-new c --attacks --debug-boxes
python3 tools/art/encode_motion_review.py target/art/c-finalization-world-new
```

Os diretórios novos evitam sobrescrever evidência anterior. A revisão é visual/funcional; tempos, dano, alcance físico, combate baseline e origem do projétil continuam preservados.
