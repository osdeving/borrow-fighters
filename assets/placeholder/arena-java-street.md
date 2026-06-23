# Java Street Arena

## Status

Placeholder candidato derivado de `assets/references/cenario-java.png`.

## Arquivos

- `assets/references/cenario-java.png`: referencia original.
- `assets/placeholder/arena-java-street.png`: fonte placeholder em 960x540, renderizada pela janela runtime em 1280x720.

## Uso

Arena atual do prototipo greybox. A composicao foi reduzida para a resolucao base da janela sem crop agressivo, preservando:

- fachada Java como sinal visual forte;
- rua larga e linha de chao legivel perto do `FLOOR_Y` atual;
- espaco central suficiente para lutadores, projeteis, hitboxes e HUD.

## Geracao

A fonte placeholder foi redimensionada para `960x540` com leve reducao de brilho/contraste para preservar leitura dos lutadores e overlays. No runtime atual, o renderer desenha essa fonte ocupando a janela `1280x720`.

## Limitacoes

- Ainda e placeholder e nao deve ser tratada como arte final.
- O fundo e mais claro e detalhado que a arena anterior; manter debug/HUD legiveis continua sendo criterio de aceite.
