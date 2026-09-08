# Python — acabamento e verificação de 07/09/2026

O conjunto mantém **20 ações, 62 quadros e os dez golpes**. Nesta continuação foram refinados caminhada, chute em pé e rasteira, com identidade original preservada e verificação no renderer nativo. O agente principal revisou a caminhada, o chute e uma partida normal C×Python. As outras ações foram reaproveitadas; os registros anteriores continuam no [histórico de revisão](review.md).

| Ação | Seleção e resultado | Tempos preservados |
| --- | --- | --- |
| walk | [v8](walk/source-v8.png), contatos com sobreposição oposta das coxas, braços alternando e saída do pé traseiro. Quatro corpos completos, sem duplicar o contato. As duas poses de saída do pé não são declaradas falsamente como passagem à frente. | 100/100/100/100 ms; escala 268/600. |
| kick | [v5](kick/source-v5.png), preparação do joelho, contato com o sapato preto na faixa ativa, recolhimento e volta à guarda. O contato anterior dependia da canela e deixava o sapato abaixo da caixa. | 133/117/83/83 ms; recuperação 166 ms dividida em duas poses, escala 268/495. |
| sweep | [v2](sweep/source-v2.png), apoio de mão/joelho, sola ativa menos elevada, recolhimento baixo e meia subida. A recuperação deixou de saltar diretamente da posição ajoelhada para a postura em pé. | 150/133/100/100 ms; recuperação 200 ms dividida em duas poses, escala 0,432258. |

As [medidas de contato](../../candidates/python/review/refinement-2026-09-07/contact-review.json) usam pontos manuais na fonte, os pivôs explícitos e uma escala uniforme por ação. O sapato do kick mapeia aproximadamente para (+139,1;−53,6), dentro da caixa original. O centro da sola da rasteira mapeia para (+167,7;−127,9), com a ponta próxima da borda superior da caixa. O renderer confirma esses contatos. Nenhuma caixa foi movida para encaixar a arte; a faixa elevada original da rasteira foi preservada.

## Fontes e identidade

A [referência permanente](reference/master-existing.png) e o [idle polido](idle/source-v1.png) fixaram mulher adulta esguia, cabelo preto longo, camisa branca completa, saia preta pregueada, fivela dourada, sapatos pretos de salto e tira e uma cobra azul/amarela. A pose mais lateral da caminhada acompanha o giro discreto do corpo e dos braços; a roupa e as proporções continuam as da Python original.

Walk v4–v7 repetiram contato ou cruzaram as pernas incorretamente. Na v8, a folha de C foi usada **somente como referência das poses das pernas**; rosto, roupa, livro e demais elementos visuais de C não foram importados. A geração final continua usando o master e o idle de Python. No kick, v2 ficou alto, v3 baixo e v4 acima da faixa; v5 corrigiu a posição do pé. Todos os prompts e PNGs originais permanecem por versão.

Os [metadados anteriores](review-plan-before-finalization.json) e os `action-before-finalization.json` preservam o mapeamento anterior. O [índice das fontes e hashes](../../candidates/python/review/refinement-2026-09-07/art-source-index.json) identifica as seleções. Recortes, pivôs, fases e tempos estão nos `action.json`. Não há montagem de membros, desenho de poses por código, ajuste físico, alteração de origem do projétil ou escala individual para disfarçar diferenças anatômicas.

O [helper local de alpha](prepare-finalization-alpha.py) usa o keyer genérico sem `--rust-contour`. A folha do kick continha magenta escuro residual que o primeiro keyer deixou semitransparente; uma máscara baseada na cor da fonte retirou esse fundo, ausente da identidade. A fonte RGB permaneceu intacta. Todos os quadros novos foram inspecionados sobre fundos claro/escuro e nas duas orientações no [painel de alpha](../../candidates/python/review/refinement-2026-09-07/alpha-two-facings.png). O projétil de dados azul/amarelo já produzido foi reaproveitado.

## Execução nativa

O [Lab filtrado](../../candidates/python/review/refinement-2026-09-07/lab/capture-report.json) concluiu 38 PNGs de kick, sweep e idle, com manifesto carregado idêntico ao candidato. Foram conferidos preparação, contatos nos ticks 8/12 do kick e 9/12 da rasteira, as duas recuperações e volta ao idle. O [painel de transições](../../candidates/python/review/refinement-2026-09-07/lab/native-transition-crops.png) conserva os pixels das capturas reais. O Lab usa orientação direita; o World complementa a conferência dos dois lados.

O [World completo v2](../../candidates/python/review/refinement-2026-09-07/world/capture-summary.json) concluiu 650 capturas e 1845 estados. Ambos os jogadores executaram os dez golpes, nas orientações opostas, e voltaram apoiados ao idle após cada um. Foram inspecionados os quatro desenhos da caminhada avançando/recuando, retorno ao idle, kick/sweep, transições dos dois golpes aéreos e os dez contatos/emissão. O [índice](../../candidates/python/review/refinement-2026-09-07/world/evidence-index.json) identifica 38 PNGs selecionados e oito painéis. A vida final 89×89 resulta dos projéteis normais; não houve reposição de HP. Manifesto e atlas permaneceram iguais durante toda a execução.

O [vídeo](../../candidates/python/review/refinement-2026-09-07/world/runtime-motion.mp4) tem 1845 quadros a 60 fps e 30,750 s, validados por ffprobe. Ele segue os tempos registrados, sem interpolação nem pausa extra. A amostragem conserva a imagem anterior entre ticks sem novo PNG; 1845 estados não significam 1845 capturas distintas. A conferência cobre sequências e tempos registrados; não é um teste interativo de teclado.

A primeira execução concorrente do World terminou por SIGTERM, código 143, com 434 PNGs até o soco aéreo e sem relatório final. Foi preservada como [incompleta](../../candidates/python/review/refinement-2026-09-07/world/discarded-capture.json), sem atribuir uma causa não verificada. A v2 foi repetida isoladamente e concluiu; somente ela sustenta a evidência de cobertura completa.

O agente principal abriu uma [partida nativa C×Python](../../candidates/python/review/refinement-2026-09-07/native-match-result.png), sem variável de opt-in, que terminou 2×0. Vitória/derrota permaneceram apoiadas. Os [hashes e comando](../../candidates/python/review/refinement-2026-09-07/native-match-assets.json) identificam os candidatos usados. O Sprite Studio foi exercitado no piloto Rust; não se declara uma sessão adicional de Python no editor.

## Reprodução

```sh
python3 tools/art/prepare_reviewed_actions.py assets/production/python/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/python/production.json
python3 tools/art/render_sprite_review.py assets/candidates/python/python-fighter.sprite.json
env -u WAYLAND_DISPLAY BORROW_FIGHTERS_SPRITE_CANDIDATES=1 target/debug/examples/capture_sprite_review python target/art/python-finalization-lab-new kick,sweep
env -u WAYLAND_DISPLAY BORROW_FIGHTERS_SPRITE_CANDIDATES=1 target/debug/examples/capture_motion_review target/art/python-finalization-world-new python --attacks --debug-boxes
python3 tools/art/encode_motion_review.py target/art/python-finalization-world-new
```

Os diretórios novos preservam evidências anteriores. Combate baseline, tempos, dano, trajetória física e origem do projétil continuam preservados.
