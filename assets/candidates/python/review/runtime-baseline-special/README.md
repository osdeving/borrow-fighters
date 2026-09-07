# Especial com baseline no Combat Lab

Captura nativa após a correção do Lab, com `BORROW_FIGHTERS_SPRITE_CANDIDATES=1`; manifesto visual candidato e `combat_manifest` baseline conferidos no relatório (considerando serialização f32). Fonte completa: `target/art/python-lab-baseline-special-final`. Foram capturadas 16 amostras do especial.

Primeiro frame de emissão: [special-f001-idle.png](special-f001-idle.png), quadro `special_00`, orientação Right, tick 1. O novo efeito de dados sai da boca da cobra no primeiro frame. O recorte antigo da personagem caída não aparece; o PNG candidato 144 × 68 produz canvas de 86,4 × 40,8 px pela fórmula existente.

A origem reconstruída da posição do projétil, descontando um tick de movimento medido entre amostras, é `(205.333, -157.333)` em relação ao centro inferior do corpo. Coincide com a origem baseline `(205.333, -157.333)`. Relatório completo preservado em [capture-report.json](capture-report.json).

Baseline SHA256: `a593f46d43912b388a8d4d822f185d70e8489d1d180c5d5cdf051f1989385ecb`. Candidato visual SHA256: `41a54fc3c2a425a197ed23295a039ff014838148204375e4c281ef9e4ac03155`.

Esta revisão confirma a apresentação da emissão no primeiro frame e a origem carregada. Não substitui revisão artística final, movimento nas duas orientações ou transições de partida. Specs, colisões, origem e fórmula do renderer não foram alterados. Estimativas de vantagem e dummy do Lab continuam baseados em MoveSpec.
