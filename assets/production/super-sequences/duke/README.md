# Duke — Garbage Collector

[collector-poses.png](collector-poses.png) é um atlas novo de oito poses,
1536 × 1024, RGBA, produzido com `imagegen` integrado em 9 de setembro de 2026.
Referência de identidade: [Duke existente](../../duke/reference/master-existing.png).
Mantém cone preto, corpo marfim, nariz vermelho, membros pretos, emblema de café
e vapor. A boca temporária nas poses de comer/mastigar é a extensão exigida pelo
roteiro; não foram adicionados olhos ou rosto humano.

| Índice | Pose específica |
| --- | --- |
| 0 | Queda com mãos levantadas. |
| 1 | Agachar e alcançar o lixo no chão. |
| 2 | Levar o lixo à boca aberta. |
| 3 | Mastigar com bochechas cheias. |
| 4 | Segunda chave de mastigação, comprimindo bochechas e corpo. |
| 5 | Salto de satisfação. |
| 6 | Queda pesada do Duke gigante. |
| 7 | Impacto com corpo comprimido e mãos no chão. |

O [renderer](../../../../src/engine/render/authored_actors.rs) recorta cada pose
explicitamente, incluindo o vapor separado da silhueta, e preserva escala entre
as ações. Doze coletores chegam aos pontos do lixo; queda, pegada, mordida,
mastigação e saída seguem o mesmo relógio dos eventos de áudio. O lixo permanece
no chão até a mão chegar. O Duke principal usa poses novas de antecipação e
mastigação durante toda a coleta, substituídas pela queda gigante, impacto e
restauração; não há uma segunda versão sólida do atacante sobreposta.

A primeira geração reproduziu um checkerboard pintado. A edição de
[extração de fundo](../background-extraction-prompt.txt), feita também com
`imagegen`, produziu a transparência real: 66,56% de pixels com alpha zero.
O PNG selecionado foi copiado sem processamento de raster posterior.

Prompt original: [prompt.txt](prompt.txt). Dimensões, componentes de alpha,
arquivo fonte da ferramenta e SHA-256: [generation.json](generation.json).
O contrato jogável está na [rodada23](../../../../docs/23-authored-super-sequences.md).
