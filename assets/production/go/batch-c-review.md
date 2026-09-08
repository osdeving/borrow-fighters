# Novo Go — lote C

> Registro da revisão estática na entrega do lote. A integração e a recaptura nativa final estão concluídas; veja o [laudo final](finalization-review.md). As referências a pendências abaixo descrevem aquela etapa anterior.

**Aprovado estaticamente para integração:** sete ações, 21 chaves novas, nove fontes imagegen versionadas. Master novo utilizado em todas as chamadas; nenhuma arte do Go anterior foi usada como referência. A captura integrada World/Lab depende da promoção pelo coordenador e ainda não é alegada por este laudo.

O [registro estruturado](batch-c-review.json) contém `sheet`, `scale_to_runtime`, notas, hashes e medidas dos quadros por ação para o integrador. Os sete `action.json` possuem retângulos explícitos, pivôs, tempos e fases. `review-plan.json`, `production.json` e o atlas global da substituição não foram alterados por este lote.

| Ação | Fonte | Escala única | Resultado |
| --- | --- | ---: | --- |
| anti_air | [v1](anti_air/source-v1.png) | 264/578 | Preparação, uppercut alto e recuperação; luva dentro da caixa original acima da cabeça. Apoio preservado. A extensão do tronco/pernas na pose ativa aumenta naturalmente a altura da cabeça. |
| air_punch | [v1](air_punch/source-v1.png) | 264/680 | Punho diagonal para baixo, registro virtual pela cintura perto de −150px e retorno à guarda aérea. Tronco inclina no contato. Recorte central amplo preserva a pata traseira que cruza a divisão nominal da folha. |
| air_kick | [v2](air_kick/source-v2.png) | 264/660 | Joelho e tornozelo corrigidos por geração após v1 alcançar longe/baixo. Pata de contato dentro da caixa em altura; extremo da garra fica aproximadamente1px além da borda frontal. Cintura registrada, sem mover corpo para acertar a caixa. |
| special | [v2](special/source-v2.png) | 264/700 | Primeira chave já emite com a palma baixa na origem baseline. V1 tinha mão alta e foi rejeitada. Recuo/retorno próprios; projétil cyan separado reaproveitado. |
| spawn | [v1](spawn/source-v1.png) | 264/709 | Ajuste de faixa, reverência, guarda. A primeira postura de pernas estendidas é cerca de15px mais alta que a guarda flexionada; o movimento vem do desenho, sem reescala. |
| victory | [v1](victory/source-v1.png) | 264/720 | Relaxamento, punho comemorando e gesto final contido, com expressão adulta e patas apoiadas. |
| defeat | [v1](defeat/source-v1.png) | 264/743 | Cansaço em pé, descida sobre um joelho e derrota apoiada. O corpo se dobra; a última pose não foi ampliada. |

O alpha usa o helper genérico autorizado, sem `--rust-contour`. Fur, bigodes, dedos e garras foram inspecionados em fundo claro/escuro e nas duas orientações. O recorte de special termina em y820 para excluir um único pixel residual no canto da folha em y835, abaixo de toda a arte; os pixels do personagem não foram retocados. As fontes originais e tentativas rejeitadas continuam preservadas.

O especial usa a origem já existente: (+97,92;−123,84) relativa à âncora. Com pivô de fonte (243,801) e escala264/700, a projeção cai em aproximadamente (502,64;472,64), em pixels opacos da face inferior/externa da palma. O projétil não foi desenhado junto do corpo. A inspeção integrada deve verificar a emissão no tick0 e a textura cyan separada.

Previews isolados estão em `target/art/go-replacement-batch-c/preview/review`: sete GIFs, tiras de quadros, `overview.png` e `contacts-both-facings.png`. O painel de contatos projeta as caixas/ponto baseline sobre os PNGs em escala de jogo, em ambas as direções. É uma revisão estática; não substitui uma captura do renderer nativo. Os GIFs de revisão usam a pausa final do helper e não devem ser tratados como prova do tempo real.

Validação: 21 quadros visíveis com alpha real, pivôs válidos, nomes/tempos/fases preservados e nenhum campo `combat` novo. Os limites ativos dos três ataques do lote foram conferidos contra o contrato atual. Os263 arquivos históricos de `assets/production/go` e `assets/candidates/go` continuavam intactos no fechamento estático, conforme o registro de hashes.

A geometria corporal, física do salto, colisões, danos e projétil baseline permanecem responsabilidade do jogo. A revisão integrada final deve observar a continuidade com o idle novo, sobretudo o salto para os aéreos e o retorno de cada golpe.
