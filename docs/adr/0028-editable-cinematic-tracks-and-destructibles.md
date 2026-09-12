# ADR 0028 — Cenas por peças, trajetórias e destroços com estado

- Status: aceita para a implementação solicitada.
- Data: 2026-09-12.

## Contexto

As novas biografias de Duke e Old C, a aterrissagem da EP e a passagem do
capítulo precisam permitir alterações locais. O usuário pediu explicitamente
assets reutilizáveis, interpolação, textos em arquivos e crescimento organizado.
A aventura já separa geometria, progressão, catálogos visuais e renderização.

## Decisão

Ampliar o catálogo `PieceCatalog` existente, sem introduzir ECS: os problemas
atuais são autoria e composição, e os poucos atores têm estados explícitos.
Catálogos identificam imagens, recortes, apoios, sockets e clips. Cada instância
seleciona uma peça e sua transformação; várias instâncias compartilham a textura.

`adventure/biography.rs` valida tomadas e interpola posição, escala, rotação e
opacidade em ticks. Câmera usa o mesmo contrato. Textos podem se prender a
objetos, acompanhando seus movimentos; títulos e legendas vêm do catálogo de
idioma. `engine/biography.rs` apenas desenha esses dados com Raylib.
Cada biografia tem duas tomadas somando 720 ticks; seus tempos internos e
composições são editáveis. Alterações de duração global precisam manter o
contrato entre `Story` e a trilha original.

A chegada da EP usa um único relógio físico, incluindo tomada alta, retorno à
câmera jogável, impacto e recuperação. A evacuação observa o contato com o
chão. O arquivo de chegada possui tuning próprio; o renderer não cria uma
segunda EP nem decide quando o combate começa.

Locomoção usa distância realmente percorrida, apoios de poses e um clip
compartilhado entre gameplay, aproximações e saída do quarto. Chutes usam o
relógio autoritativo do ataque. A escala corporal não deriva de cada recorte.

Destroços são instâncias espaciais com IDs, resistência, arte e fragmentos no
`world.json` do capítulo. O estado conserva dano e queda; cada golpe aplica
dano uma vez no contato. Só a abertura física permite seguir. O encontro
seguinte contém dois inimigos configurados separadamente; retry restaura o
grupo inteiro no checkpoint.

## Consequências

Uma caixa, ator, livro, computador ou limousine pode ser reposicionado ou ter
seu recorte/arquivo trocado isoladamente. Editar JSON não exige gerar arte.
PNGs separados e recortes de atlas são igualmente suportados. Texturas são
carregadas uma vez por caminho. Dados inválidos produzem diagnóstico e a
recarga de cada conjunto preserva sua versão anterior.

O domínio continua isolado do Versus conforme a [ADR 0021](0021-isolated-adventure-experiment.md).
ECS pode ser reavaliado se o volume de entidades ou combinações de componentes
demonstrar necessidade; não resolve por si só autoria, assets ou interpolação.

## Verificação

Testes puros de trajetórias, chaves/referências inválidas, colisão/dano por
contato, derrota/retry e vitória do grupo; capturas nativas das cenas e
animações. Fmt, Clippy, matriz de features, fronteiras e dependências dos
pacotes. [Diário e resultados](../worklogs/cinematic-expansion.md).
