# ADR 0031 — Calça contínua e poses controladas na corrida

Status: aceita para a reconstrução autorizada da corrida de Rust.

## Contexto

O rig de recortes rígidos sobrepunha duas peças enrugadas em cada joelho.
A anatomia e os alvos do pé mantinham as pernas excessivamente flexionadas.
Aumentar a largura agravou a oclusão e não corrigiu a pose-base.

A [pesquisa de fontes primárias](../worklogs/run-cycle-research.md) distingue
animação-base, sincronização por distância, skinning e ajustes de contato.
A qualidade da pose e da silhueta vem antes das correções procedurais.

## Decisão

Substituir a sobreposição de coxa/canela por uma textura contínua de roupa em
cada perna, deformada numa malha pequena com dois ossos e correção localizada
do joelho. Botas permanecem rígidas, com tornozelo e sola explícitos. A arte
tem poucas dobras e nenhum anel/cuff no joelho. Preservar cabeça/identidade,
articular quadril e corrigir as proporções do tronco sem esmagar o rosto.

O ciclo tem poses-base controladas para contato, compressão, passagem e voo.
Distância percorrida continua determinando sua fase. Pé de apoio acompanha o
chão, e os limites da articulação impedem colapso ou extensão impossível.
Física, velocidade, combate e as travessias de costas continuam independentes.

No voo, resolver primeiro as juntas e depois orientar a bota em torno do
tornozelo para acompanhar a canela. A orientação retorna gradualmente àquela
do apoio antes do contato; ela não realimenta o IK nem altera os comprimentos.
Flexão relativa e janelas de interpolação pertencem a `air_ankle` no JSON.

Manter dados específicos em JSON e desenho texturizado junto de `engine`.
Não adotar uma engine/ECS ou runtime externo de animação para esse recorte.
Não chamar a implementação de motion matching: não existe busca em banco de
animações capturadas.

## Consequências

Arte, landmarks, pesos, poses e apoios podem ser ajustados independentemente.
Os desenhos antigos ficam preservados como histórico, fora do caminho visual
da corrida corrigida. Testes geométricos precisam vir acompanhados de revisão
de todos os frames nos dois sentidos, a 1x/2x, e vídeo de partida/parada/virada.
Um solver numericamente válido não é evidência suficiente de boa animação.
