# Rust air_punch — recalibração do pivô visual

O quadro ativo da arte existente `source-v2.png` traz uma translação vertical
embutida. O ajuste remove essa translação por um pivô virtual explícito. **Não
houve nova geração, edição de pixels, mudança de anatomia nem mudança de combate.**

## Medição e decisão

A inspeção usou a fonte RGB, a folha com alpha, os quatro quadros exportados e as
capturas históricas do Combat Lab nos ticks 1, 6 e 14. As capturas do Lab incluem
movimento físico entre amostras; a comparação de continuidade foi feita em uma
prévia de âncora fixa, para separar a pose da trajetória real.

| Marco na fonte | Preparação 0 | Ativo 1 | Retorno 2 | Retorno 3 |
|---|---:|---:|---:|---:|
| Topo do cabelo, alpha ≥ 128 | 109 | 153 | 109 | 109 |
| Centro aproximado da fivela | 363 | 407 | 365 | 370 |
| Pivô Y anterior | 650 | 650 | 650 | 650 |
| Pivô Y corrigido | 650 | **694** | 650 | 650 |

O cabelo foi medido em ROIs explícitas; a fivela foi localizada manualmente nas
imagens. Os dois marcos principais do ativo estão 44 px abaixo da preparação. A
escala existente `0.497217` transforma a correção em `−21.877548 px` de mundo. O
pivô preparado arredonda de `323` para `345`, produzindo uma correção de 22 px no
runtime. A cabeça permanece inclinada durante o golpe; a cintura deixa de cair
inteira junto com ela.

## Contato preservado

O retângulo aproximado da luva ativa na fonte global é
`x=885..953, y=443..497`. Com o pivô global X inalterado em 743 e o novo Y 694,
projeta-se em `x=70.60..104.42, y=−124.80..−97.95` em relação à âncora.
A caixa baseline do AirPunch é
`x=50.67..146.67, y=−130.67..−74.67`; a luva permanece dentro dela. Nenhum campo
`combat`, origem, dano, alcance, startup, frame ativo, recovery ou dimensão física
foi alterado. As durações continuam `83/150/66/67 ms`.

- [Comparação de contato antes/depois, nos dois sentidos](../../../../target/sprite-production/rust-air-punch-pivot-review/active-contact-before-after.png).
- [Sequência anterior em âncora fixa](../../../../target/sprite-production/rust-air-punch-pivot-review/before/review/air_punch-frames.png).
- [Sequência recalibrada em âncora fixa](../../../../target/sprite-production/rust-air-punch-pivot-review/after/review/air_punch-frames.png).
- [Prévia GIF recalibrada](../../../../target/sprite-production/rust-air-punch-pivot-review/after/review/air_punch.gif).
- [Medições, método e hashes dos PNGs preservados](pivot-calibration-2026-09-07.json).
- [Snapshot integral anterior do action.json](action-before-pivot-calibration-2026-09-07.json).

## Limite da verificação

Os painéis antes/depois e o overlay de contato foram examinados estaticamente em
tamanho runtime, nas duas orientações e em fundos claro/escuro. A fivela se
mantém na linha da preparação e a mão continua dentro da caixa. Os hashes de
`source-v2.png`, `sheet-v2.png`, `keyed.png` e `prepared.png` foram preservados.

Esta alteração é somente do `action.json` de produção. O agente responsável pelo
piloto deve atualizar a nota correspondente no plano de revisão, reexportar o
candidato e capturar o runtime atualizado. O GIF é uma prévia produzida pelo
renderizador auxiliar existente; a presente revisão não afirma reprodução
contínua observada nem validação nativa do novo pivô. O `R` do torso ainda
espelha junto da arte e não foi modificado nesta recalibração.
