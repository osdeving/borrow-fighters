# Especial com baseline no Combat Lab

Captura nativa após a correção do Lab, com `BORROW_FIGHTERS_SPRITE_CANDIDATES=1`; manifesto visual candidato e `combat_manifest` baseline conferidos no relatório (considerando serialização f32). Fonte completa: `target/art/go-lab-baseline-special-final`. Foram capturadas 19 amostras do especial.

Primeiro frame de emissão: [special-f001-idle.png](special-f001-idle.png), quadro `special_00`, orientação Right, tick 1. O efeito de canais aparece sobre a palma emissora e segue horizontalmente; a posição usa a origem baseline.

A origem reconstruída da posição do projétil, descontando um tick de movimento medido entre amostras, é `(97.920, -123.840)` em relação ao centro inferior do corpo. Coincide com a origem baseline `(97.920, -123.840)`. Relatório completo preservado em [capture-report.json](capture-report.json).

Baseline SHA256: `b03f5fdca18a7d6f87c46bfef23fae3ce23b99ad218c1abdfa182e26ec1a6027`. Candidato visual SHA256: `59559b4d92448eda6e907e04e4c1a96e5d3b568a66bf33de4c0919954c08dfb4`.

Esta revisão confirma a apresentação da emissão no primeiro frame e a origem carregada. Não substitui revisão artística final, movimento nas duas orientações ou transições de partida. Specs, colisões, origem e fórmula do renderer não foram alterados. Estimativas de vantagem e dummy do Lab continuam baseados em MoveSpec.
