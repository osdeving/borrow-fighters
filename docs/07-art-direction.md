# 07 — Direção de Arte e Moods

## Status

Documento vivo. A produção atual dos sprites e seus laudos estão na [matriz de cobertura](19-sprite-production-coverage.md); as explorações e origens dos placeholders abaixo permanecem como histórico.

Este documento orienta contribuições visuais sem fechar cedo demais a identidade do jogo.

## Objetivo visual

**Borrow Fighters** deve parecer um jogo de luta 2D estilizado, legível e engraçado, onde cultura de programação vira forma, cor, gesto e impacto. O elenco combina mascotes e humanos com volume, materiais e iluminação coerentes no tamanho de jogo.

Na [aventura autorizada](27-rust-story-adventure.md), essa linguagem de animação
passa a sustentar cenas narrativas e o movimento de Rust. EPs são pessoas de
natureza mística: rosto, postura e pausas devem comunicar emoções e intenções.
Humanos e EPs comportam variedade de caráter e aparência. A origem de uma EP
não justifica atuação fria ou ausência de afeto.

## Direção do prólogo e primeiro encontro

- **Ada:** começar com trabalho cotidiano e humanidade visível. Seu aprendizado
  do Linker e a mensagem misteriosa antecedem Assembly; a transição a híbrida
  humana/EP pode ser sugerida sem explicar seu mecanismo. Evitar apresentar
  Ada como entidade pronta desde o primeiro quadro.
- **Assembly:** primeiro despertar com peso, precisão e presença mística;
  preservar o mistério da mensagem e de sua relação com Ada. Sua identidade
  visual já descrita abaixo orienta a cena.
- **Rust:** acordar como numa manhã comum, com respiração, despertar e mudança
  de atenção antes da ameaça. Mostrar intenção em antecipação, golpe e reação.
  Depois da vitória, um pequeno balanço de cabeça com pesar expressa necessidade,
  sem a pose festiva de vitória de uma partida.
- **Errática:** ameaça com silhueta, aproximação e ataque legíveis. Sua
  instabilidade é uma condição de existência; não equivale a provar que EPs
  sejam incapazes de emoção ou que humanos sejam todos cruéis.
- **Terminal:** blocos curtos digitados, contraste legível e ritmo que permita
  revelar ou avançar texto. A referência de terminal inspira a apresentação;
  frases, gráficos e encenação pertencem ao Linker.

As referências de tom Naruto e Fullmetal Alchemist orientam apenas o peso
emocional, o mistério e a humanidade de seres criados. Não importam personagens,
mitologia, acontecimentos ou recursos visuais dessas obras para o cânone.
O teste inicial pode usar desenho em camadas e poses provisórias; a evidência
em movimento deve mostrar atuação, sem confundir aproximação de câmera com
animação corporal pronta. A produção fica limitada ao episódio da
[entrega 27](27-rust-story-adventure.md), com a apresentação autorizada na
[entrega 28](28-adventure-texts-and-opening.md).

Na manhã, ancorar ombro/antebraço e quadril no colchão, assento na borda e pés
no chão, em vez de alinhar todas as poses pelo fim do recorte. A respiração
preserva o ponto de apoio. Conferir a sequência completa ao mudar qualquer pose.

A [primeira rodada de melhorias do prólogo](30-prologue-scene-improvements.md)
experimenta um quarto com identidade tecnológica de Rust: setup, pôsteres,
terminal e luzes discretas, mantendo a manhã e o caráter de casa brasileira.
Cursor e ventiladores dão movimento ao ambiente sem disputar atenção com o
despertar. As palavras dos pôsteres e da tela vêm do catálogo editável.

Na rua, a calçada do garoto, a ciclovia e a faixa jogável formam planos
distintos. A separação por profundidade e pelos elementos do cenário deve
deixar claro que Rust não entra na ciclovia. Pedais e rodas em movimento,
gestos com a linha e a cauda da pipa sustentam a vida cotidiana. Ao aparecer a
ameaça, o garoto se assusta, solta a linha e foge definitivamente daquele
encontro; a pipa segue pelo vento. Conferir silhuetas e sobreposições nas duas
extremidades da câmera, preservando a leitura da errática e dos golpes.
A [ADR 0024](adr/0024-prologue-background-life.md) mantém esses atores fora da
colisão; os assets e sua [procedência](../assets/adventure/ART-PROVENANCE.md)
pertencem à aventura. A rodada permanece experimental até a avaliação visual.

A segunda rodada acrescenta asfalto entre calçada e ciclovia. Automóveis
ilustrados passam no plano distante; um carro azul entra na faixa próxima ao
surgir a EP, buzina, freia e atinge um poste. Capô dobrado, farol quebrado,
oscilação breve do poste e poeira localizada comunicam o impacto; carro
amassado, fragmentos no chão e fumaça discreta mantêm sua consequência.
Os efeitos se concentram na área do acidente, preservando a silhueta dos
personagens e a continuidade da fuga do garoto. A arte íntegra/amassada usa
a mesma escala e rodas ancoradas nos pontos medidos do atlas.

A [rua modular e sua evacuação](31-brazilian-street-evacuation.md) preservam
essa separação em peças substituíveis. O catálogo oferece seis silhuetas de
veículos do cotidiano: hatch, sedã, picape, SUV, ônibus e van. A cena usa as
cinco primeiras no trânsito; a van permanece disponível para composição.
As diferenças precisam aparecer em carroceria, proporção e rodas, além da cor.

Bar e Mercearia Casa Nossa, abrigo de ônibus e objetos da calçada têm alpha
e ficam sobrepostos ao bairro, com recortes e apoios próprios. As placas dos
PNGs ficam sem palavras: o runtime desenha os textos editáveis sobre elas.
Catálogo de peças e instâncias da cena são arquivos separados, permitindo
trocar a arte ou reutilizar um objeto sem repintar o fundo.

Quando a EP aparece, a vida cotidiana se transforma em evacuação: carros e
ônibus aceleram para fora; ciclistas freiam, descem, soltam a bicicleta e
correm. A fuga mostra mudanças de apoio, mãos e pernas, além do deslocamento
horizontal. A bicicleta cai e permanece no chão; a rua não recebe novas
voltas de figurantes durante o mesmo encontro. Garoto, pipa e acidente
mantêm suas consequências. Conferir a continuidade das poses e a escala
uniforme do ciclista vestido de vermelho-tijolo, inclusive ao mostrar sua
bicicleta vazia. [Adereços e apoios](../assets/adventure/street/PROPS.md) e
[veículos](../assets/adventure/street/VEHICLES.md) registram origem e seleção.

A [quarta rodada, de chegada e vizinhança](32-cinematic-neighbourhood-arrival.md),
abre a rua pela pipa contra o céu. A câmera desce e amplia o enquadramento ao
longo de cerca de seis segundos até apresentar Rust; exploração e HUD ficam
disponíveis ao fim desse movimento. A vida cotidiana continua durante a
chegada. O ônibus ocupa uma largura maior que a dos automóveis. O letreiro
completo `BAR E MERCEARIA CASA NOSSA` usa uma única linha, com tamanho uniforme
e fonte encorpada e filtrada para continuar legível.

Clientes esperam no ponto e conversam junto à mercearia; o lojista permanece
na entrada, enquanto um cachorro caramelo descansa, fareja e observa a rua.
Quando a EP aparece, os clientes correm para dentro da loja. O lojista espera
o último entrar e puxa a porta corrugada até o chão; o cão foge para fora em
segurança. O contato com a entrada, a oclusão das pessoas e o tirador da porta
devem comunicar abrigo e fechamento contínuos. Conservar a escala corporal
entre poses, distinguindo mulher com sacola, jovem de mochila e lojista de
avental. [Moradores e porta](../assets/adventure/street/NEIGHBOURS.md) e
[caramelo](../assets/adventure/street/CARAMELO.md) documentam a arte separada.

O som acompanha essa manhã: ar discreto e poucos pássaros no quarto, ar e
passagens espaçadas de motores na rua. A evacuação reduz o trânsito até
silenciá-lo, enquanto o ar externo permanece. Latido breve, rolo metálico e
contato da porta acompanham ações específicas, preservando o peso do acidente.
O [guia de áudio](../assets/adventure/audio/README.md) registra síntese e níveis;
a revisão humana em movimento deve avaliar o equilíbrio desses sons.

A apresentação após o pesar usa jornais, quadros biográficos e montagem de
personagens. C++ aparece adulta no cotidiano noturno e no despertar heroico;
Python ensina humanos sobre EPs numa universidade. As manchetes representam
imprensa da ficção. Logo, subtítulo e demais palavras são desenhados pelo jogo
a partir do JSON editável, sem texto incorporado aos PNGs novos.

## Pilares visuais

### 1. Legibilidade antes de detalhe

O jogador deve conseguir ler personagem, pose, ataque, dano e direção rapidamente.

### 2. Silhueta forte

Cada personagem precisa ser reconhecível mesmo em placeholder, baixa resolução ou pose rápida.

### 3. Humor técnico visual

Piadas devem nascer de conceitos reais: borrow checker, null pointer, garbage collector, goroutines, exceptions, segfaults, boilerplate, compilador e afins.

### 4. Impacto claro

Golpes precisam ter feedback visual simples: antecipação, contato, hitstop, flash, deslocamento ou mudança de pose.

### 5. Arte escalável

O estilo precisa permitir placeholders agora e refinamento depois, sem exigir produção final pesada cedo demais.

### 6. Brasil tecnológico como cenário

As arenas principais devem se inspirar em locais brasileiros de ciência, tecnologia, arquitetura, inovação e cultura urbana. A direção visual deve mostrar Brasil como lugar de futuro, pesquisa, beleza e mistério, sem depender de estereótipos ou exotização.

## Moods iniciais para explorar

| Mood | Ideia | Risco |
|---|---|---|
| Terminal Arcade | UI e arena com energia de terminal, neon moderado e debug overlays | Virar visual escuro demais |
| Compiler Lab | Ambiente de laboratório, warnings, erros e processos de build como cenário | Ficar técnico demais e pouco divertido |
| Mascot Fight Club | Mascotes cartunescos em arena exagerada | Aproximar demais de mascotes existentes |
| Office Surreal | Mesa, docs, tickets e reuniões virando arena absurda | Parecer piada corporativa genérica |

Nenhum mood está aprovado como direção final. Propostas devem comparar pelo menos uma vantagem e um risco.

## Estado visual atual do protótipo

O menu principal da [rodada 3 da aventura](29-story-terminal-menu.md) adota
INK, papel creme, dourado e verde-água do título final, com a mesma Barlow
Condensed. Moldura de terminal, cursor bloco piscante e revelação binária
aproximam as duas entradas. Esta direção se aplica somente ao menu principal;
seleção, lutas, submenus e livro de lore conservam sua apresentação.
Go fica fora da montagem de abertura por pedido do usuário, sem apagar seus assets.

Os seis lutadores têm atlas de ações revisadas em `assets/candidates/`, escolhidos por padrão; `BORROW_FIGHTERS_SPRITE_CANDIDATES=0` permite comparar o baseline preservado. A produção por ação e a validação específica estão na [matriz atual](19-sprite-production-coverage.md). Arenas, retratos e demais elementos fora dessa rodada mantêm seu estado anterior.

Em 7 de setembro de 2026, o usuário rejeitou o Go caricatural e pediu outra versão com mais realismo. O [novo master](../assets/production/go/reference/master.png) fixa um gopher adulto atlético, pelagem azul-ardósia com volume, olhos animais pequenos, focinho natural, calça carvão, faixa e wraps escuros. As 20 ações usam essa identidade; o conjunto antigo está [arquivado](../assets/production/go-cartoon-archive/ARCHIVE.md). Essa substituição visual conserva corpo físico, velocidade, golpes e projétil separados.

## Origem dos placeholders — histórico preservado

O slice anterior exercitou estas decisões com os assets baseline, ainda disponíveis para referência e fallback:

- arena inicial: `assets/placeholder/arena-sirius.png`, derivada de referência em `assets/references/sirius.png`;
- arenas em rotação: `assets/placeholder/arena-fortaleza.png`, `assets/placeholder/arena-java-street.png`, `assets/placeholder/arena-biotic.png`, `assets/placeholder/arena-porto-digital.png` e `assets/placeholder/arena-vale-pinhao.png`;
- arena anterior ainda disponível: `assets/placeholder/arena-terminal-compiler-lab.png`;
- nomes runtime de arena: `Sirius Light Ring` em Campinas, `Tech Coast Beacon` em Fortaleza, `Java Street Terminal` em Sao Paulo, `BioTIC Garden` em Brasilia, `Porto Digital Cache` em Recife e `Pinhao Smart Grid` em Curitiba;
- VFX leves de cenário: feixes, pacotes de dados, chuva, shimmer, pulsos e scanlines sutis, sempre em baixa opacidade para não competir com HUD, sprites e hit effects;
- personagens atuais: Rust, Duke, Go, C, Python e C++ em atlas placeholder com manifesto JSON;
- retratos de roster: `assets/placeholder/roster-rust.png`, `assets/placeholder/roster-duke.png`, `assets/placeholder/roster-c.png`, `assets/placeholder/roster-python.png` e `assets/placeholder/roster-cpp.png`, derivados dos sprites jogáveis para manter consistência de demo;
- entrada cinematográfica: manifests separados para Rust, Duke, Go, C e Python;
- projectile Rust: engrenagem separada do sprite do personagem;
- projectile Duke: bean separado do sprite do personagem;
- projectile Go: canal/burst separado do sprite do personagem;
- projectile C: bitstream separado do sprite do personagem;
- projectile Python: fluxo de dados separado do sprite da personagem;
- projectile C++: burst separado de operadores `++` e brackets;
- HUD, ajuda e debug visual são opcionais por feature flag.

O C veio de dois atlas de referencia em `assets/references/langc-03.png` e `assets/references/langc-04.png`, com fundo chroma key removido por script local. Ele deve ser avaliado como placeholder jogavel de escala, fluidez e leitura, nao como direcao final do personagem.

Python entrou como candidata visual gerada por IA em `assets/references/python-fighter-atlas-source.png` e repacotada por `tools/art/build_python_fighter_atlas.py`. O atlas jogavel atual usa a pose sheet raster `assets/references/python-fighter-raster-source.png`, empacotada por `tools/art/build_python_high_res_fighter_atlas.py`, com camisa branca, saia preta e poses proprias para os nove golpes proximos; `assets/placeholder/python-fighter-atlas-backup.png` e `assets/placeholder/python-fighter-backup.sprite.json` preservam a versao anterior. A entrada cinematografica dela vive em `assets/references/python-start-atlas-source.png` e `tools/art/build_python_start_atlas.py`, mostrando a personagem montando um cavalete e apontando um grafico de barras colorido como gag de ciencia de dados. A personagem deve ser tratada como original adulta inspirada por Python, ciencia de dados e visao computacional, nao como retrato de pessoa real. Ela ja pode ser escolhida no roster como `python.py`, mas ainda precisa de revisao de leitura em movimento, escala, boxes e identidade mecanica.

C++ entrou como candidata visual raster em `assets/references/cpp-fighter-raster-source.png`, empacotada por `tools/art/build_cpp_fighter_atlas.py` em dois atlas (`cpp-fighter-atlas-a.png` e `cpp-fighter-atlas-b.png`) usando `frames[].image` no manifest. A personagem representa a filha de C, com bolsa/operadores como alusão a herança, `++` e abstrações modernas. Ela ja pode ser escolhida no roster como `cpp.cpp`, mas ainda precisa de revisão de escala fina, boxes e leitura de movimento antes de qualquer status acima de placeholder jogável.

Nada disso é final. O valor desses assets agora é validar proporção, leitura de pose, pivots, altura do projectile, contraste com cenário e necessidades de animação.

A produção por ação dos seis personagens está registrada na
[matriz de cobertura](19-sprite-production-coverage.md), com fontes raster e
referências em [assets/production](../assets/production/README.md). O piloto Rust
valida geração, alpha, pivôs, timing e uso real dos clips antes de aplicar o
processo aos demais. Os atlas em `assets/candidates/` são optativos durante a
revisão; os placeholders continuam preservados como referência e fallback.

## Linguagem visual do Linker

O Linker deve parecer uma força de ligação, não magia genérica. Ele pode aparecer como alinhamento impossível entre cabos, símbolos, partículas, luz, logs, circuitos, reflexos e gestos dos personagens.

Direções visuais:

- Ada Lovelace aparece diretamente no prólogo; inscrições, diagramas, retratos
  parciais e sinais esquecidos podem ampliar sua presença depois;
- Assembly deve parecer fora de fase, com partes do corpo alternando entre matéria, ausência, `0` e `1`;
- a instabilidade de Assembly deve ser sóbria e inquietante, não glitch cômico;
- entidades erráticas surgidas involuntariamente pelas condições criadas pelo
  mau uso do Linker podem ter silhuetas menores, deformadas e incompletas;
- o Linker pode ser legível por cor, símbolo ou movimento, mas nunca deve competir com hit effects e leitura de combate.

## Arenas brasileiras

Toda proposta de arena deve responder:

- qual lugar brasileiro inspira a arena;
- qual conceito técnico ou científico ela comunica;
- como o chão e a profundidade ajudam a luta;
- quais detalhes podem virar easter eggs sem disputar atenção;
- qual mistério visual ela acrescenta ao mundo.

Direções iniciais de arena vivem em [`docs/12-worldbuilding.md`](12-worldbuilding.md).

## Easter eggs

A rodada [Apresentação e Brasil cotidiano](22-presentation-and-brazilian-stage-life.md)
implementa caramelo de pelo curto com corrida em quatro poses e cameo gestual de
“Já acabou, Jéssica?” com figurino da referência em São Paulo. Os atlas, cortes e
origem estão em [stage-life](../assets/production/stage-life/README.md). Detalhes
ficam atrás dos lutadores e são controlados por `ShowStageLife`. O Sirius revisado
retira o animal estático do fundo anterior. Os seis novos especiais compõem
geometrias técnicas de tela inteira mantendo os corpos visíveis e a colisão local.

Easter eggs são parte da identidade visual do jogo. Eles devem aparecer como detalhes de cenário, cartazes, props, telas, nomes de lojas, logs, pichações fictícias ou animações distantes.

Exemplos possíveis:

- cachorro caramelo ao fundo;
- cartaz de bebê reborn;
- placas com trocadilhos de programação;
- QR code falso que vira `404`;
- bug report colado em poste;
- startup fictícia com nome absurdo;
- referência regional pequena validada por pessoas da região.

Regras:

- não atrapalhar leitura de golpes;
- não competir com HUD, hit effects ou silhueta dos lutadores;
- evitar estereótipos como atalho visual;
- manter fácil de remover se o meme envelhecer mal.

## Maturidade de arte

| Estado | Significado |
|---|---|
| Placeholder | Serve para testar gameplay |
| Exploração | Ideia visual ainda livre |
| Candidato | Pode entrar em protótipo ou slice |
| Aprovado | Alinhado com direção atual |
| Final | Pronto para distribuição externa |

## Regras para contribuir com arte

- Referências devem indicar fonte e intenção.
- Não usar assets protegidos como se fossem finais.
- Separar inspiração de cópia.
- Explicar como a proposta melhora legibilidade ou identidade.
- Preferir variações pequenas e comparáveis.
- Marcar claramente o que é placeholder.

## Personagens

Toda proposta de personagem deve cobrir:

- conceito técnico representado;
- arquétipo de gameplay;
- silhueta;
- paleta inicial;
- golpes ou piadas visuais;
- risco de escopo;
- como funcionaria em placeholder.

Usar `docs/templates/character-concept.md`.

## Moodboards

Todo moodboard deve cobrir:

- objetivo;
- referências;
- paleta;
- formas;
- UI ou feedback de combate;
- o que está fora de escopo;
- riscos.

Usar `docs/templates/mood-proposal.md`.

## Questões em aberto

- Resolução base dos sprites.
- Pixel art, cartoon vetorial, frame-by-frame ou híbrido.
- Proporção dos personagens.
- Nível de exagero visual.
- Regras de UI durante combate.
- Critério de aceite para sprite "candidato" versus "placeholder".
- Como representar Rust, Java e outras linguagens sem copiar mascotes ou logos como arte final.
