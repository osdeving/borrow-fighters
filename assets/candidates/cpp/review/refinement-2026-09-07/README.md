# C++ — evidências do refinamento

O [laudo](../../../../production/cpp/finalization-review.md) registra as gerações, escolhas e aprovação visual da caminhada, guarda baixa e chute aéreo. [Antes/depois nativo](before-after-native.png), [crouch e guarda](crouch-guard-native-keys.png) e [quadros nos dois sentidos](refined-keys-both-facings.png) acompanham a entrega.

- [Vídeo World](world/runtime-motion.mp4): 1.845 quadros, 60fps, 30,750s, codificados a partir de 624 PNGs nativos. Não há interpolação; imagens amostradas são mantidas até o próximo tick capturado.
- [Resumo World](world/capture-summary.json): cobertura dos dez golpes por jogador, dez retornos à guarda, manifesto visual realmente carregado, baseline de combate e estado final.
- [Índice de painéis](world/evidence-index.json): PNGs de origem, ticks, orientação, retângulos e relação com a âncora física. Os recortes seguem cada jogador; o vídeo e os PNGs completos preservam o deslocamento no cenário.
- [Resumo Lab](lab/capture-summary.json): 36 capturas de air_kick/crouch/crouch_block, com 12 PNGs selecionados nesta pasta.
- [Partida sem opt-in](native-match-result.png) e [hashes](native-match-assets.json): C++ × Rust, resultado 49×0, inspecionada pelo agente principal.

Os originais completos e o script dos painéis estão em `target/art/cpp-finalization-world`; o Lab completo está em `target/art/cpp-finalization-lab`. Os resumos aqui contêm somente os registros relevantes às evidências selecionadas e informam os caminhos dos relatórios completos.

Para rever o vídeo localmente:

```sh
ffplay assets/candidates/cpp/review/refinement-2026-09-07/world/runtime-motion.mp4
```

Espaço pausa; `s` avança um quadro. Entre duas amostras, quadros repetidos correspondem à retenção do PNG anterior. A revisão visual foi amostrada, com as chaves e transições citadas no laudo; os testes de cobertura e tempo não substituem essa inspeção.
