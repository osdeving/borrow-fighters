# Novo Go — Sprite Studio nativo

O manifesto canônico foi aberto pelo seletor GTK real e percorrido pelos botões de clip e `Next frame` via AT-SPI: **20 clips, 60 quadros, zoom 1.00x**. Cada captura é o framebuffer da janela nativa, obtido com ImageMagick `import -window 0xa00003`. O JSON exibido pela interface é idêntico ao candidato; pivô X/Y, duração e escala dos 60 quadros foram comparados ao manifesto.

O [relatório](review-captures.json) registra ações, quadros, valores da interface e hashes antes/depois de manifesto, atlas e métricas físicas. Os hashes permaneceram iguais; nenhum ajuste foi salvo. Os PNGs originais da UI são preservados, e os painéis abaixo recortam o centro da janela sem redimensionar nem retocar a arte. A área inclui o transporte nativo para conservar os pés do anti-air alto.

| Clip | Quadros inspecionados | Leitura visual |
|---|---:|---|
| [idle](panels/idle.png) | 3 | Três guardas com respiração discreta; cabeça, pelagem, faixa e apoios coerentes. |
| [walk](panels/walk.png) | 4 | Quatro desenhos distintos, passo e recolhimento legíveis; ciclo de 240 ms preservado. |
| [jump](panels/jump.png) | 3 | Três poses suspensas distinguem impulso, recolhimento e extensão de queda; corpo inteiro. |
| [crouch](panels/crouch.png) | 2 | Duas alturas selecionadas: entrada e sustentação baixa; mãos e patas completas. |
| [block](panels/block.png) | 3 | Guarda, absorção com tronco e retorno; proteção das mãos permanece legível. |
| [crouch_block](panels/crouch_block.png) | 3 | Três guardas baixas com joelho/mão íntegros; mesma identidade e volume corporal. |
| [hit](panels/hit.png) | 3 | Reação aberta, contração e retorno; deformação do tronco comunica dano. |
| [punch_light](panels/punch_light.png) | 3 | Preparação, golpe curto e guarda; ausência de recortes de mão/antebraço. |
| [punch_heavy](panels/punch_heavy.png) | 3 | Avanço profundo e braço estendido contrastam com a preparação e a recuperação. |
| [kick](panels/kick.png) | 3 | Joelho recolhido, sola avançada e recolhimento; apoio da perna restante visível. |
| [sweep](panels/sweep.png) | 3 | Apoio baixo de mão/pata, perna lateral elevada e retorno mais alto; extremidades completas. |
| [overhead](panels/overhead.png) | 3 | Braço preparado acima da cabeça, golpe descendente e retorno à guarda. |
| [throw](panels/throw.png) | 3 | Mãos abertas, gesto curto de agarrar/puxar e guarda; sem vítima incorporada. |
| [anti_air](panels/anti_air.png) | 3 | Punho sobe acima da cabeça; cabeça e patas inteiras nas três poses. |
| [air_punch](panels/air_punch.png) | 3 | Preparação recolhida, tronco inclinado e golpe descendente, recolhimento suspenso. |
| [air_kick](panels/air_kick.png) | 3 | Joelho preparado, extensão horizontal da pata e retorno; nenhuma extremidade recortada. |
| [special](panels/special.png) | 3 | Palma de emissão presente no primeiro quadro, acomodação e guarda; projétil separado. |
| [spawn](panels/spawn.png) | 3 | Postura inicial, reverência e chegada à guarda; faixa e patas contínuas. |
| [victory](panels/victory.png) | 3 | Relaxamento, punho elevado e pose sustentada, com corpo inteiro e apoio visível. |
| [defeat](panels/defeat.png) | 3 | Corpo cede, ajoelha e termina sentado no chão; cabeça, mãos e patas preservadas. |

A nova identidade foi mantida: gopher adulto azul-ardósia, peito claro, focinho modelado, olhos pequenos, calça carvão, faixa preta, luvas sem dedos e patas nuas. Não foi encontrado defeito impeditivo de corpo recortado ou troca de identidade nos quadros inspecionados. O contorno claro na pelagem acompanha a luz pintada do master.

O painel do Studio informa idle_00 com 145 × 269 px e mostra aviso pela largura ficar 2 px abaixo da faixa genérica de idle (147–200 px). Essa silhueta estreita é compatível com o novo master; não justifica alterar corpo físico ou esticar a arte. O anti-air mede 335 px de altura por causa do punho levantado; a mesma faixa do painel é referência de idle, não restrição para todos os golpes.

Esta prova cobre carregamento, desenho e navegação quadro a quadro na orientação direita. Não comprova reprodução contínua, duas orientações, sincronização de caixas ou trajetória física no Studio. Essas verificações pertencem ao World/Lab e à partida nativa. A interface mostra zero campos de combate no novo manifesto visual; a preservação do `combat_manifest` é verificada no runtime, não por esse painel.

Comando do processo observado:

```sh
env -u WAYLAND_DISPLAY GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1 tools/sprite-studio/src-tauri/target/debug/borrow-fighters-sprite-studio
python3 target/art/go-finalization-studio/load_atspi_candidate.py
python3 target/art/go-finalization-studio/capture_atspi_review.py
```

Os [helpers de carregamento](load_atspi_candidate.py) e [captura](capture_atspi_review.py) usam a interface real e estão preservados; o ID da janela deve ser identificado novamente se a sessão for reaberta. O resize de um pixel solicita repaint ao Weston, sem modificar zoom ou conteúdo. Nenhum processo do World/Lab ou da partida foi encerrado por esta revisão.
