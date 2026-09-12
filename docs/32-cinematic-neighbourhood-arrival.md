# 32 — Chegada cinematográfica e vizinhança

Goal autorizado em 10/09/2026, após aprovação da rua modular. Branch:
`feature/prologue-scene-improvements`. Referência preservada: **`bdd9ae6`**.
Implementado em **`f90a0d1`** e **`6c513e1`**, com revisão visual nativa,
24 verificações funcionais, matriz de testes e gravação do som do jogo.
[Prévia e evidências](evidence/cinematic-neighbourhood/README.md).

## Resultado implementado

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

A chegada atual dura dezoito segundos, em `Stage::Encounter`, com relógio
fixo e ambiente vivo. Durante esse intervalo o combate e os comandos de ação
não avançam. Retry no checkpoint acordado não repete a chegada; restart da
história a reproduz. Avançar trecho durante a chegada libera a exploração,
sem pular involuntariamente a luta; pular tudo mantém o contrato existente.

Câmera aplica uma transformação ao mundo e conserva a interface em tela.
Enquadramento final coincide exatamente com o início da câmera jogável.
A rua usa os assets e a composição já aprovados, com moradores/cão/porta
separados e substituíveis pelo catálogo. Não criar editor, ECS ou pipeline.

Na revisão de 12/09/2026, a rua ampliada ganhou tempo para mostrar sua rotina:
3,5 s na pipa, descida até o menino e a calçada, observação lenta do trânsito
e dos moradores junto à mercearia até 13 s. Um fade de um segundo cobre a
troca para Rust; ele aparece entre 14 e 15 s, e o enquadramento assenta
gradualmente até entregar controle aos 18 s. A câmera não atravessa os
três mil pixels entre o bairro e Rust durante uma panorâmica acelerada.
O ambiente continua no mesmo relógio durante o corte, sem reiniciar carros,
ciclistas, moradores ou a pipa.

O trilho está em [`arrival-camera.json`](../assets/adventure/street/arrival-camera.json).
Cada keyframe informa tick, alvo, zoom, barras, opacidade do fade e âncora
(`hub` ou `gameplay`). A interpolação tem velocidade zero nas junções;
a troca de âncora exige preto completo nos dois lados. O último ponto
define tanto a duração quanto a liberação de comandos e precisa coincidir
com a câmera jogável. Alterações válidas são lidas ao iniciar/reiniciar a
história, sem regenerar sprites ou recompilar o jogo.

O estado dos moradores deriva do relógio de `AmbientState`. Posições calmas
são preservadas ao susto; cada pessoa tem tempo suficiente para alcançar
fisicamente a entrada antes de ser ocultada. A porta só começa a baixar
depois da última entrada, fecha com contato visível no chão e permanece
fechada. Moradores de fundo não entram em hitboxes ou regras de combate.

Áudio externo separa ar e trânsito, permitindo apagar o trânsito durante a
fuga sem interromper de forma brusca o restante da paisagem sonora. Sons
novos são originais e reproduzíveis, com procedência e níveis documentados.

## Verificação executada

- Revisados letreiro em resolução nativa, dimensão do ônibus e sobreposições.
- Capturada câmera do início à entrega de controle; testados input antecipado,
  pausa, retomada, avanço de trecho, retry e restart.
- Conferidos moradores em ambas as posições, porta aberta/baixando/fechada,
  caramelo e ausência de retorno após evacuação, inclusive EP tardia.
- Capturado som real do quarto e exterior; conferidos transição, fuga e
  ausência de trânsito posterior. Validados loops, fades, pausa e cues únicos.
- Executados fmt, Clippy, testes, isolamento, empacotamento e links;
  registradas evidências e commits.

Resultado: 461 testes conjuntos aprovados, 67 de aventura isolada, 395 de
luta e três de core; dois testes preexistentes de dispositivo de áudio
ignorados. Fmt, Clippy estrito, 56 fixtures de fronteiras, 47 do mixer e
14 de pacote aprovados. Captura funcional com 24/24 checks; prévia contínua
de 30,73 s, oito cues confirmados no áudio nativo, sem clipping e com o
trânsito ausente na janela posterior à evacuação. Detalhes e limitações nas
[evidências](evidence/cinematic-neighbourhood/README.md).

[Decisão de arquitetura](adr/0026-cinematic-arrival-and-neighbours.md).
[Diário](worklogs/cinematic-neighbourhood-arrival.md).

## EP — tomada aérea e impacto no chão

A revisão autorizada em 12/09/2026 conserva a introdução da pipa e acrescenta
outra tomada ao encontro. Quando Rust alcança a ameaça, a câmera busca o céu;
a EP entra pelo alto, desce e continua no mesmo trajeto enquanto a câmera
volta ao plano jogável. Após revisão do peso da queda, a EP vem pequena e distante,
cresce na aproximação e acelera continuamente de 320 a 777 px/s. Aos 40 ticks
o enquadramento já é o normal; os últimos 212 px caem em 18 ticks. O contato
acontece no tick 58 (0,97 s), com joelho e mão no chão,
compressão breve, poeira, fragmentos e um tremor curto. **Esse contato causa
o tumulto:** fuga, buzina, ciclistas e moradores partem do mesmo marco.

O corpo usa poses separadas do atlas da errática: antecipação aérea, apoio
no chão e recuperação. O cenário permanece reutilizável. A posição usa
interpolação cúbica com tangentes balísticas, sem desacelerar para sustentar
poses no ar. Rastros e linhas de velocidade seguem a derivada real da trajetória,
sem reiniciar a queda na volta da câmera.
O controle fica suspenso até terminar a recuperação; a EP não causa dano
durante a encenação. Pausa congela corpo, câmera, ambiente e efeitos. Enter/RB
conclui apenas a aterrissagem; um novo avanço pode seguir à apresentação.
Retry acordado omite ambas as tomadas, enquanto reiniciar a história as restaura.

Tempos, tangentes, zoom, enquadramento, índices de pose e parâmetros de poeira
ficam em [`ep-arrival.json`](../assets/adventure/street/ep-arrival.json), validados
antes de uso. F5 recarrega uma revisão válida sem recompilar; se a tomada estiver
em andamento, a revisão aguarda a próxima entrada na rua para preservar o
contato já programado. O arquivo pode ser alterado sem regenerar qualquer imagem.
Os dois sons originais têm [gerador independente](../assets/adventure/audio/generate_ep_arrival_audio.py).

O roteiro nativo [`capture_ep_arrival_x11.py`](../tools/review/capture_ep_arrival_x11.py)
registra o céu, a abertura da câmera, queda no plano jogável, contato e recuperação.
Também confere comandos antecipados, pausa, trajetória, retry e avanço local.
