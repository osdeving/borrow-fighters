# 31 — Rua brasileira modular e evacuação

Goal solicitado após a segunda rodada do prólogo, em 10 de setembro de 2026.
Branch: `feature/prologue-scene-improvements`. Ponto de retorno aprovado:
`c2dbbe4` (`feat: add prologue traffic and pole accident`).

**Estado:** implementação integrada na branch, em validação. A captura nativa
e as verificações completas desta rodada ainda estão sendo produzidas.

## Pedido

- Manter pessoas, veículos e objetos como peças isoladas e sobrepostas,
  substituíveis e reaproveitáveis sem redesenhar a cena.
- Diferenciar silhuetas dos carros do cotidiano brasileiro e incluir ônibus.
- Acrescentar ponto de ônibus, placas e o Bar e Mercearia Casa Nossa.
- Fazer todos reagirem à EP: garoto foge; ciclistas freiam, abandonam a
  bicicleta e correm; carros e ônibus aceleram e saem. Preservar o acidente.
- Encerrar a circulação após a evacuação. Permanecem objetos abandonados,
  o veículo batido e os efeitos do acidente, sem novas voltas de figurantes.

## Implementação

O [catálogo externo](../assets/adventure/street/catalog.json) da aventura
associa IDs a PNG/recorte, apoio, tamanho e animação. A
[composição da rua](../assets/adventure/street/scene.json) guarda instâncias,
posições, escala e rótulos. Os mesmos IDs podem apontar para um PNG individual
ou uma região de atlas; trocar a arte não exige redesenhar o bairro nem alterar
o combate. Textos de placas e fachada permanecem editáveis. O contrato não
introduz editor, ECS ou um sistema genérico de cenas.

O catálogo disponibiliza **seis silhuetas**: hatch compacto, sedã, picape,
SUV, ônibus urbano e van. A circulação desta cena usa cinco — hatch, sedã,
picape, SUV e ônibus — enquanto a van fica disponível para reutilização.
Variação vem de desenho e proporção, além da cor. O carro do acidente
conserva suas peças íntegra/amassada e o impacto no poste.
[Assets e procedência dos veículos](../assets/adventure/street/VEHICLES.md).

Fachada do **Bar e Mercearia Casa Nossa**, abrigo de ônibus, plaquinhas e
canto de engradados/cadeira/lixeira são peças sobrepostas com alpha, separadas
do bitmap do bairro. O mesmo canto de objetos aparece em duas instâncias,
com escalas diferentes. Palavras e linhas do ponto vêm do catálogo de textos;
as superfícies dos PNGs ficam vazias para recebê-las. A bicicleta caída também
é uma peça própria, reaproveitável.
[Recortes, apoios e prompts dos adereços](../assets/adventure/street/PROPS.md).

A reação captura as posições no momento da EP e segue trajetórias contínuas,
sem teleporte ou wrap após o susto. Relógio único coordena animação e sons.
Pausa congela, retry rearma e restart restaura a manhã calma. A evacuação
funciona a partir da posição que cada ator ocupava quando a EP apareceu,
inclusive quando o jogador demora para se aproximar.

Os ciclistas freiam, descem com os pés no chão, soltam o guidão e fogem a pé
com quatro poses de corrida. A bicicleta gira durante a queda e permanece
abandonada; o som acompanha seu contato com o chão. Carros e ônibus aceleram
até sair, e a fuga do garoto com a pipa é preservada. Depois da evacuação,
nenhum figurante faz uma nova volta: ficam bicicletas abandonadas, carro
batido, objetos da rua e efeitos do acidente. Os atores decorativos continuam
sem corpos, hitboxes ou influência no resultado do combate.

O [guia das peças](../assets/adventure/street/README.md) reúne os arquivos
consumidos pelo runtime. Recortes e apoios são dados; as poses de ciclista
preservam escala corporal e direção própria ao espelhar a arte.

## Verificação e entrega

- Testar troca de PNG/recorte e reutilização da mesma peça em duas instâncias.
- Testar continuidade no susto, aceleração, abandono, fuga e ausência de novos
  figurantes durante combate prolongado, além de pausa/retry/skip/restart.
- Conferir as duas câmeras, leitura de placas, escala e sobreposições reais.
- Capturar rua calma, caos e rua vazia, com áudio nativo na prévia.
- Rodar Fmt, Clippy, testes, fronteiras, empacotamento e links Markdown.
- Commitar etapas coerentes, preservando `c2dbbe4` como retorno explícito.

Arquitetura na [ADR 0025](adr/0025-replaceable-street-pieces.md).
Andamento no [diário da branch](worklogs/prologue-scene-improvements.md).
