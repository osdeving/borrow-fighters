# Go novo — referência fixa de produção

O usuário descartou explicitamente o Go anterior por estar caricatural demais e
pediu uma versão mais realista. [Master](master.png) é a nova referência fixa,
gerada de verdade pela ferramenta integrada: [v1](source-v1.png) estabelece a
identidade e [v2](source-v2.png) ajusta pernas/apoio e fornece matte magenta.
Os prompts preservados registram a geração; os sprites humanos serviram somente
como referência de acabamento. Nenhuma pose do Go antigo foi usada como arte nova.

Identidade: gopher antropomórfico adulto e atlético, olhos animais pequenos com
pálpebras, focinho modelado com nariz escuro e incisivos discretos, orelhas
arredondadas pequenas, bigodes finos, pelagem curta azul-ardósia com peito
acinzentado claro e material pintado com volume. Calça de treino carvão até os
tornozelos, faixa preta com duas pontas curtas, wraps/luvas sem dedos escuros e
patas nuas com garras curtas. Não acrescentar cauda longa, armadura, armas, roupas
novas, olhos grandes ou expressão infantil. Preservar os detalhes do master em
toda geração; só acrescentar poses novas como referência de continuidade.

A [comparação em escala de jogo](lineup-preview.png) usa os PNGs reais dos outros
personagens e o master redimensionado, sem redesenhar o elenco. O master tem
altura opaca de 1.331 px e alvo visual de 264 px. Essa imagem é referência de
design, não o clip idle final; o apoio de cada pata será conferido novamente nos
desenhos da ação. [Métricas da referência](master-metrics.json).

Para folhas novas, um ponto de partida prático é corpo ereto de cerca de 660 px,
com eixo do corpo perto do centro de sua célula e piso implícito perto de Y900 em
um canvas com 1.024 px de altura. Três células de 768 px ou quatro de 768 px dão
espaço aos golpes amplos. São instruções de composição; medir os pixels antes de
escolher a escala por ação. Não ajustar cada pose pela própria caixa de alpha.

O corpo em pé de 224 px, corpo agachado de 128 px, nove ataques próximos e especial
permanecem no contrato existente. Preparação, atividade, recuperação, contato e
emissão precisam seguir [action-contracts.json](../action-contracts.json).
No anti-air deve caber a mão acima da cabeça; nos aéreos, separar a pose da
trajetória física. O especial já emite no primeiro quadro, perto da mão baixa
que alcança (+97,92; −123,84) relativamente à âncora. A animação não carrega o
projétil desenhado junto do corpo.

O alpha do master foi preparado pelo helper genérico de magenta, autorizado pelo
usuário. Não usar `--rust-contour`. Preservar os fios de pelagem e bigodes durante
a inspeção clara/escura. PNGs de revisão e fontes com matte não vão ao runtime.
