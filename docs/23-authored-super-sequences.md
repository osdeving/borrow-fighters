# 23 — Sequências autorais de especiais e identidade sonora

Esta página registra a rodada23. As reações, Python, C++ e a troca persistente
de arena evoluíram na [rodada24](24-reactions-and-transformations.md).

## Pedido e plano de execução

Rodada de 9 de setembro de 2026. O usuário autorizou animações específicas,
sprites novos, sons pesquisados na internet, sequências mais longas que golpes
normais e dois commits: o estado anterior primeiro, esta entrega ao terminar.
O estado anterior está preservado no commit `481448b`.

1. Diversificar vozes de Rust, Old C, Go e C++, com fontes redistribuíveis,
   procedência e variação entre golpes. Preservar Duke e Python conforme a última
   indicação do pedido; Rust recebe esforço jovem e Old C esforço envelhecido.
2. Criar uma sequência explícita de super sob `World`, com relógio de frames,
   captura dos dois atores, pausa curta na entrada, fases e eventos sonoros.
   A música pausa e retoma; o áudio dos eventos específicos continua tocando.
3. Produzir atlas exclusivos para os clones de Duke e a comédia física de C++,
   lixo separado e elementos de construção de Rust. Preservar fontes e prompts.
4. Implementar os quatro roteiros abaixo, sincronizando poses, som, contatos,
   queda e retomada de controle. Go e Python mantêm seus cinematográficos atuais.
5. Adaptar treino/showcase a durações próprias, com pausa, frame-step, repetição,
   troca de lado e resultado real. Documentar lista de golpes e contra-jogo.
6. Validar regras, áudio, fases gráficas e controles no runtime; produzir evidência
   audiovisual e commitar o conjunto ao final.

## Contrato jogável desta rodada

Os quatro supers usam `Y` / `]` / `LB+RT`. Os especiais de assinatura anteriores
permanecem em `T` / `\\` / `RT`. Esta rodada substitui a apresentação dos quatro
cinematográficos; não adiciona um terceiro botão nem exige medidor.

A sequência é aceita quando o atacante está livre no chão e o round permite
combate. Uma pausa de entrada perceptível (~0,1s, não literalmente um milésimo de
segundo, que não ocuparia sequer um frame a 60 Hz) confirma que o alvo foi pego.
Movimento, projéteis e ataques comuns ficam suspensos; novos comandos não pulam
fases nem causam hits adicionais. O alvo e sua defesa são capturados na entrada.
Bloquear em pé ou abaixado reduz fortemente o dano, mas permite chip.

Durante esta fase de protótipo, a captura de um super aceito é garantida inclusive
à distância. Java posiciona a queda final sobre o alvo; C reinicia a cena; C++
corre efetivamente até o adversário; Rust transforma o espaço do alvo. Somente
eventos explícitos da sequência aplicam dano, nunca a geometria decorativa.
Essa exceção substitui a exigência de contato inicial próximo da rodada22 para
estes quatro golpes. Custos, raridade e maneiras de escapar serão balanceados
depois. Não introduzir KO instantâneo como regra do golpe.

Defesa, invencibilidade de treino, vida, KO, restauração de música, reset e saída
da cena devem continuar coerentes. Um KO no meio da animação termina a sequência
visual antes de anunciar o resultado. Inputs simultâneos não podem gerar duas
sequências concorrentes nem dar prioridade fixa ao Player1.

## Roteiros e identidade

| Personagem | Cinematográfico | Sequência obrigatória |
|---|---|---|
| Duke / Java | Garbage Collector | Pausa; copos, cascas de banana e papel caem por toda a arena, assentam no chão; clones caem em posições de lixo, pegam e comem rapidamente com sons cadenciados; todos somem após coletar; Duke gigante cai sobre o alvo e provoca queda. |
| Rust | Ownership Eclipse | Breve apagão; eclipse escurece o fundo; o anel do Sirius, placas metálicas, feixes e conexões de ownership se materializam, dobram/reorganizam o piso e transformam o mundo; pulso final seguido de dissolução e restauração. |
| Old C | General Protection Fault / #GP | Terminais com erros se multiplicam em cascata; a arena quase desaparece sob tela azul, mantendo os atores; BIOS/POST cobre absolutamente tudo por uma breve fase; reboot devolve a arena com o alvo atingido e caído. |
| C++ | Undefined Behavior: Footgun | Bazuca gigante aponta ao próprio pé; erro de ponteiro, disparo cômico sem gore; levanta o pé e pula; fica furiosa, avança até o adversário esteja onde estiver e alterna socos/chutes numa rajada rápida; final forte e retorno ao controle. |

O tiro no pé é comédia visual e não remove HP da própria C++ nesta rodada.
Cada sequência tem duração suficiente para ler a preparação e os eventos, sem
forçar os timings antigos de um soco. O dano total e o chip ficam em dados puros,
com vários contatos explícitos somente na rajada de C++.

## Arenas de origem

Os personagens continuam jogáveis em qualquer cenário. A arena de origem é uma
âncora visual/narrativa, não seleção obrigatória nem bônus de combate.

| Personagem | Arena de origem | Elementos reconhecíveis |
|---|---|---|
| Rust | Sirius / Campinas | Anel de luz, placas segmentadas, cabos e feixes de pesquisa. |
| Duke / Java | Java Street / São Paulo | Vida urbana, consumo, papelada e coleta organizada. |
| Old C | Porto Digital / Recife | Infraestrutura antiga, terminais e memória de sistemas. |
| C++ | Vale do Pinhão / Curitiba | Arquitetura modular, interfaces e abstrações sobre legado. |
| Python | BioTIC / Brasília | Ciência, geometria, visualização de dados e biotecnologia. |
| Go | Tech Coast / Fortaleza | Redes, canais, tráfego de dados e concorrência. |

## Verificação e entrega

- Testes determinísticos de fases, bloqueio/chip, invencibilidade, KO, reset,
  canto, distância, dois lados, simultaneidade e emissão única de áudio.
- Atlas: transparência real, poses específicas, cortes/pivôs explícitos e leitura
  em escala de jogo; não substituir estas ações por animação de idle.
- Sons: links de origem, licença, transformações e arquivo final; evitar cópia de
  um mesmo grunhido com pitch diferente como única distinção de personagem.
- Capturas e vídeo com som de todas as fases. Verificar pausa e retomada de música.
- `cargo fmt`, `cargo clippy`, `cargo test`, links Markdown e YAML relevantes.
- Atualizar README, backlog, changelog, guia técnico e catálogo de golpes.

Decisão estrutural: [ADR0016](adr/0016-authored-super-sequences.md).

## Entrega implementada

Verificação final: 321 testes Rust aprovados, `cargo fmt`, Clippy estrito e
41 verificações dos controles da aplicação. O teste de dispositivo de áudio,
ignorado na suíte padrão, passou separadamente. [Evidências e reprodução](evidence/authored-supers/README.md).

| Super | Duração | Dano sem guarda | Chip máximo | Momento do dano |
|---|---:|---:|---:|---|
| Ownership Eclipse | 300 frames / 5 s | 28 | 7 | Pulso no frame 212. |
| Garbage Collector | 350 frames / 5,83 s | 32 | 8 | Duke gigante toca o chão no frame 264. |
| General Protection Fault | 320 frames / 5,33 s | 32 | 8 | Reboot devolve a arena no frame 222. |
| Undefined Behavior: Footgun | 356 frames / 5,93 s | 30 | 9 | Oito contatos de 3 HP a cada 10 frames, desde 204; final de 6 HP no 284. |

O chip para em 1 HP. A guarda é capturada na entrada e não pode ser trocada
depois de confirmar o especial. O alvo aéreo fica parado nos oito frames de
entrada e aterrissa durante os frames 8–20, antes dos contatos; sua posição
horizontal permanece. A C++ percorre a distância real até ele nos frames
140–204. Bloqueio preserva a pose defensiva, enquanto acerto pleno termina em queda.

As três cenas de teste — luta, Combat Lab e Move Showcase — usam o mesmo
`World`. O showcase espera o fim dos 300–356 frames antes de trocar de golpe;
pausa, avanço unitário, repetição e inversão de lados continuam disponíveis.
No Lab, o rodapé mostra fase, relógio, contatos e HP em fonte incorporada suave.
O apagão e a BIOS escondem cenário, lutadores, HUD, rodapé e marcador de gravação.

Arte nova: oito poses de Duke, seis poses de comédia física e seis de ataque
para C++, três peças de lixo e quatro estruturas do Sirius. Os atlas são PNGs
com alpha real e cortes/pivôs explícitos; o descarte dos clones acompanha os
sons de coleta. [Produção Duke](../assets/production/super-sequences/duke/README.md),
[C++](../assets/production/super-sequences/cpp/README.md),
[lixo](../assets/production/super-sequences/trash/README.md),
[Rust](../assets/production/super-sequences/rust/README.md).

Áudio: 25 vozes produzidas de gravações e 18 efeitos, com variações por golpe
e por contato. Rust, Old C, Go e C++ têm fontes próprias; os 12 arquivos de voz
de Duke/Python e seus bindings anteriores permanecem iguais, conforme a
confirmação do usuário. Foram usados sons CC0 com autores, URLs, hashes,
recortes e filtros documentados nos [créditos](../assets/audio/ATTRIBUTION.md)
e na [produção](../assets/audio/production-2026-09-09.json).
O [reel com índice](../assets/audio/audition.html) reúne 36 trechos nos volumes
do manifesto. O ambiente verificou reprodução e cursores de música; avaliação
subjetiva dos timbres deve ser feita ouvindo o reel.

Durante a entrada, a música pausa no próprio stream e as vozes/efeitos anteriores
são cortados. Cues novos continuam tocando; a música retoma da posição guardada.
Reset, troca de golpe/personagem e saída da cena cancelam os efeitos pendentes.

Catálogo completo de golpes, inputs, defesa e limitações:
[matriz de combate](15-character-combat-matrix.md#supers-cinematográficos-e-captura-autoral).

A revisão audiovisual também encontrou um gargalo na gravação: a função anterior
chamava a biblioteca para cada pixel de cada frame. A leitura agora obtém o
buffer RGBA contíguo e inverte as linhas em blocos; F9 e o exemplo de captura
compartilham essa operação. Os testes cobrem ordem das linhas, canais e alpha.
A demonstração usa vídeo determinístico e mixagem dos cues registrados pelo
`World`, com os arquivos e volumes do manifesto. A reprodução real e a pausa
do stream foram verificadas separadamente; capturas PulseAudio com atraso
variável no ambiente remoto foram descartadas da entrega.
