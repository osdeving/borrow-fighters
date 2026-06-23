# Python Fighter Atlas

## Status

Placeholder candidato para a primeira personagem mulher do jogo, representando Python e ciência de dados.

O visual é uma personagem adulta original com energia de pesquisadora/engenheira de IA, cobra Python em volta do corpo, camisa social branca cropped, saia colegial/collegiate até pouco acima dos joelhos e salto de combate. A direção usa inspiração temática de Fei-Fei Li e visão computacional, mas não tenta reproduzir retrato, rosto ou identidade visual real de nenhuma pessoa.

## Arquivos

- `assets/references/python-fighter-atlas-source.png`
- `assets/placeholder/python-fighter-atlas.png`
- `assets/placeholder/python-fighter.sprite.json`
- `assets/placeholder/python-data-projectile.png`

## Geração

```bash
python3 tools/art/build_python_fighter_atlas.py
```

O script usa uma fonte AI-generated já limpa para alpha, detecta as poses principais por componente visual, remove componentes cinza de sobra, repacota para a mesma grade runtime do C (`6x16`, células `384x256`) e gera um manifesto `borrow-fighters.sprite.v1` com 94 frames. Ele também exporta `python-data-projectile.png` a partir do frame `projectile_0`.

## Intenção Visual

- `punch_light`: a cobra dá um bote rápido.
- `punch_heavy`: a própria personagem soca.
- `taunt`: ela apresenta pontos, gráficos e energia de ciência de dados sem texto legível no PNG.
- `special`: fluxo de dados azul/amarelo guiado pela cobra.

## Limitações

- Ainda é placeholder e não arte final.
- A fonte gerada trouxe 77 poses principais; alguns frames vizinhos reutilizam poses para preencher o contrato de 94 frames do C.
- Já está integrada ao roster jogável, menu e CLI como `python.py`, mas ainda não possui áudio próprio nem balanceamento final.
- Hitboxes/hurtboxes por frame precisam ser revisadas no Sprite Studio antes de usar como dado de combate confiável.
- O visual deve ser revisado por leitura em movimento, escala e aceitação de personagem antes de virar candidato aprovado.
