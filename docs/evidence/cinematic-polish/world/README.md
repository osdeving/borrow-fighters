# Junções e fundo da rua

Capturas do renderer nativo em quatro posições:
[início](start.png), [meio](middle.png), [vizinhança](neighborhood.png)
e [extremidade direita](end.png).

Pilares de alvenaria ficam atrás das fachadas e fecham as margens transparentes
entre lotes. A última fachada parcial cobre os 128 pixels restantes da rua;
nenhum morro distante aparece como fresta até a calçada nesses enquadramentos.
Todas as peças mantêm o apoio y355 e o piso de Rust permanece y580.

`distance_scroll=0.90` conserva uma diferença discreta entre morros e fachadas.
O deslocamento parte do centro da câmera: zoom não cria movimento lateral
entre os planos. O movimento pode ser conferido na [sequência da pipa](../kite/README.md)
e nas [travessias](../gait/README.md), que usam o mesmo renderer.

[Mapa e catálogo](../../../../assets/adventure/world/README.md) ·
[Prompt e referência do novo pilar](../../../../assets/adventure/world/prompts.md).
O pilar foi gerado com a ferramenta integrada imagegen; a fonte RGBA permanece
em `assets/adventure/world/boundary-pillar.png`, com recorte somente no catálogo.

Reprodução, em sessão gráfica:

```sh
python3 docs/evidence/cinematic-polish/world/capture.py
```

O harness posiciona Rust para inspecionar o cenário; a evidência de movimento
do capítulo usa comandos públicos e está no vídeo das travessias.
