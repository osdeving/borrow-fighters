# 32 — Chegada cinematográfica e vizinhança

Goal autorizado em 10/09/2026, após aprovação da rua modular. Branch:
`feature/prologue-scene-improvements`. Referência preservada: **`bdd9ae6`**.

## Resultado pretendido

- Letreiro Casa Nossa legível, com fonte adequada a tamanhos pequenos e
  filtragem correta; ônibus urbano com comprimento e altura proporcionais.
- Quarto com ar da manhã e pássaros suaves. Exterior com trânsito cotidiano
  discreto, sem melodia deslocada, buzinas ou tumulto no ambiente calmo.
- Moradores na mercearia e no ponto, além de um cachorro caramelo. A EP
  interrompe a rotina: moradores entram na mercearia; o lojista espera o
  último, puxa a porta metálica de enrolar e se abriga. O cão foge em segurança.
- Rua permanece evacuada durante o combate. Som de veículos também termina,
  conservando os efeitos pontuais do acidente e do fechamento da porta.
- Ao entrar na rua, câmera começa próxima da pipa contra o céu, desce e
  abre o enquadramento até Rust. Só então libera controles e interface.

## Corte e continuidade

A chegada dura cerca de seis segundos, em `Stage::Encounter`, com relógio
fixo e ambiente vivo. Durante esse intervalo o combate e os comandos de ação
não avançam. Retry no checkpoint acordado não repete a chegada; restart da
história a reproduz. Avançar trecho durante a chegada libera a exploração,
sem pular involuntariamente a luta; pular tudo mantém o contrato existente.

Câmera aplica uma transformação ao mundo e conserva a interface em tela.
Enquadramento final coincide exatamente com o início da câmera jogável.
A rua usa os assets e a composição já aprovados, com moradores/cão/porta
separados e substituíveis pelo catálogo. Não criar editor, ECS ou pipeline.

O estado dos moradores deriva do relógio de `AmbientState`. Posições calmas
são preservadas ao susto; cada pessoa tem tempo suficiente para alcançar
fisicamente a entrada antes de ser ocultada. A porta só começa a baixar
depois da última entrada, fecha com contato visível no chão e permanece
fechada. Moradores de fundo não entram em hitboxes ou regras de combate.

Áudio externo separa ar e trânsito, permitindo apagar o trânsito durante a
fuga sem interromper de forma brusca o restante da paisagem sonora. Sons
novos são originais e reproduzíveis, com procedência e níveis documentados.

## Verificação

- Revisar letreiro em resolução nativa, dimensão do ônibus e sobreposições.
- Capturar câmera do início à entrega de controle; testar input antecipado,
  pausa, retomada, avanço de trecho, retry e restart.
- Conferir moradores em ambas as posições, porta aberta/baixando/fechada,
  caramelo e ausência de retorno após evacuação, inclusive EP tardia.
- Capturar som real do quarto e exterior e conferir transição, fuga e silêncio
  de trânsito posterior. Validar loops, fades, pausa e ausência de cues repetidos.
- Fmt, Clippy, testes, isolamento, empacotamento e links; evidências e commits.

[Decisão de arquitetura](adr/0026-cinematic-arrival-and-neighbours.md).
[Diário](worklogs/cinematic-neighbourhood-arrival.md).
