# Python — import antigravity e reações de impacto

A rodada preserva a identidade adulta, cabelo preto comprido, blusa branca, saia
preta plissada, sapatos de salto e serpente azul/dourada. Todas as poses e efeitos
novos vieram da ferramenta integrada imagegen. A produção anterior permanece nas
fontes versionadas e no [snapshot](snapshots/2026-09-08-before-extraordinary/production.json).

O atlas principal tem **25 clips / 88 quadros**. Esta rodada seleciona:

| Ação | Quadros | Tempo | Sequência |
| --- | ---: | ---: | --- |
| `signature_special` | 8 | 1667 ms | Gesto/import, serpente cresce, espirais, levitação, direcionar, flutuação com pernas soltas, pouso, postura pronta |
| `throw` | 6 | 800 ms | Alcançar, capturar, levantar/girar, soltar para trás, acompanhar, postura pronta |
| `thrown` | 4 | 880 ms | Capturada, invertida, cambalhota, descida |
| `launched` | 4 | 780 ms | Impacto no queixo, subida arqueada, ápice, descida |
| `heavy_hit` | 3 | 370 ms | Recuo forte, tronco dobrado, recuperação com pés apoiados |

O especial divide os ticks 0..25/26..58/59..99 em preparação/atividade/recuperação.
O arremesso divide0..9/10..21/22..47 em preparação/captura/recuperação; seu relógio
visual avança durante os12 ticks de captura. A reação `thrown` mantém o quadro 0
por 200 ms e o voo entra no quadro invertido. O tempo físico continua no combate.

O especial usa escala uniforme 268/371. O corpo mantém apenas as espirais presas
à personagem: o vortex destacado da primeira fonte foi removido por imagegen na
v2. O [atlas de efeitos](../../candidates/python/python-signature-fx.sprite.json)
contém dois quadros próprios de serpente vertical para loop e quatro quadros de
dispersão. A fonte circular inicial foi substituída por uma serpente espiral alta,
com cerca de 190×427world, compatível com o campo físico 200×427world. A dispersão
colapsa ao redor da metade do campo; pivôs ainda usam a âncora no chão.

A física controla toda a levitação. Os pivôs virtuais compensam pernas recolhidas;
na reação de uppercut a cintura registra −173world, como no idle, em vez do −160world
usado pelo C++. A saia permanece fechada e a serpente acompanha as rotações.

O alpha usa a autorização de [produção](../README.md#alpha-e-escala). Depois do
key magenta, [cleanup_red_matte.py](cleanup_red_matte.py) remove apenas resíduos
vermelhos junto à transparência, preservando o azul da serpente. As primeiras
preparações permanecem disponíveis. Toda escala é uniforme por ação.

A revisão estática em fundos claro/escuro está em
[especial e efeitos](../../candidates/python/review/extraordinary/signature-and-effects.png)
e [reações](../../candidates/python/review/extraordinary/reactions.png).
`cargo test --test sprite_candidates` passou após a exportação dos novos clips,
inclusive cada tick das fases. A [revisão runtime](../../candidates/python/review/extraordinary/runtime-review.json)
concluiu 14 casos por personagem (oito ataques próprios e seis como vítima), nas
duas orientações. Todos os 25 quadros destas cinco ações foram observados nos
clips reais. Doze PNGs originais selecionados e os JSONs completos ficam com o
relatório; as capturas completas temporárias permanecem nos diretórios registrados.
O [audit](extraordinary-audit.json) registra hashes e contagens. Arte integrada e
verificada para este goal, com lançamento, troca de lado, contato e chão reais.

A correção [lift v2](throw/source-lift-v2.png) move somente os braços do quadro 2
do arremesso para cima e para a frente: as mãos agora sustentam pernas/torso da
vítima no meio da captura. [A seleção](throw/select_lift_v2.py) preserva os pixels
dos outros cinco quadros, conferidos por hash. Os quatro casos de arremesso foram
recapturados após a correção.

O contexto final começa a 230 unidades base e mantém a aproximação real: o
vortex aparece no tick 56, mostra os dois quadros de loop antes do contato no 64
e então dispersa/lança. O registro normal tem 259 PNGs para o vídeo a 60 fps.
