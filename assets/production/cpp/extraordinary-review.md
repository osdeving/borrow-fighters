# C++ — Undefined Bazooka e reações de impacto

A rodada preserva a identidade adulta, blusa branca, calça e botas pretas/douradas,
luvas, cabelo ondulado e bolsa circular. Todas as poses e efeitos novos vieram da
ferramenta integrada imagegen. Os sprites da rodada anterior permanecem nas fontes
versionadas e no [snapshot](snapshots/2026-09-08-before-extraordinary/production.json).

O atlas principal tem **25 clips / 88 quadros**. Esta rodada seleciona:

| Ação | Quadros | Tempo | Sequência |
| --- | ---: | ---: | --- |
| `signature_special` | 8 | 1800 ms | Bolsa, retirada da bazuca, ombro, mira/disparo baixo, acompanhamento, recuo/fumaça, guardar arma, postura pronta |
| `throw` | 6 | 800 ms | Alcançar, capturar, levantar/girar, soltar para trás, acompanhar, postura pronta |
| `thrown` | 4 | 880 ms | Capturada, invertida, cambalhota, descida |
| `launched` | 4 | 780 ms | Impacto no queixo, subida arqueada, ápice, descida |
| `heavy_hit` | 3 | 370 ms | Recuo forte, tronco dobrado, recuperação com pés apoiados |

O especial divide os ticks 0..31/32..54/55..107 em preparação/atividade/recuperação.
O arremesso divide0..9/10..21/22..47 em preparação/captura/recuperação; seu relógio
visual avança durante os12 ticks de captura. A reação `thrown` mantém o quadro 0
por 200 ms e o voo entra no quadro invertido. O tempo físico continua no combate.

O especial usa escala uniforme 268/333 por ação. Na primeira pose ativa a boca da
arma está em(1698,325), pivô(1465,359), projeção(+187,52;−27,36)world em relação ao
centro/chão. A origem física foi coordenada com o código para sair dessa boca.
O [atlas de efeitos](../../candidates/cpp/cpp-signature-fx.sprite.json) fornece
foguete e explosão próprios, com seis quadros e escala/pivôs explícitos.

Foram rejeitados: a primeira folha de atores com efeitos cruzando células; a
explosão alta da primeira fonte, substituída por explosões largas e baixas. A fonte
v2 de efeitos contém uma alternativa extra no alto; os quatro impactos escolhidos
são explicitamente os quatro desenhos inferiores. A seleção nunca depende de
contagem automática de células.

O alpha usa a autorização de [produção](../README.md#alpha-e-escala). Depois do
key magenta, [cleanup_red_matte.py](cleanup_red_matte.py) remove apenas resíduos
vermelhos junto à transparência; cores internas e azul permanecem. As primeiras
preparações também foram preservadas. A escala permanece uniforme por ação;
pivôs de voo registram cintura/centro e o runtime aproxima a base na descida.

A revisão estática em fundos claro/escuro está em
[especial e efeitos](../../candidates/cpp/review/extraordinary/signature-and-effects.png)
e [reações](../../candidates/cpp/review/extraordinary/reactions.png).
`cargo test --test sprite_candidates` passou após a exportação dos novos clips,
inclusive cada tick das fases. A [revisão runtime](../../candidates/cpp/review/extraordinary/runtime-review.json)
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

O foguete sai no tick 62 do showcase e toca o chão no 74, gerando a explosão
que aplica 24 de dano e lança o adversário. O registro normal tem 259 PNGs para o
vídeo a 60 fps; a direção espelhada também foi inspecionada.
