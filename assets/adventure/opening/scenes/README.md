# Biografias por peças

[catalog.json](catalog.json) define recortes e apoios das peças;
[scenes.json](scenes.json) define tomadas, camadas, câmera, keyframes e textos
presos a objetos. O runtime interpola as transformações a 60 Hz. Não há vídeo
pré-renderizado usado pelo jogo.

Para mover a limousine, edite `duke-paulista.tracks[car].keys`. Para reposicionar
ou reutilizar um livro, edite/duplique sua instância. Para trocar somente uma
peça, aponte o respectivo `frames[].image` para outro PNG relativo a
`assets/adventure` e ajuste `source`/`anchor`. O mesmo arquivo pode conter
vários recortes; imagens avulsas também funcionam.

As listas de tracks estão em ordem de desenho. O anchor de cada sprite é seu
apoio; `scale` é relativo à largura definida no catálogo. Câmera usa `position`
como alvo e `scale` como zoom. Os keyframes são interpolados com smoothstep;
o último fica estável. Cada lista começa em tick zero. IDs de instâncias são
únicos por tomada; peças podem se repetir. `labels` seguem o parent, seu apoio,
escala e rotação, preservando a leitura do texto ao espelhar um personagem.

Cada personagem tem duas tomadas totalizando 720 ticks (12 segundos).
Duke ocupa 29–41 s; Old C, 41–53 s; Rust, 53–57 s; logo, 57–64 s.
Alterar a duração global exige sincronizar `Story` e o gerador da trilha.
Posições, enquadramento, apoios, tempos internos e frases não exigem recompilar.
**F5** recarrega o conjunto validado durante a apresentação, conservando o
relógio. Erros de imagem, referência ou sequência preservam os assets anteriores.

Textos de biografia, livros e terminal estão em
[pt-BR.json](../../texts/pt-BR.json), nas chaves `opening.duke.*` e `opening.c.*`.

## Procedência

Arte candidata criada com o `image_gen` integrado em 12/09/2026, usando as
ilustrações existentes de Python/C++ como referência de acabamento e os
retratos locais de Duke/Old C como referência de identidade.

- [environments.png](environments.png): fonte final `exec-43296947-0198-4f25-8fa1-5435423fe6cd.png`,
  três ambientes, RGB 966×1628. [Prompt](../../prompts/opening-background.txt)
  e [remoção de letras incorporadas](../../prompts/opening-background-cleanup.txt).
  A placa e o endereço gerados foram apagados pelo tool; as palavras da cena
  vêm dos arquivos de texto.
- [actors-props.png](actors-props.png): fonte `exec-e9f5c4d9-4362-474b-aba4-f1cd113531d4.png`,
  atlas RGBA 1254×1254. [Prompt inicial](../../prompts/opening-props.txt).
  Duas correções de fundo pelo mesmo tool; seleção final possui alpha real.
  A [última instrução](../../prompts/opening-props-alpha.txt) pediu somente extração do fundo, preservando as nove
  ilustrações e substituindo os pixels de quadriculado por transparência.

PNGs selecionados copiados sem regravação de pixels. Composição, legendas e
animação são dados independentes. Detalhes arquitetônicos sem interação
pertencem aos fundos; os objetos importantes de atuação são peças do catálogo.

## Integridade dos PNGs selecionados

- `environments.png`: `8272d5659d296d9f927af75bb1d9ad156c8c6e8aa055cf8abcb00aca64fe423f`.
- `actors-props.png`: `53678f8f8ec86dabe229d7e967508825033d1fb268a87bac0d400b767624f12e`.
