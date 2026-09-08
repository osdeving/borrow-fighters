# Arte revisada — Borrow Fighters

A rodada atual adiciona arremessos reais, reações aéreas e cinco especiais de assinatura: **25 clips por selecionável**, 510 quadros de ator no elenco incluindo Go, e 30 efeitos separados. Fontes e prompts foram produzidos com a ferramenta de imagens integrada. Ver [cobertura atual](../../docs/19-sprite-production-coverage.md) e [registro da assinatura](../../docs/21-signature-spectacle-and-throws.md). O [vídeo atual](../showcase/signature-spectacle-2026-09-08.mp4) reúne os cinco especiais, arremesso e gancho. Os panoramas/GIFs das pastas `review` acompanham os atlas atuais; os vídeos e laudos da tabela abaixo registram a finalização anterior.

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

O valor `1` continua selecionando explicitamente os atlas revisados. O carregador exige os 20 clips e mantém o fallback quando o conjunto estiver ausente, incompleto ou inválido. O nome da pasta `candidates/` permanece como endereço de integração; o estado artístico é o registrado acima. A [produção por ação](../production/README.md) preserva referências, fontes, prompts e revisões rejeitadas. Nenhum placeholder foi sobrescrito.
