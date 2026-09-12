# Arquivo de produção — composição substituída

Este documento preserva a procedência da tentativa modular anterior. Seus
campos, composições e instruções de runtime foram substituídos pelos
[quadros pintados](README.md). Os PNGs antigos não são carregados pelo jogo.

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
Duke ocupa 29–41 s; Old C, 41–53 s; logo, 53–60 s.
Alterar a duração global exige sincronizar `Story` e o gerador da trilha.
Posições, enquadramento, apoios, tempos internos e frases não exigem recompilar.
**F5** recarrega o conjunto validado durante a apresentação, conservando o
relógio. Erros de imagem, referência ou sequência preservam os assets anteriores.

Textos narrativos das biografias estão em
[pt-BR.json](../../texts/pt-BR.json), nas chaves `opening.duke.*` e `opening.c.*`.
Os livros são deliberadamente sem rótulos ou letras. O monitor mostra sua
traseira à câmera; a tela é voltada a Old C. Não há texto de terminal nessa vista.

## Composição e perspectiva

O apoio da limousine é o contato da roda dianteira próxima com o asfalto.
Posição e tamanho crescem juntos da esquerda distante até a guia diante do
prédio; o carro mantém rotação zero e fica parado durante a saída de Duke.
O sprite tem perspectiva 3/4 rasa própria, sem rotacionar um perfil para tentar
imitar profundidade. Duke anda pelo lado da calçada, atrás do carro.

A reunião contém seis diretores, além de Duke na cabeceira e a secretária em
pé com bloco. Câmera afastada e escalas por fileira preservam o espaço da sala.
As pessoas são desenhadas antes do tampo. `foreground: { after, polygon }` em um
track permite desenhar novamente somente seus braços depois da mesa: `polygon`
é um contorno fechado de pontos `[x, y]` normalizados à imagem, e reutiliza
posição, escala, espelhamento e relógio do mesmo track. A borda acompanha a
manga e o cotovelo; não corta o torso em um retângulo sobre a madeira. Isso
evita pés sobre o tampo sem duplicar keyframes. O contorno tem 3–32 pontos,
cabe na imagem e possui área; `after` identifica uma camada posterior. Passes
recortados exigem rotação zero, verificada antes da recarga. O preenchimento
usa intervalos horizontais por pixel de tela e preserva vazios côncavos.

No escritório, monitor moderno visto por trás, teclado baixo sob as mãos,
café, caixa de pizza e estante são peças independentes. A identidade de Old C,
mesa, livros existentes e os três fundos originais foram preservados. Mudar a
posição de qualquer objeto não requer gerar nova imagem.

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

### Correções modulares de 12/09/2026

Novos sprites pelo mesmo `image_gen` integrado, RGBA com alpha real. Tentativas
com quadriculado incorporado foram rejeitadas. Os recortes estão somente no
catálogo; os arquivos abaixo são cópias byte a byte dos resultados selecionados.

| Arquivo | Resultado selecionado | Receita |
| --- | --- | --- |
| [limousine-perspective.png](limousine-perspective.png) | `exec-029fb7a3-7ca6-4225-9456-2f9afc6731a4.png` | [Prompt](prompts/limousine-perspective.txt) |
| [old-c-props-v2.png](old-c-props-v2.png) | `exec-1a53dcd2-6c2e-4c01-80ef-1074c2ad6617.png` | [Monitor, café e pizza](prompts/old-c-props.txt) |
| [old-c-keyboard-bookshelf.png](old-c-keyboard-bookshelf.png) | `exec-4ec54d05-cc0b-4995-96c5-2b37b2de2edc.png` | [Estante](prompts/old-c-bookshelf.txt) |
| [old-c-keyboard.png](old-c-keyboard.png) | `exec-7cfb8838-27f3-4fc9-8fd7-3af572fc7fba.png` | [Teclado baixo](prompts/old-c-keyboard.txt) |
| [meeting-people-v2.png](meeting-people-v2.png) | `exec-61e5c8ea-9ca3-408d-8220-90d2d6eae4c2.png` | [Participantes](prompts/meeting-people.txt) |

`old-c-props-v2.png` usa somente monitor/café/pizza; a estante tem recorte
isolado no segundo atlas. O teclado inicial desse atlas não é instanciado:
a peça final usa a imagem avulsa com perspectiva baixa correta para as mãos.

## Integridade dos PNGs selecionados

- `environments.png`: `8272d5659d296d9f927af75bb1d9ad156c8c6e8aa055cf8abcb00aca64fe423f`.
- `actors-props.png`: `53678f8f8ec86dabe229d7e967508825033d1fb268a87bac0d400b767624f12e`.
- `limousine-perspective.png`: `eb0286ad9306e279c1194bccf6e19d81c42c647a47277bc6ea06754e88b9f155`.
- `meeting-people-v2.png`: `f860aedcd28e32315f0b1023ddff48407dd51253994d41e4c5ed2598b0bc9171`.
- `old-c-props-v2.png`: `3f2c07b63a448d5acbd50b6bda2fe4aa6ae3cb80fa9bef276fac984ad5bb5e52`.
- `old-c-keyboard-bookshelf.png`: `0c93365520c7463dfcf735247a0a3d09b822c38921d0ce3d0cc2ff775309b4ab`.
- `old-c-keyboard.png`: `bacac00e5b57e32359f8e44c461eea671151830f20c2bea0e60e7d05dd657761`.
