# Novo Go — salto corrigido

> Registro da revisão estática na entrega do lote. A integração e a recaptura nativa final estão concluídas; veja o [laudo final](../finalization-review.md). As referências a pendências abaixo descrevem aquela etapa anterior.

**V2 selecionada após comparação com idle e os golpes aéreos.** A nova geração corrige a cabeça e o tronco pequenos do salto anterior. O coordenador comparou v2 e v3 e escolheu v2 pelo volume corporal mais natural; v3 continua preservada como tentativa rejeitada.

[Fonte v2](source-v2.png), [prompt real](prompt-v2.txt), [alpha preparado](keyed-v2.png), [ação selecionada](action.json) e [medidas/hashes](refinement-review-v2.json). O master fixo e o idle definiram identidade e proporções; v1 serviu apenas de referência de pose. A ferramenta utilizada foi o image_gen integrado, com duas gerações reais, v2 e v3.

| Quadro | Recorte na folha 1881×836 | Pivô local | Duração | Topo / sola sólidos na escala de jogo |
| --- | --- | --- | ---: | --- |
| jump_00 | x0, y0, 627×836 | 336, 745 | 60 ms | −258 / −37 px |
| jump_01 | x627, y0, 627×836 | 294, 723 | 340 ms | −252 / −46 px |
| jump_02 | x1254, y0, 627×836 | 236, 784 | 400 ms | −264 / −1 px |

A escala única é **0,37** para os três quadros, total **800 ms**. As medidas usam alpha ≥128 após redimensionamento; não incluem franjas translúcidas. A faixa superior de 30 px da cabeça mede 55/54/53 px, contra 55 px no idle. Flexão do tronco e altura da cintura variam naturalmente na pose de ápice. Não se usou a altura total da figura com pernas dobradas para determinar a escala.

Preparação e ápice mantêm o registro do painel selecionado. Apenas o pivô virtual da descida foi ajustado de aproximadamente Y761 para Y784, cerca de 8 px renderizados para cima, trazendo a sola sólida para 1 px acima da âncora. Esse registro não altera a posição física do personagem ou a trajetória do salto.

Veja a [comparação final com idle e aéreos](review/transition-v2.png), as [duas direções](review/both-facings-v2.png) e a [sequência GIF](review/jump.gif). Os três quadros foram inspecionados nas duas orientações e em fundos claro/escuro, com mãos, pés e margens completos. O helper adiciona uma pausa de inspeção ao último quadro do GIF; os tempos reais são os de action.json.

Foi solicitado alpha real, mas a ferramenta devolveu RGB com checkerboard. A preparação autorizada usou `tools/art/remove_generated_checkerboard.py`, sem modo Rust, deformação do corpo ou escala individual por pose. Fonte original, v1 e calibração anterior permanecem preservadas em [snapshot-before-v2](snapshot-before-v2/registration.json); o fundo tratado não é apresentado como geração adicional.

Nenhum candidato, atlas global, review-plan ou código foi exportado/alterado por esta correção. A promoção e recaptura Lab/World ficam a cargo do coordenador. O defeito anterior está resolvido na comparação estática selecionada; este laudo ainda não afirma validação nativa da nova versão.
