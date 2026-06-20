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

## Moods iniciais para explorar

| Mood | Ideia | Risco |
|---|---|---|
| Terminal Arcade | UI e arena com energia de terminal, neon moderado e debug overlays | Virar visual escuro demais |
| Compiler Lab | Ambiente de laboratório, warnings, erros e processos de build como cenário | Ficar técnico demais e pouco divertido |
| Mascot Fight Club | Mascotes cartunescos em arena exagerada | Aproximar demais de mascotes existentes |
| Office Surreal | Mesa, docs, tickets e reuniões virando arena absurda | Parecer piada corporativa genérica |

Nenhum mood está aprovado como direção final. Propostas devem comparar pelo menos uma vantagem e um risco.

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
