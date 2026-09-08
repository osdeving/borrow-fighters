# C++ — revisão do candidato

**Estado:** 20 ações e 61 quadros exportados; fontes, alpha, prévias em ambos os sentidos e captura real do Combat Lab revisados. O Lab carregou o candidato e o manifesto de combate baseline do World. Aprovação artística final permanece pendente.

## Entrega e rastreabilidade

- [Manifest do candidato](../../candidates/cpp/cpp-fighter.sprite.json) e [atlas 3160×3064](../../candidates/cpp/cpp-fighter-atlas.png).
- [Visão geral nos dois sentidos](../../candidates/cpp/review/overview.png), GIF por ação e quadros em tamanho real na mesma pasta.
- [Plano de produção](production-plan.md), [plano de escalas](review-plan.json), [entrada do exporter](production.json) e [auditoria estática](static-audit.json).
- [Referência original](reference/master-existing.png) e [idle fixo gerado](idle/source-v1.png). Cada chamada posterior recebeu as duas referências.
- 31 fontes geradas e prompts versionados por ação; cada fonte foi gerada pelo imagegen integrado, uma ação por folha. Nenhuma pose foi construída com código.

Fontes RGB em magenta foram preservadas. O recorte alpha usa o helper genérico autorizado, sem `--rust-contour`. Nos11 estados deste agente, remoção de pequenos componentes desconectados é registrada em [alpha-cleanup-own-actions.json](alpha-cleanup-own-actions.json). Nos seis ataques terrestres, retângulos terminam em y736 para excluir um pixel residual isolado em (0,767), sem mudar os pixels da fonte.

## Seleção e revisão

A identidade segue mulher adulta de cabelo castanho/dourado, camisa branca curta amarrada, calça preta, botas e luvas com detalhes dourados. Bolsa redonda e alça são contínuas. Nenhuma ação usa a identidade de Python.

| Ação | Quadros | Duração total | Escala uniforme | Fonte selecionada |
| --- | ---: | ---: | ---: | --- |
| idle | 3 | 720ms | 0.388970 | [idle/keyed-v1.png](idle/keyed-v1.png) |
| walk | 4 | 400ms | 0.412308 | [walk/assembled-selected-v4.png](walk/assembled-selected-v4.png) |
| jump | 3 | 800ms | 0.412308 | [jump/keyed-v1.png](jump/keyed-v1.png) |
| crouch | 3 | 540ms | 0.388970 | [crouch/keyed-v2.png](crouch/keyed-v2.png) |
| block | 3 | 340ms | 0.388970 | [block/keyed-v1.png](block/keyed-v1.png) |
| crouch_block | 3 | 340ms | 0.388970 | [crouch_block/keyed-v4.png](crouch_block/keyed-v4.png) |
| hit | 3 | 320ms | 0.406061 | [hit/keyed-v1.png](hit/keyed-v1.png) |
| punch_light | 3 | 266ms | 0.388970 | [punch_light/keyed-v3.png](punch_light/keyed-v3.png) |
| punch_heavy | 3 | 550ms | 0.388970 | [punch_heavy/keyed-v1.png](punch_heavy/keyed-v1.png) |
| kick | 3 | 433ms | 0.388970 | [kick/keyed-v2.png](kick/keyed-v2.png) |
| sweep | 3 | 516ms | 0.388970 | [sweep/keyed-v2.png](sweep/keyed-v2.png) |
| overhead | 3 | 533ms | 0.388970 | [overhead/keyed-v1.png](overhead/keyed-v1.png) |
| anti_air | 3 | 450ms | 0.432258 | [anti_air/keyed-v1.png](anti_air/keyed-v1.png) |
| air_punch | 3 | 366ms | 0.388970 | [air_punch/keyed-v1.png](air_punch/keyed-v1.png) |
| air_kick | 3 | 433ms | 0.412308 | [air_kick/keyed-v1.png](air_kick/keyed-v1.png) |
| throw | 3 | 383ms | 0.388970 | [throw/keyed-v1.png](throw/keyed-v1.png) |
| special | 3 | 300ms | 0.479428 | [special/keyed-v2.png](special/keyed-v2.png) |
| spawn | 3 | 540ms | 0.394698 | [spawn/keyed-v1.png](spawn/keyed-v1.png) |
| victory | 3 | 1200ms | 0.388970 | [victory/keyed-v1.png](victory/keyed-v1.png) |
| defeat | 3 | 1000ms | 0.388970 | [defeat/keyed-v1.png](defeat/keyed-v1.png) |

Os nove golpes próximos usam exatamente os limites em milissegundos derivados dos ticks inclusivos do baseline: preparação termina em `floor(active_start×1000/60)` e recuperação começa em `floor((active_end+1)×1000/60)`. A auditoria confirma todos os relógios. Não há campos `combat` novos nos 61 quadros exportados.

O salto tem três desenhos por 60/340/400ms. Pivôs virtuais 712/695/711 descontam a diferença de posição da cintura 317/300/316 na fonte, deixando-a a −395px da âncora. Os pés se dobram durante o salto; a trajetória vem do World.

O especial v2 já começa emitindo. A bolsa avança presa pela alça, com o gesto guiando a alça; não aparece uma segunda bolsa. Sua borda dourada em (814,329), âncora no chão (386,657) e escala 268/559 resultam em (+205,20;−157,25)world, contra (+205,33;−157,33) do baseline. O projétil existente permanece separado. A fonte v1 tinha alcance curto e alto e foi preservada como tentativa.

Crouch v2 dobra tronco e quadris e ocupa aproximadamente 113–119px; crouch_block v4 se comprime mais e ocupa 92–95px, ambos na escala anatômica 268/689. As tentativas anteriores mantinham o torso alto. Derrota termina ajoelhada e apoiada, evitando aumento de escala em uma queda deitada.

## Ressalvas artísticas

- A caminhada v4 seleciona os quadros 0/1/3 gerados em v2 e o contato oposto do quadro 2 gerado em v3. O arquivo montado apenas reúne recortes intactos; `source_components` registra fontes, retângulos e operação. A versão v3 inteira piorava as passagens. A alternância dos apoios melhorou, mas o contrabalanço dos braços continua discreto e o quadro regenerado tem pequena variação de cabeça/roupa. Ainda merece acabamento antes de arte final.
- Idle e defesa baixa têm variações muito sutis. A defesa baixa é significativamente mais compacta que o crouch comum; revisar a transição em jogo.
- O chute aéreo toca a região frontal/inferior da caixa. A captura confirma parte da bota dentro dela, mas o salto/contorno externo dourado se prolonga aproximadamente 15px abaixo da borda inferior; merece refinamento visual sem alterar a caixa.
- Cabelos, detalhes dourados, bolsa e fisionomia apresentam pequenas variações entre gerações. Previews mostram alpha limpo no tamanho real, mas não substituem uma aprovação artística quadro a quadro.
- O espelhamento reflete os operadores na bolsa como os demais pixels. Não existe arte independente por orientação.

## Verificação atual

Concluídos: preparação, exportação, 20 GIFs e prévias em ambos os sentidos, 61 retângulos com pixels visíveis/alpha real, pivôs dentro dos recortes, nomes únicos, 20 clips próprios e os nove relógios de contato. As fontes e o atlas permaneceram congelados durante as capturas do Lab e do World. O SHA-256 do manifest foi conferido antes/depois: `d7e731fa6307607760d7b9a455d2c451e2089b5b524004bcbe42617ea0be0255`.

A captura final usa o Combat Lab corrigido: 253 PNGs, 19 clips, `loaded_manifest_matches_candidate=true` e manifesto de combate baseline carregado. Os 20 registros capturados com hitbox ativa usam o desenho de contato próprio da ação. A inspeção dos nove golpes confirmou mão/bota atravessando a caixa ativa; a primeira emissão do especial sai junto da borda da bolsa. Crouch, defesa baixa, salto estático e derrota apoiada também foram inspecionados.

O [relatório completo](../../candidates/cpp/review/runtime-lab/capture-report.json) e o [índice de evidência](../../candidates/cpp/review/runtime-lab/evidence-index.json) acompanham 15 PNGs selecionados (~22 MiB). Os 253 PNGs originais permanecem em `target/art/cpp-combat-lab-final`; o log está em [capture-final.log](capture-final.log). O harness de Lab não cobre walk, usa orientação nativa direita e suas poses estáticas não simulam uma partida. O agente principal captura o World nos dois sentidos para caminhada, arco físico e transições; essa revisão é documentada na consolidação global.

Evidências principais: [especial emitindo](../../candidates/cpp/review/runtime-lab/special-f001-idle.png), [jab](../../candidates/cpp/review/runtime-lab/punch_light-f006-active.png), [sweep](../../candidates/cpp/review/runtime-lab/sweep-f012-active.png), [chute aéreo com ressalva](../../candidates/cpp/review/runtime-lab/air_kick-f012-active.png), [crouch](../../candidates/cpp/review/runtime-lab/crouch-f000-idle.png) e [defesa baixa](../../candidates/cpp/review/runtime-lab/crouch_block-f000-idle.png).
