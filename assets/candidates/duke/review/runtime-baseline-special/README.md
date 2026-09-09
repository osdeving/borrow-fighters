# Especial com baseline no Combat Lab

Captura nativa após a correção do Lab, com `BORROW_FIGHTERS_SPRITE_CANDIDATES=1`; manifesto visual candidato e `combat_manifest` baseline conferidos no relatório (considerando serialização f32). Fonte completa: `target/art/duke-lab-baseline-special-final`. Foram capturadas 16 amostras do especial.

Primeiro frame de emissão: [special-f001-idle.png](special-f001-idle.png), quadro `special_00`, orientação Right, tick 1. Os grãos saem junto à boca da arma no primeiro frame, mantendo a arma separada do projétil.

A origem reconstruída da posição do projétil, descontando um tick de movimento medido entre amostras, é `(146.667, -117.333)` em relação ao centro inferior do corpo. Coincide com a origem baseline `(146.667, -117.333)`. Relatório completo preservado em [capture-report.json](capture-report.json).

Baseline SHA256: `f46c78756f455d3a741bdac37ca66d88711e77b5d048fc8ff7b021ed062d847d`. Candidato visual SHA256: `e3332d284173a229457d90cf4d30a20749cae11fedd63d4c98fa2adc587a4afe`.

Esta revisão confirma a apresentação da emissão no primeiro frame e a origem carregada. Não substitui revisão artística final, movimento nas duas orientações ou transições de partida. Specs, colisões, origem e fórmula do renderer não foram alterados. Estimativas de vantagem e dummy do Lab continuam baseados em MoveSpec.
