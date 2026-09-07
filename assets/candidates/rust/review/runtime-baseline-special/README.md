# Especial com baseline no Combat Lab

Captura nativa após a correção do Lab, com `BORROW_FIGHTERS_SPRITE_CANDIDATES=1`; manifesto visual candidato e `combat_manifest` baseline conferidos no relatório (considerando serialização f32). Fonte completa: `target/art/rust-lab-baseline-review`. Foram capturadas 17 amostras do especial. O relatório Rust preservado inclui também três golpes; esta nota inspeciona somente o especial.

Primeiro frame de emissão: [special-f001-idle.png](special-f001-idle.png), quadro `special_00`, orientação Right, tick 1. A trilha da engrenagem permanece junto à mão estendida no primeiro frame; não há deslocamento vertical observado entre saída e gesto.

A origem reconstruída da posição do projétil, descontando um tick de movimento medido entre amostras, é `(138.667, -117.333)` em relação ao centro inferior do corpo. Coincide com a origem baseline `(138.667, -117.333)`. Relatório completo preservado em [capture-report.json](capture-report.json).

Baseline SHA256: `400128e2a525a3b2c0a6a4c3ab17ffd63bf9afdd48317664fffdcc6210f36b38`. Candidato visual SHA256: `b3bcc448f989a8c99e97f9307c2d0788def9a93ca96f205e028b42b7ea67da00`.

Esta revisão confirma a apresentação da emissão no primeiro frame e a origem carregada. Não substitui revisão artística final, movimento nas duas orientações ou transições de partida. Specs, colisões, origem e fórmula do renderer não foram alterados. Estimativas de vantagem e dummy do Lab continuam baseados em MoveSpec.
