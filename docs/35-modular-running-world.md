# 35 — Corrida e mundo lateral modular

A revisão parte do checkpoint `6740d6d`, criado antes das alterações conforme
pedido. O prólogo agora tem 4608 pixels de largura: Rust começa em x160,
a chegada da EP é acionada em x3148 e a aterrissagem ocorre em x3548. O
capítulo reutiliza essa rua e acrescenta travessa de 4096 e passagem de 3840
pixels. A corrida usa aceleração até 455 pixels/s, com freio progressivo.

## Montagem e espaço

O mapa físico decide limites, spawn, acontecimentos, obstáculos, inimigos e
regiões de interação. A imagem é uma representação dessas coordenadas.
Somente o fundo distante usa parallax discreto (`distance_scroll = 0,90`); fachadas e objetos narrativos
acompanham o chão, sem deslizar em relação a pessoas ou triggers.
O centro da câmera ancora o parallax, inclusive durante o zoom. Pilares
independentes, atrás das fachadas, fecham as junções entre lotes.

Seis fachadas independentes — casas, sobrado, restaurante, café e portão —
combinam-se com mercearia, ponto de ônibus e adereços existentes. Todas têm
apoio inferior central em y355. A calçada e o canteiro foram extraídos da
pintura original e repetem com proporção fixa, alternando espelhamento para
coincidir as bordas. Rust continua apoiado no piso y580. A câmera só envia
os trechos visíveis; aumentar uma rua não amplia nenhuma textura.

A vizinhança original usa `hub_origin`: esse deslocamento conserva mercearia,
porta, moradores, menino, ciclistas, trânsito e acidente como um grupo local.
A pipa abre a tomada nesse grupo; a câmera observa a vizinhança antes de um
corte coberto por fade para Rust. Os POIs do capítulo usam a mesma posição
da vizinhança.

| Alteração | Arquivo |
| --- | --- |
| Comprimento, limites, spawn e chegada da EP | [world/map.json](../assets/adventure/world/map.json), seção `prologue` |
| Casas, restaurante, café, portão e ponto de ônibus | [world/map.json](../assets/adventure/world/map.json), instâncias de cada cena |
| PNG, recorte, escala-base e apoio de uma fachada | [world/catalog.json](../assets/adventure/world/catalog.json) |
| Comprimento, limites, POIs, carga, inimigos e saídas do capítulo | [chapter/world.json](../assets/adventure/chapter/world.json) |
| Rig, trajetórias dos pés, braços e inclinação de Rust | [run-rig.json](../assets/adventure/locomotion/run-rig.json) |
| Distância por passada e seletores Walk/Run | [motion.json](../assets/adventure/locomotion/motion.json) |
| Descida, câmera, poeira e impacto da EP | [ep-arrival.json](../assets/adventure/street/ep-arrival.json) |
| Peças e movimentos de Duke/Old C | [scenes.json](../assets/adventure/opening/scenes/scenes.json), [catalog.json](../assets/adventure/opening/scenes/catalog.json) |
| Falas, legendas e comandos | [pt-BR.json](../assets/adventure/texts/pt-BR.json), [chapter-texts.json](../assets/adventure/chapter/chapter-texts.json) |

Para aumentar uma cena, ajuste seus limites/saída e acrescente instâncias
dentro da largura desejada. O prólogo e o retorno à rua do capítulo representam
o mesmo lugar: mantenha as larguras e a posição da vizinhança consistentes
entre os dois arquivos. Reabra a sessão depois de alterar geometria ou
composição do mundo. F5 continua recarregando textos, biografias e animação.

## Movimento e apresentações

**A/D e direcional correm**; **V/RT chuta**. O rig exclusivo de corrida tem
corpo, dois braços e três peças para cada perna. O relógio de passada depende
da distância realmente percorrida, com apoio fixo durante contato, alternância
de pernas e suspensão. Aproximações cuidadosas às pessoas, o quarto e o
gesto de pesar conservam a caminhada. Os membros e seus movimentos são
editáveis separadamente; nenhum frame de gameplay depende de vídeo pronto.

A EP cai durante aproximadamente 0,97 s, com aceleração, crescimento da
silhueta distante e rastros ligados à velocidade. A câmera volta ao plano
jogável antes do contato; ajoelhar, poeira, som e tumulto seguem o mesmo
relógio. Pausa, skip e retry preservam as transições.

Duke e Old C receberam uma revisão posterior com pinturas completas por tomada
para corrigir perspectiva e proporções. A limousine já aparece estacionada
paralela à guia; a reunião inclui executivos e uma secretária em pé. Old C
trabalha num setup moderno e estuda em outro enquadramento. Os livros não têm
lettering sobreposto. [Acabamento e capturas](36-cinematic-polish.md).
A apresentação solo de Rust foi retirada: Old C leva diretamente
ao logo em 53 s, e a abertura termina em 60 s; Rust permanece no elenco do logo.

## Verificação

[Evidências nativas e resultados](evidence/modular-running-world/README.md)
registram a revisão visual, percurso e testes. A arquitetura mantém dados
tipados pequenos, sem ECS; a decisão está na
[ADR 0029](adr/0029-modular-running-world.md). O
[diário](worklogs/modular-running-world.md) permite retomar a implementação.
