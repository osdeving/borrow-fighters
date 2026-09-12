# Mundo lateral

`map.json` controla limites, spawn, início da queda e ponto de aterrissagem do prólogo. `scenes` contém fachadas/objetos em coordenadas de mundo; o capítulo usa os limites/POIs de `../chapter/world.json`, sem duplicá-los nesta composição.

Cada instância possui `id`, `piece`, `position` (apoio inferior central) e `scale`. As fachadas usam y355, como a mercearia original. Rust percorre o piso y580. Os PNGs não determinam colisões nem eventos.

`distance_scroll` controla quanto o fundo acompanha as fachadas: `1` trava os
planos juntos; o valor atual `0.90` conserva uma diferença discreta. O movimento
depende do centro da câmera, para que o zoom não desloque o fundo lateralmente.
As instâncias `joint.*`, desenhadas antes das casas, usam `wall.pillar` para
fechar as frestas. Os pilares mantêm apoio em y355 e são peças reutilizáveis;
ao acrescentar uma fachada, acrescente também sua junção quando necessário.

`catalog.json` controla PNG, recorte, dimensão e apoio de cada casa, portão, restaurante ou café. Para ampliar uma rua, edite a largura e limites físicos e acrescente instâncias dentro dessa largura; não amplie as texturas. `hub_origin` desloca o grupo original da mercearia/trânsito e deve acompanhar os POIs `shop`, `driver` e `neighbour` do capítulo.

Arte criada com a ferramenta integrada imagegen, usando `../street/props.png` como referência. A saída com checkerboard foi substituída pela ferramenta por fundo uniforme magenta; `sources/facades-keyed.png` preserva o original importado. ImageMagick extraiu a chave de cor e recortes individuais; a máscara rejeita pixels com vermelho e azul dominantes sobre verde para evitar halos magenta. `pavement.png` e `planter.png` são recortes da pintura original, preservados em proporção fixa. A imagem distante repete alternando espelhamento horizontal; não há costura de cores nas bordas.

Prompts de produção em `prompts.md`. Os PNGs de produção devem permanecer no repositório; arquivos na pasta de geração pessoal não são dependências do jogo.

`boundary-pillar.png` foi gerado pela ferramenta integrada com alpha nativo,
usando `house-cream.png` como referência. O recorte e o apoio estão no catálogo;
a imagem original permanece intacta, sem chave de cor nesta peça.
