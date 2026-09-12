# C++ — dois braços legíveis no repouso

O braço posterior era desenhado antes do tronco, mas seu alvo de mão apontava
para o lado oposto do corpo: `[-24, -139]`. Isso colocava praticamente o braço
inteiro atrás da pintura do torso. A nova guarda abre a mão posterior para
`[39, -164]`, preservando a roupa, a anatomia, a ordem das camadas e as imagens.
A respiração desloca o alvo junto com o tronco. As poses de entrada e retorno
ao repouso usam a mesma correção; parada e interação também foram ajustadas.

Comparação nativa, com oito fases por ciclo:

| Orientação | Antes | Depois |
| --- | --- | --- |
| Direita | [Antes](before-idle-right.png) | [Depois](after-idle-right.png) |
| Esquerda | [Antes](before-idle-left.png) | [Depois](after-idle-left.png) |

A inspeção também incluiu [partida](clip-17-start-left.png),
[parada](clip-18-stop-right.png), [interação](clip-05-interact-right.png) e
[repouso após corrida na escala jogável](simulation-0180.png).
As duas mãos continuam distinguíveis durante a respiração e o assentamento.

O [vídeo nativo](lab-simulation-60fps.mp4) usa o renderer compartilhado pelo
laboratório e capítulo: 450 frames/ticks, 1280×720, 60 fps e 7,5 segundos.
Ele registra repouso, partida, corrida, parada, inversão e salto; não possui
áudio. A [telemetria](telemetry.jsonl), [invocação](invocation.json),
[resultado](result.json) e [FFprobe](video-probe.json) registram a captura.
FFmpeg decodificou o vídeo completo com saída 0.

```sh
cargo run --no-default-features --features adventure --bin borrow-actor-lab -- \
  --hidden --frames 450 --review /tmp/cpp-arm-review
cargo test --no-default-features --features adventure --lib \
  adventure::production::animation::tests
```

A regressão `idle_rear_wrist_clears_the_torso_and_both_hands_stay_distinct`
amostra 1.025 poses de respiração: o pulso posterior precisa permanecer fora
dos limites da pintura do tronco, as mãos devem ficar separadas e os dois
alvos devem estar ao alcance dos ossos. A comparação visual verifica também
o espelhamento, que a geometria local sozinha não aprova.
