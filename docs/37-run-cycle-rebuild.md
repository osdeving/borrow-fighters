# 37 — Reconstrução da corrida de Rust

A corrida é um movimento de uso constante. A revisão anterior corrigiu sua
largura, mas manteve uma combinação de proporções, flexão excessiva e recortes
de roupa que produzia pernas amassadas e botas encobertas. Esta rodada revisa
a base anatômica e a representação da roupa, com comparação antes/depois.

## Referências e aplicação

Não há uma única técnica usada por todos os grandes jogos. A documentação da
[Epic sobre Distance Matching](https://dev.epicgames.com/documentation/en-us/unreal-engine/distance-matching-in-unreal-engine)
parte de sequências existentes e sincroniza suas poses com o deslocamento.
Os ajustes de [Pose Warping](https://dev.epicgames.com/documentation/en-us/unreal-engine/pose-warping-in-unreal-engine)
adaptam passada e contato à movimentação.

[Thomas Vasseur descreve o processo de Dead Cells](https://www.gamedeveloper.com/production/art-design-deep-dive-using-a-3d-pipeline-for-2d-animation-in-i-dead-cells-i-):
uma fonte 3D editável produz poses e sprites. Sua lição aplicável aqui é validar
poses e timing e conservar uma fonte que permita corrigir a animação. Adotar
esse pipeline inteiro exigiria um modelo 3D preparado para a identidade do jogo.

Para esta arte 2D, a referência de
[pesos de malha do Spine](https://esotericsoftware.com/spine-weights) é mais
direta: deformar uma peça contínua por ossos, controlando a região da dobra
e sua oclusão. Essa escolha é nossa aplicação das referências, não uma cópia
do pipeline de um desses jogos. [Pesquisa completa](worklogs/run-cycle-research.md).

## Implementação

A calça usa uma textura contínua por perna. A malha articula coxa/joelho/canela
com deformação localizada; as botas continuam rígidas. A arte tem planos de
tecido mais limpos, sem anéis nas articulações. Poses de contato, compressão,
passagem e recuperação são conferidas junto das proporções e da silhueta.

O rig anterior mantinha a pelve baixa em relação aos comprimentos da perna e à
altura da sola, forçando a flexão mesmo no contato. A revisão corrige essa
relação e reposiciona a recuperação do pé distante. O tamanho dos tênis é
uniforme: a âncora é recalibrada para conservar o vetor tornozelo–sola. As
faixas do tronco mantêm cabeça e emblema em escala original, ajustando somente
trechos simples entre essas regiões.

Durante o apoio, a sola continua definindo a orientação e posição do tênis.
Durante o voo, o solver primeiro resolve a trajetória do tornozelo e os dois
ossos; depois a bota gira rigidamente em torno desse tornozelo para acompanhar
a canela. `air_ankle` em `run-rig.json` define a flexão relativa e as faixas de
entrada/saída dessa orientação. O alinhamento desaparece gradualmente antes
do contato. Assim, corrigir o ângulo do tênis não muda joelho/comprimentos nem
cria um ciclo de realimentação entre a orientação e o solver.

Fase, apoio, posição e áudio continuam relacionados à distância efetivamente
percorrida. Física e colisão permanecem no domínio de gameplay. Cabeça, roupa,
rig, câmera e dados de movimento pertencem à aventura. Não há runtime externo
de animação, ECS ou banco de motion matching.

O carregamento confere o alcance dos alvos e a compatibilidade entre malha e
crop. F5 aceita o conjunto inteiro ou conserva o anterior, permitindo corrigir
arte, landmarks e trajetórias sem interromper os relógios da cena.

| Conteúdo | Arquivo |
| --- | --- |
| Alvos, pelve, ritmo e proporções | [run-rig.json](../assets/adventure/locomotion/run-rig.json) |
| Malha, pesos e landmarks | [run-mesh.json](../assets/adventure/locomotion/run-mesh.json) |
| Textura contínua | [leg.png](../assets/adventure/locomotion/run-mesh/leg.png) |
| Fonte e prompts de geração | [prompts.md](../assets/adventure/locomotion/source/run-mesh/prompts.md) |
| Catálogo de poses e peças | [catalog.json](../assets/adventure/locomotion/catalog.json) |
| Distância e clips | [motion.json](../assets/adventure/locomotion/motion.json) |

## Verificação visual

O protocolo compara 16 fases nos dois sentidos, em tamanho real e ampliado,
incluindo as fases 0, 0,125 e 0,375 apontadas pelo usuário. O vídeo exercita
partida, corrida constante, freio, retomada e reversão com o movimento real
do jogo. O baseline é preservado e não pode ser sobrescrito pelo harness.

A [evidência antes/depois](evidence/run-cycle-rebuild/README.md) inclui as folhas,
12 segundos a 60 fps e capturas nos cenários reais do prólogo e do capítulo.
Foram conferidos os quadros consecutivos, transições e a telemetria de
deslocamento. A captura terminou sem erro e sem mudanças nos 129 hashes de
entrada. Após a correção do tornozelo, a suíte completa teve 538 testes
aprovados e dois testes existentes
de dispositivo de áudio ignorados; fmt, Clippy e os 20 testes de pacote passaram.

Reprovar: perna em sanfona, mudança de forma da bota, lacunas no joelho/tornozelo,
encurtamento aparente por recortes sobrepostos, pé deslizando no apoio ou
deformação da cabeça/emblema para compensar medidas ruins. Contato e geometria
testáveis complementam a inspeção; compilar não aprova uma animação.

[Decisão](adr/0031-continuous-run-leg-mesh.md) ·
[Diário e retomada](worklogs/run-cycle-rebuild.md).
