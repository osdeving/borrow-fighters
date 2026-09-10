# 31 — Rua brasileira modular e evacuação

Goal solicitado após a segunda rodada do prólogo, em 10 de setembro de 2026.
Branch: `feature/prologue-scene-improvements`. Ponto de retorno aprovado:
`c2dbbe4` (`feat: add prologue traffic and pole accident`).

## Pedido

- Manter pessoas, veículos e objetos como peças isoladas e sobrepostas,
  substituíveis e reaproveitáveis sem redesenhar a cena.
- Diferenciar silhuetas dos carros do cotidiano brasileiro e incluir ônibus.
- Acrescentar ponto de ônibus, placas e o Bar e Mercearia Casa Nossa.
- Fazer todos reagirem à EP: garoto foge; ciclistas freiam, abandonam a
  bicicleta e correm; carros e ônibus aceleram e saem. Preservar o acidente.
- Encerrar a circulação após a evacuação. Permanecem objetos abandonados,
  o veículo batido e os efeitos do acidente, sem novas voltas de figurantes.

## Corte de implementação

Catálogo próprio da aventura associa IDs a PNG/recorte, apoio, tamanho e
animação. Composição da rua fica num arquivo separado, com instâncias e
posições. Os mesmos IDs podem apontar para um PNG individual ou uma região
de atlas; trocar a arte não exige redesenhar o bairro nem alterar o combate.
Textos de placas e fachada permanecem editáveis. Não criar editor ou ECS.

O trânsito possui silhuetas de hatch compacto, sedã, utilitário/picape e
ônibus urbano; variação vem de desenho e proporção, além da cor. Adereços
têm fundo transparente e não são incorporados ao bitmap do bairro.

A reação captura as posições no momento da EP e segue trajetórias contínuas,
sem teleporte ou wrap após o susto. Relógio único coordena animação e sons.
Pausa congela, retry rearma e restart restaura a manhã calma. A evacuação
deve funcionar mesmo se o jogador demorar para se aproximar da EP.

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
