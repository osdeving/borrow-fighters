# Reações dos seis defensores

As três folhas mostram snapshots de **contatos reais no World**, desenhados pela
mesma função de sprites usada na luta, no showcase e nos supers do Combat Lab.
Cada linha acompanha um defensor; as colunas avançam o relógio do mesmo impacto.
A escala de 0,48 serve apenas para comparar os seis atores na mesma folha. Os
valores físicos, HP, clip e frame de cada amostra estão no
[laudo JSON](reaction-review.json).

- [Golpe leve](hit-six-defenders.png): recoil, compressão e três desenhos antes
  de terminar o stun. O último quadro mostra o retorno ao idle.
- [Golpe pesado](heavy-six-defenders.png): inclinação e recuo maiores, com todos
  os desenhos ajustados ao stun real de cada contato.
- [Super](super-six-defenders.png): lançamento, arco, aterrissagem, chão e
  recuperação progressivos; agora não existe mais o congelamento em 0,1 s.

A captura foi inspecionada visualmente em 9 de setembro de 2026. Os cinco atlas
com `heavy_hit`, `launched`, `thrown` e `knockdown` foram reaproveitados. Go usa
seus três desenhos de `hit`, com rotação e compressão contínuas para representar
voo e queda; não recebeu desenhos novos nesta rodada. Em KO, a pose permanece no
chão e não executa get-up. O lançamento de super reserva margem interna antes da
borda e mantém o arco vertical quando não há espaço horizontal.

## Verificação

[`tests/reaction_matrix.rs`](../../../../tests/reaction_matrix.rs) executa
**1.704 cenários**: 71 pares personagem/golpe suportados × seis defensores × duas
direções × com/sem metadata de colisão. A matriz cobre golpe leve/pesado, chute,
rasteira, overhead, anti-air, ataques aéreos, projétil, arremesso, assinatura e
cinemático. Cada caso exige dano real, interrupção de ataque/projétil do alvo,
pelo menos dois desenhos de reação, quatro estados de transformação e término
do voo no chão.

Os outros seis testes verificam que os três desenhos de hit cabem no stun,
guarda alta/baixa preserva a postura, recoil espelha sem mudar o corpo, dano em
alvo aéreo termina em queda protegida (inclusive golpe pesado), KO não levanta e supers não teleportam nem
empurram o alvo além da margem interna. Também passaram os testes existentes de
playback, seleção de sprites, throws/launches e sequências autorais.

O Combat Lab de golpes comuns continua sendo uma inspeção de ator e dummy
retangular opcional. A simulação de dois lutadores desse modo é específica dos
supers autorais; os outros contatos completos são vistos no showcase/luta.

## Reproduzir

```sh
cargo test --test reaction_matrix --test throws_launches --test sprite_playback --test sprite_selection
cargo run --example capture_reaction_review -- /tmp/reaction-review
```

A captura exige o mesmo ambiente gráfico Raylib/OpenGL do jogo, mas abre uma
janela oculta, não toca áudio e não envia input ao host. Nesta revisão foi usado
Xephyr isolado. O exemplo produz três PNGs e o JSON diretamente do framebuffer,
sem montagem ou pintura posterior das imagens.
