# 07 — Direção de Arte e Moods

## Status

Documento vivo. Nada aqui é arte final.

Este documento orienta contribuições visuais sem fechar cedo demais a identidade do jogo.

## Objetivo visual

**Borrow Fighters** deve parecer um jogo de luta 2D cartunesco, legível e engraçado, onde cultura de programação vira forma, cor, gesto e impacto.

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

O slice jogável usa arte placeholder, mas já exercita decisões importantes para a direção visual:

- arena inicial: `assets/placeholder/arena-sirius.png`, derivada de referência em `assets/references/sirius.png`;
- arenas em rotação: `assets/placeholder/arena-fortaleza.png` e `assets/placeholder/arena-java-street.png`;
- arena anterior ainda disponível: `assets/placeholder/arena-terminal-compiler-lab.png`;
- personagens atuais: Rust, Duke, Go e C em atlas placeholder com manifesto JSON;
- entrada cinematográfica: manifests separados para Rust, Duke, Go e C;
- projectile Rust: engrenagem separada do sprite do personagem;
- projectile Duke: bean separado do sprite do personagem;
- projectile Go: canal/burst separado do sprite do personagem;
- projectile C: bitstream separado do sprite do personagem;
- HUD, ajuda e debug visual são opcionais por feature flag.

O C veio de dois atlas de referencia em `assets/references/langc-03.png` e `assets/references/langc-04.png`, com fundo chroma key removido por script local. Ele deve ser avaliado como placeholder jogavel de escala, fluidez e leitura, nao como direcao final do personagem.

Nada disso é final. O valor desses assets agora é validar proporção, leitura de pose, pivots, altura do projectile, contraste com cenário e necessidades de animação.

## Linguagem visual do Linker

O Linker deve parecer uma força de ligação, não magia genérica. Ele pode aparecer como alinhamento impossível entre cabos, símbolos, partículas, luz, logs, circuitos, reflexos e gestos dos personagens.

Direções visuais:

- Ada Lovelace deve aparecer de forma sutil, quase arqueológica: inscrições, diagramas, retratos parciais, notas, máquinas e sinais esquecidos;
- Assembly deve parecer fora de fase, com partes do corpo alternando entre matéria, ausência, `0` e `1`;
- a instabilidade de Assembly deve ser sóbria e inquietante, não glitch cômico;
- entidades erráticas dos frontenzos podem ter silhuetas menores, deformadas e incompletas;
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
