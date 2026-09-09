# Arte revisada — Borrow Fighters

A rodada atual acrescenta **64 desenhos próprios de reação para Python e C++**:
oito clips opcionais por personagem, incluindo reação sincronizada a cada
pancada da rajada. Cada uma passa a 33 clips / 120 quadros. Os seis manifestos
candidatos somam 161 clips / 574 quadros; os atlas cinematográficos separados
ficam fora desse total. Ver [vídeo atual](../showcase/python-cpp-reactions-2026-09-09.mp4),
[evidências por contato](../../docs/evidence/python-cpp-reactions/README.md) e
[cobertura](../../docs/19-sprite-production-coverage.md).

A rodada anterior de [assinatura e arremessos](../../docs/21-signature-spectacle-and-throws.md)
deixou 25 clips por selecionável, 510 quadros de ator incluindo Go e 30 efeitos
separados. Os panoramas/GIFs e laudos abaixo registram os conjuntos anteriores;
os clips `reaction_*` novos têm fontes, prompts e revisão próprios na
[produção de Python](../production/python/reactions-2026-09-09/README.md) e
[de C++](../production/cpp/reactions-own-2026-09-09/README.md).

Na finalização anterior, os seis lutadores tiveram **120 animações, 378 quadros selecionados e os dez golpes de cada personagem**, com alpha real e metadados explícitos. Go recebeu uma nova identidade semirrealista conforme o pedido do usuário; a versão cartoon anterior foi arquivada.

| Personagem | Clips / quadros | Panorama | World: movimento e dez golpes | Laudo atual |
|---|---:|---|---|---|
| Rust | 20 / 71 | [Imagens](rust/review/overview.png) | [Vídeo](rust/review/refinement-2026-09-07/world-motion-and-attacks.mp4) | [Revisão final](../production/rust/finalization-review.md) |
| Duke / Java | 20 / 61 | [Imagens](duke/review/overview.png) | [Vídeo](../production/duke/finalization-evidence/runtime-motion.mp4) | [Revisão final](../production/duke/finalization-review.md) |
| Go — novo | 20 / 60 | [Imagens](go/review/overview.png) | [Vídeo](go/review/refinement-2026-09-07/world/runtime-motion.mp4) | [Revisão final](../production/go/finalization-review.md) |
| C | 20 / 63 | [Imagens](c/review/overview.png) | [Vídeo](c/review/refinement-2026-09-07/world/runtime-motion.mp4) | [Revisão final](../production/c/finalization-review.md) |
| Python | 20 / 62 | [Imagens](python/review/overview.png) | [Vídeo](python/review/refinement-2026-09-07/world/runtime-motion.mp4) | [Revisão final](../production/python/finalization-review.md) |
| C++ | 20 / 61 | [Imagens](cpp/review/overview.png) | [Vídeo](cpp/review/refinement-2026-09-07/world/runtime-motion.mp4) | [Revisão final](../production/cpp/finalization-review.md) |

O [master do novo Go](../production/go/reference/master.png) e seu [contrato de identidade](../production/go/reference/README.md) orientam novas gerações por ação. A [galeria do Go antigo](go-cartoon-archive/review/overview.png), a [comparação antiga dos seis idles](roster-review.png) e os [resultados do primeiro lote](runtime-matches/README.md) são histórico, não aprovação da identidade substituta.

[Comparação atual dos seis lutadores no tamanho do jogo](roster-final-review.png).

Cada pasta `review/` dos seis personagens contém 20 GIFs e 20 painéis quadro a quadro, com escala/pivô/duração do manifesto. Os GIFs mostram duas orientações e fundos claro/escuro; sua pausa final serve apenas à revisão. Os vídeos novos preservam os 1.845 estados de simulação em 30,75 s a 60 fps, mantendo a última captura entre amostras, sem interpolar poses. Lab, World, partidas nativas e o Studio de Rust/Go têm seu alcance exato registrado nos laudos e na [matriz de cobertura](../../docs/19-sprite-production-coverage.md).

Os projéteis ficam separados do corpo. Rust recebeu limpeza de alpha da engrenagem existente; C, Go e C++ reaproveitam os seus. Os [grãos de Duke](../production/duke/projectile/README.md) foram gerados novamente e o [data stream de Python](python/python-projectile.png) substitui a textura inicial que continha a personagem caída. Colisões, origem, velocidade e dano foram preservados.

A arte revisada é carregada por padrão:

```bash
cargo run -- --fight --p1 python --p2 cpp
cargo run -- --lab combat --character rust --move light_punch
# Comparação com os placeholders preservados:
BORROW_FIGHTERS_SPRITE_CANDIDATES=0 cargo run -- --fight --p1 python --p2 cpp
```

O valor `1` continua selecionando explicitamente os atlas revisados. O carregador
exige os 25 clips anteriores para os cinco selecionáveis e 20 para Go; os oito
`reaction_*` de Python/C++ são opcionais. Conjunto ausente, incompleto ou inválido
mantém o fallback. O nome da pasta `candidates/` permanece como endereço de
integração; o estado artístico é o registrado acima. A
[produção por ação](../production/README.md) preserva referências, fontes,
prompts e revisões rejeitadas. Nenhum placeholder foi sobrescrito.
