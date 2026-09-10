# ADR 0026 — Chegada cinematográfica e moradores da rua

- Estado: aceita no experimento autorizado.
- Data: 2026-09-10.

## Contexto

A primeira rua já possui peças substituíveis e evacuação coletiva. O usuário
pediu uma chegada cinematográfica, moradores que se abrigam na mercearia,
porta de enrolar, caramelo e ambientação sonora adequada ao lugar.

## Decisão

Manter a chegada dentro de `Stage::Encounter`. `Story` expõe a fase de chegada
e retém o avanço do combate até entregar controle; o ambiente continua vivo.
Uma descrição pura da câmera fornece alvo e zoom ao renderer; somente o mundo
recebe essa transformação. Pausa omite updates, retry acordado dispensa a
chegada e restart a restaura. Avanço explícito da chegada termina a tomada.

Um módulo de vizinhança deriva moradores, cão e porta dos clocks de
`AmbientState`, sem relógio de parede nem alteração das regras do combate.
O renderer próprio usa peças do catálogo e respeita a entrada da fachada e
sua oclusão. Constantes de fechamento são compartilhadas com áudio; a porta
espera todos alcançarem o abrigo antes de baixar.

Áudio mantém ar e trânsito como camadas independentes, com volume guiado
pela idade da evacuação. O tráfego deve desaparecer após a rua esvaziar;
música e loops de trânsito não devem sugerir normalidade durante o combate.

## Consequências

A introdução, passagem de controle, trajetórias e ordem do fechamento ficam
testáveis sem Raylib. Moradores e cão continuam adereços visuais independentes,
reutilizáveis. A composição e as medidas da fachada precisam permanecer
alinhadas ao ponto de entrada; documentar essas medidas e verificá-las na
captura nativa. Não se introduz sistema genérico de cutscenes ou pathfinding.

[Escopo](../32-cinematic-neighbourhood-arrival.md).
