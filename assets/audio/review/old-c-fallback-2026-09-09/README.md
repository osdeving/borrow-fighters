# Old C — fallback temporário de voz

Old C passa a usar seis gravações já aprovadas de Rust, com autorização explícita
do usuário. Rust permanece idêntico. A troca retira do runtime os recortes de
Volvion que foram apontados no playtest como estranhos; não pretende representar
uma nova voz exclusiva de Old C.

[**Ouvir a comparação e os candidatos**](index.html) ·
[Reel antes/depois, 18,25 segundos](before-after.ogg) ·
[Índice de tempos](timeline.json) ·
[Registro completo de fontes, mixagem e hashes](../../production-old-c-fallback-2026-09-09.json)

O reel apresenta **antes e depois de cada ação**, com 0,5 s de silêncio entre
clips. É uma comparação mono isolada, com os volumes do manifesto; não é uma
captura do áudio do jogo. Os players da página permitem ouvir cada arquivo.

| Uso | Antes | Depois | Gravação selecionada |
| --- | --- | --- | --- |
| Soco curto | 0.828 s | 0.396 s | `attack-borrow-jab-01.ogg` |
| Esforço de ataque | 1.159 s | 0.535 s | `attack-lifetime-01.ogg` |
| Soco forte | 0.780 s | 0.759 s | `attack-heavy-01.ogg` |
| Variação de ataque | 1.079 s | 0.660 s | `attack-kick-01.ogg` |
| Dano | 1.361 s | 0.516 s | `hurt-01.ogg` |
| Defesa | 0.737 s | 0.279 s | `block-01.ogg` |
| Entrada do super | 1.079 s | 1.068 s | `Rust attack-heavy-01 + efeitos Kenney preservados` |

As seis vozes são cópias byte a byte: sem pitch novo, ganho novo ou recodificação.
A entrada do super usa o esforço forte de Rust junto aos mesmos `glitch_001`
e `error_002` de Kenney, com ganhos 0,45/0,60 e atraso de 70 ms do erro. A mixagem
mantém 1,06 s, fades de 5/25 ms e normalização de pico para −3,48 dBFS antes da
codificação OGG. A mudança nessa entrada fica restrita à gravação vocal e ao
resultado da sua soma com as camadas existentes.

O manifesto inteiro, incluindo volumes, pitches, bindings e música, permaneceu
byte a byte igual. Os outros **87 arquivos de áudio** foram conferidos por SHA256,
incluindo todas as vozes/entradas de Rust, Duke, Go, Python e C++. Os sete arquivos
atuais de C decodificam sem amostras clipadas; o maior pico medido é −2,88 dBFS.

## Busca e limite de avaliação

A busca foi limitada a duas alternativas CC0 em páginas dos próprios autores:

- [Kenney — Voiceover Pack (Fighter)](https://kenney.nl/assets/voiceover-pack-fighter):
  inventário com locuções de contagem, modo e vitória, sem conjunto correspondente
  de esforços de ataque/dor/defesa. [Prepare yourself](kenney-prepare-yourself.ogg)
  está aqui somente como amostra de referência, fora do manifesto.
- [xathien — Steampunk Fantasy Voices](https://opengameart.org/content/steampunk-fantasy-voices):
  o autor descreve as interpretações como caricaturas. Foram guardados os
  candidatos [Guard](Guard_Attack_001_0.wav) e [Knight](Knight_Attack_001_0.wav)
  sem processamento, para uma audição humana futura; não foram adotados.

Não foi possível fazer audição subjetiva nesta execução: a ferramenta informou
que o modelo não aceita entrada de áudio. Portanto, não há alegação de que um
candidato novo foi ouvido e aprovado. A decisão usa o fallback de Rust já
autorizado, com comparação pronta para revisão humana.

## Créditos e procedência

- Depois: **Brandon Song / wolfwoot**, [Voice Clip Pack — Male Adventurer RPG](https://opengameart.org/content/voice-clip-pack-male-adventurer-rpg), CC0 1.0. Crédito sugerido: `Voice clips by Brandon Song`.
- Antes: **Volvion**, [Old Man Noises](https://freesound.org/people/Volvion/sounds/609788/), CC0 1.0. Os sete arquivos em `before/` são as versões anteriormente distribuídas.
- Glitch/erro e locução candidata: **Kenney**, CC0 1.0, [Interface Sounds](https://kenney.nl/assets/interface-sounds) e [Voiceover Pack (Fighter)](https://kenney.nl/assets/voiceover-pack-fighter).
- Guard/Knight candidatos: **xathien**, CC0 1.0, página Steampunk acima.

URLs de download, hashes das fontes e das saídas estão no registro JSON. Este
diretório serve à revisão e não é selecionado pelo empacotador de runtime.
