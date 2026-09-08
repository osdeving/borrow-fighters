> Histórico da produção anterior. O estado atual, as correções e a evidência de execução estão no [laudo de finalização de 07/09/2026](finalization-review.md).

# C — revisão do candidato

Produzidas as 20 ações com imagegen real, uma folha por ação, referência original fixa e idle novo em todas as chamadas seguintes. Resultado exportado: **20 clips, 61 frames, atlas 3488 × 2928, escala runtime 1,0**. O corpo em idle fica próximo de 268 px. O livro C permanece nas poses, incluindo chutes, dano, salto e derrota.

- [Manifesto candidato](../../candidates/c/c-fighter.sprite.json)
- [Panorama nas duas orientações](../../candidates/c/review/overview.png)
- [Plano de normalização](review-plan.json), [contratos por ação](production.json) e [mapa das fontes](source-layouts.json)
- [Relatório histórico do Combat Lab](../../candidates/c/review/runtime-lab-final/capture-report.json)

## Verificação realizada

Fontes, alpha e recortes explícitos inspecionados; nenhum retângulo exportado corta a silhueta. Cada ação usa uma escala uniforme, com pivôs independentes de apoio ou âncora virtual. Preparação e exportação passaram pelos utilitários `prepare_reviewed_actions.py` e `build_reviewed_sprite_atlas.py`; `render_sprite_review.py` gerou 20 GIFs e painéis estáticos com as duas orientações e fundos claro/escuro.

A captura histórica final de poses, `capture_sprite_review c`, terminou com o renderer real: **254 PNGs**, manifesto visual carregado idêntico ao candidato daquela execução e 19 ações capturadas. Walk não existe como pose no Lab; foi conferido nos previews e posteriormente no cenário World. Todos os samples ativos dos nove golpes usaram o quadro de contato `_1`, nos limites de timing existentes. O script não comprova transições de partida, trajetória real completa de salto ou movimento horizontal; essas verificações continuam no playtest global do projeto.

Na inspeção visual do Lab, livro/borda de contato entram nas caixas de jab, heavy, overhead, anti-air e soco aéreo; os chutes têm contato da sola/canela com as caixas. O primeiro desenho do especial já mostra emissão e reaproveita o bitstream original separado. A diferença horizontal estimada na fonte é de aproximadamente 1,6 px em relação ao ponto baseline; trata-se de uma medição da arte. O Lab antigo usava `Projectile::from_fighter`, sem consultar essa origem, portanto seu alinhamento aparente não comprova o comportamento no World.

A [recaptura corrigida do especial](../../candidates/c/review/runtime-baseline-special/README.md), com 15 amostras e ambos os manifestos conferidos, confirma a origem baseline e o fluxo binário na abertura do livro. Essa pendência técnica foi encerrada. Os nove golpes usam MoveSpec, e sua evidência de contato continua válida. Solas e joelhos de apoio estão presos ao chão nas poses terrestres.

## Limitações artísticas registradas

- **Crouch e crouch_block foram regenerados com o torso dobrado sobre as pernas:** o hold caiu de 190–205 px para cerca de 140–149 px, próximo da hurtbox de 128 px. Parte do cabelo continua acima e a cabeça avança além da frente física. A entrada de crouch fica um pouco mais alta por 60 ms. As caixas não foram alteradas.
- Sweep acerta visualmente com parte da canela, enquanto a sola sobe acima da caixa. A pose é uma varredura ascendente, candidata a refinamento.
- Jab e heavy usam apenas parte da área do livro dentro da caixa. Não representam preenchimento completo da área física; os desenhos de contato têm alcance menor que a extremidade máxima de algumas caixas.
- Walk tem uma passada de joelho acentuada e a calça larga dificulta distinguir a alternância de pernas. A fluidez em deslocamento precisa de playtest.
- Letras C espelham junto do personagem no runtime. A revisão usa o espelhamento existente e não altera essa regra.
- Fontes geradas ainda variam discretamente em rosto, cabelo, mãos e orientação do livro; o conjunto é **candidato**, não aprovação artística final.

## Revisões de fonte

Heavy seleciona v2 (v1 alto); special seleciona v4 (v1 alto, v2/v3 curtos); defeat seleciona v3, com recoil/ajoelhar/sustentar a derrota, após v1 ampliar a anatomia deitada e v2 alterar poses indevidas. Idle seleciona as células 0, 1 e 3 da v2, descartando a flexão exagerada da célula 2; os demais prompts mantiveram idle/source-v1 como referência de identidade. Crouch_block v2 foi uma revisão intermediária de recoil; a versão final é v3, mais baixa e com o livro protegendo o rosto desde o primeiro quadro. Crouch final usa duas poses compactas da v2.

Alpha: chroma magenta pelo helper genérico **sem `--rust-contour`**. Pequenas ilhas desconectadas de menos de 32 pixels foram removidas em uma segunda folha `keyed-vN`, com contagem em [alpha-cleanup-report.json](alpha-cleanup-report.json). Fonte RGB e primeiro resultado alpha foram preservados. Nenhum dado Rust, hitbox, hurtbox, física ou origem de projétil foi alterado neste pacote.

## Revisão final de movimento e poses baixas

A captura World percorreu entrada, contagem, avanço/recuo e um salto completo nas duas orientações. O primeiro passe revelou translação vertical embutida na fonte de jump; pivôs virtuais 695/602/633 alinham cintura/cabeça, removendo até 40 px extras de subida sem alterar física. O segundo passe concluiu 124 capturas em 585 ticks; cabeça/torso, ápice, queda e retorno ao idle foram inspecionados. O último frame aéreo tem os pés aproximadamente 16 px acima da âncora e pernas ainda dobradas; idle grounded ocorre no tick 543. Vídeo em [runtime-motion](../../candidates/c/review/runtime-motion/README.md).

Depois, crouch v2 selecionou 2 poses compactas e crouch_block v3 selecionou 3 poses com livro defendendo o rosto desde o início. O atlas passou de 62 para 61 frames. Antes da correção de origem baseline, o Lab foi repetido integralmente em `target/art/c-combat-lab-compact-final`, com 254 PNGs e 19 contextos, manifesto carregado idêntico. As duas poses baixas finais foram inspecionadas no renderer; o vídeo World anterior usa os mesmos desenhos/pivôs de caminhada e salto e antecede somente essa troca de poses baixas.
