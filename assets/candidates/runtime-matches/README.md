# Partidas nativas com candidatos

Capturas diretas das janelas do aplicativo, após combate entre duas CPUs com `BORROW_FIGHTERS_SPRITE_CANDIDATES=1`. Nenhum personagem ou cenário foi composto por fora do renderer.

| Partida | HP final P1 × P2 | Evidência |
|---|---|---|
| Rust × Rust | 9 × 0 | [Resultado](rust-vs-rust-result.png) |
| Go × C | 0 × 76 | [Resultado](go-vs-c-result.png) |
| Python × C++ | 17 × 0 | [Resultado](python-vs-cpp-result.png) |

Inspeção confirma resultado real, poses de vencedor/derrotado visíveis e apoiadas no chão, espelhamento e contraste na arena Sirius. As imagens finais não provam cada frame da transição ou fluidez de movimento. Os vídeos controlados do World e as capturas do Lab complementam esta evidência; o [registro de cobertura](../../../docs/19-sprite-production-coverage.md) delimita cada método.

Go × C e Python × C++ usam o binário reconstruído após a ligação do baseline ao Lab. Rust × Rust foi executado no piloto; os refinamentos posteriores de contato/salto receberam suas próprias capturas. Arte permanece candidata, com ressalvas nos laudos.
