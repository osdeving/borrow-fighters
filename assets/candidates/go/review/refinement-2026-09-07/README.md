# Novo Go — evidência final de 07/09/2026

O novo Go adulto de aparência mais realista está integrado com **20 ações, 60 quadros e os dez golpes**. O salto v2 substitui a anatomia menor encontrada na primeira revisão. A revisão principal aceitou a nova transição no runtime. Veja o [laudo final](../../../../production/go/finalization-review.md) e a [auditoria](../../../../production/go/finalization-audit.json).

- [Vídeo final a 60 fps](world/runtime-motion.mp4): 1.845 quadros, 30,750 s; [verificação de duração e pouso](world/motion-summary.json).
- [World final](world/capture-summary.json): 638 PNGs, 1.845 estados, dez golpes por jogador nos dois sentidos, dez retornos apoiados à guarda e HP final 86×86.
- [Salto completo](world/evidence-panels/jump-full-arc.png), [entrada/retorno do soco aéreo](world/evidence-panels/air_punch-transition.png) e [chute aéreo](world/evidence-panels/air_kick-transition.png), com a escala anatômica nova.
- [Dez golpes nos dois sentidos](world/evidence-panels/all-ten-moves-both-facings.png), [avanço](world/evidence-panels/walk_in.png), [recuo](world/evidence-panels/walk_out.png) e [volta à guarda](world/evidence-panels/walk-return-idle.png).
- [Lab final de jump](lab/capture-summary.json): dez PNGs das três chaves novas. O [Lab completo anterior](lab-before-jump-refinement/capture-summary.json) conserva 269 PNGs de 19 contextos; walk é coberto pelo World. A [comparação de pixels e metadados](jump-v2-frame-comparison.json) confirma os outros 57 quadros idênticos.
- [Studio final do salto v2](studio/jump-v2/README.md): três quadros novos em zoom 1.00x, pivôs e durações conferidos contra a UI. Os outros 19 clips permanecem documentados no [registro completo do Studio](studio/README.md).
- [Partida normal Go × Go](native-match-result.png), sem variável de opt-in, repetida após jump v2: **53×0**, com vitória e derrota apoiadas; [comando e hashes finais](native-match-assets.json).

O manifesto final tem SHA-256 `fc5560a0478019b1a45ef13cde21c34c177d12ff345ad373993913c5eab2e4da`; o atlas, `60f5e1b428e851146072fc7e8d5a672b479fb62bb6fbba91308519d4d8efd028`. O fechamento dos estados e a reexportação da produção mantiveram esses arquivos idênticos byte a byte.

Os painéis usam recortes dos pixels nativos, sem redimensionamento ou retoque. O [índice do World](world/evidence-index.json) registra cada imagem, tick e âncora. Os recortes acompanham o corpo físico; os PNGs completos e o vídeo preservam a translação no cenário. A captura é amostrada nas mudanças de chave/fase e a cada três ticks ativos; o encoder mantém o PNG anterior entre capturas, sem interpolação ou pausa final adicional. Para pausar e avançar um quadro: `ffplay runtime-motion.mp4`, Espaço e depois `s`.

O Lab usa poses forçadas para contextos estáticos e seu HUD sobrepõe parte da cabeça em poses altas; o World mostra a trajetória completa. Os ataques próximos não acertam na distância controlada do World; o Lab fornece alinhamento e a partida normal complementa a luta. As conclusões visuais se restringem às sequências inspecionadas.

A primeira passagem permanece em [World anterior](world-before-jump-refinement/capture-summary.json), [vídeo anterior](world-before-jump-refinement/runtime-motion.mp4) e [Lab anterior](lab-before-jump-refinement/capture-summary.json), com seus próprios hashes. O [atlas/manifesto e hashes por quadro anteriores](world-before-jump-refinement/runtime-assets/frame-checksums.json) permitem reproduzir a comparação. A partida anterior também permanece com sufixo `before-jump-refinement`; ela não substitui a recaptura final.

Os relatórios completos e todos os PNGs finais estão em `target/art/go-realistic-approved-world` e `target/art/go-realistic-approved-lab-jump`. Os 263 arquivos do Go cartoon continuam preservados em `assets/production/go-cartoon-archive` e `assets/candidates/go-cartoon-archive`; seis insumos de combate também foram reconferidos sem alteração.
