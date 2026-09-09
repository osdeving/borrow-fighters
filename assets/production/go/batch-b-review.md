# Go novo — lote B

> Registro da revisão estática na entrega do lote. A integração e a recaptura nativa final estão concluídas; veja o [laudo final](finalization-review.md). As referências a pendências abaixo descrevem aquela etapa anterior.

Seis ações novas prontas para integração, com **18 poses** e o [novo master adulto](reference/master.png) como referência fixa em todas as gerações. Nenhuma arte do Go antigo foi utilizada. As folhas foram produzidas pelo `image_gen` integrado; fontes, prompts vN e tentativas rejeitadas permanecem em cada pasta.

| Ação | Fonte selecionada | Escala uniforme para runtime | Tempos das fases em ms | Revisão |
| --- | --- | --- | --- | --- |
| punch_light | v4 | 0,35013262599469497 | 50 / 83 / 100 | [contato](punch_light/review/punch_light-1.png), [sequência](punch_light/review/punch_light.gif) |
| punch_heavy | v2 | 0,3641379310344828 | 183 / 167 / 233 | [contato](punch_heavy/review/punch_heavy-1.png), [sequência](punch_heavy/review/punch_heavy.gif) |
| kick | v1 | 0,3557951482479784 | 116 / 117 / 150 | [contato](kick/review/kick-1.png), [sequência](kick/review/kick.gif) |
| sweep | v2 | 0,3707865168539326 | 166 / 150 / 217 | [contato](sweep/review/sweep-1.png), [sequência](sweep/review/sweep.gif) |
| overhead | v1 | 0,37660485021398005 | 166 / 100 / 200 | [contato](overhead/review/overhead-1.png), [sequência](overhead/review/overhead.gif) |
| throw | v2 | 0,3384615384615385 | 100 / 50 / 216 | [contato](throw/review/throw-1.png), [sequência](throw/review/throw.gif) |

Cada pasta tem `action.json`, `sheet-v1.png` e `preparation.json`. As folhas de preparação medem 2400×920, em três células de 800×920; os pivôs são explícitos. As imagens geradas originais medem aproximadamente 1881×836, apesar da composição solicitada de 2304×1024. A preparação apenas recorta e acrescenta margem transparente. A escala única de cada ação foi medida a partir de uma pose corporal normal, sem redimensionar o contato separadamente. [Resumo para integração](batch-b-review.json).

## Contato e anatomia

- **Jab:** v4 mantém o cotovelo dobrado e põe os nós dos dedos no alcance curto. As variantes anteriores ficavam altas ou recuadas demais.
- **Soco forte:** v2 baixa o contato, com rotação de tronco e apoio dos pés; alcance maior que o jab.
- **Chute baixo:** sola e calcanhar legíveis dentro da caixa baixa; pé de apoio no pivô e retorno à perna recolhida.
- **Sweep:** v2 deixa a pata estendida apontando para a frente e a sola entre −136 e −96 em runtime. A mão e as garras de apoio têm pequena diferença de profundidade: o pivô usa o plano intermediário, com cerca de 3 px de diferença acima/abaixo do chão. v3 foi preservada e rejeitada porque perdeu detalhe de pelagem sem melhorar suficientemente esse apoio.
- **Overhead:** punho elevado na preparação e trajetória descendente até o contato, seguido de guarda. A mão levantada não foi usada para reduzir a escala do corpo.
- **Throw:** duas mãos separadas, cotovelos dobrados e agarre próximo. A mão dianteira toca a caixa curta; a mão de suporte fica naturalmente mais recuada.

Todos os marcos de contato medidos estão dentro das caixas originais. Os valores numéricos e as ressalvas ficam em [batch-b-review.json](batch-b-review.json). As caixas desenhadas nos painéis são diagnósticas e não entram nas folhas de runtime.

## Alpha e verificação

A ferramenta devolveu RGB, embora alpha real tenha sido solicitado. A extração local autorizada utilizou `remove_generated_magenta.py` e `remove_generated_checkerboard.py`, sem `--rust-contour`. No jab v4, resíduos isolados nos cantos vazios foram removidos; corpo, pelagem e bigodes foram preservados. As 18 poses foram inspecionadas em tamanho de jogo; os contatos também foram vistos espelhados, sobre fundo claro e escuro. Há margem transparente em todos os recortes finais.

Asserções de registro confirmaram igualdade de todos os tempos/fases e contratos com `action-planning.json`, presença de alpha e margens, e inclusão dos seis marcos de contato nas caixas. Nenhum candidato global, atlas, regra de combate ou arquivo `src/` foi alterado por este lote. A validação final em Lab/World e os testes de candidatos ficam com a integração do conjunto completo.

Os GIFs são diagnósticos e arredondam durações para múltiplos de 10 ms; os tempos exatos permanecem nos JSONs. Os painéis e clips também estão em `target/art/go-replacement-b`. [Script de preparação](punch_light/prepare_batch_b.py) reproduz os recortes, pivôs, folhas e revisões a partir das fontes com alpha.

Sem commit ou push.
