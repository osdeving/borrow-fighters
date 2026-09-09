# Lixo — Garbage Collector

[garbage-items.png](garbage-items.png): três props separados — casca de banana,
copo descartável amassado e papel embolado — em PNG RGBA 2172 × 724. Produzido
com `imagegen` integrado em 9 de setembro de 2026, sem imagem externa de entrada.

O alpha já veio correto na geração inicial: 69,07% dos pixels são transparentes.
O arquivo foi copiado intacto. Os três recortes têm largura diferente e são
normalizados pelo renderer para leitura a aproximadamente 35px no chão.

O lixo cai, assenta e espera a chegada de cada coletor; desaparece do chão no
momento da pegada e é levado até a boca. A posição dos props não calcula dano.

Prompt: [prompt.txt](prompt.txt). Fonte da ferramenta, dimensões, componentes de
alpha e SHA-256: [generation.json](generation.json). Implementação:
[authored_actors.rs](../../../../src/engine/render/authored_actors.rs).
