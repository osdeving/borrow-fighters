# 36 — Acabamento do prólogo e continuidade da rua

Esta revisão prioriza a apresentação de Duke e Old C, compara capturas reais
com a direção de Ada/Python/C++ e corrige movimento e continuidade do cenário.

## Biografias

Cada personagem recebe duas pinturas completas, com câmera interpolada suave.
Duke aparece junto à limousine estacionada paralela à guia na Paulista e
depois à cabeceira de uma reunião. Old C aparece trabalhando no setup atual,
com monitor voltado para ele, e estudando em um segundo enquadramento.

Os PNGs são independentes por tomada. Catálogo, câmera, duração e legendas
continuam editáveis em arquivos; detalhes internos de um quadro são corrigidos
na sua própria imagem. A [ADR 0030](adr/0030-painted-biography-shots.md)
registra essa escolha autorizada pelo usuário. O mundo jogável conserva sua
montagem por peças. [Guia das biografias](../assets/adventure/opening/scenes/README.md).

## Ritmo e movimento

A abertura da pipa dura 18 segundos no conteúdo padrão: observa a pipa,
desce para menino/calçada, acompanha comércio e trânsito e corta para Rust
sob preto completo. A duração vem do último keyframe, compartilhada pelo
controle, câmera, pausa e avanço. O corte evita atravessar milhares de pixels
em poucos segundos. A queda veloz da EP continua independente desse trilho.

Na corrida, a largura dos membros acompanha o volume da pose parada sem
alterar comprimento das pernas, apoios, velocidade ou colisão. As travessias
do capítulo usam vistas de costas na ida e de frente na volta, conforme a
direção do segmento da rota; o percurso físico e os pontos de conversa se
mantêm. [Guia de movimento](../assets/adventure/locomotion/README.md).

## Continuidade do mundo

Pilares de alvenaria atrás das fachadas fecham as frestas entre lotes. A rua
inclui a fachada parcial necessária para cobrir sua extremidade direita.
O fundo acompanha 90% do deslocamento das fachadas, com diferença discreta,
e usa o centro da câmera como referência: mudar o zoom não desloca os morros
horizontalmente. [Guia do mundo](../assets/adventure/world/README.md).

| Ajuste | Arquivo |
| --- | --- |
| Quadros, câmera e duração de Duke/Old C | [scenes.json](../assets/adventure/opening/scenes/scenes.json) |
| PNG e recorte de cada quadro | [catalog.json](../assets/adventure/opening/scenes/catalog.json) |
| Ritmo da pipa e corte para Rust | [arrival-camera.json](../assets/adventure/street/arrival-camera.json) |
| Volume e trajetória das pernas | [run-rig.json](../assets/adventure/locomotion/run-rig.json) |
| Clips de corrida/travessia | [motion.json](../assets/adventure/locomotion/motion.json) |
| Junções e intensidade do parallax | [map.json](../assets/adventure/world/map.json) |
| Legendas | [pt-BR.json](../assets/adventure/texts/pt-BR.json) |

[Capturas, vídeos e verificações](evidence/cinematic-polish/README.md) ·
[Diário e recuperação](worklogs/cinematic-polish.md).
