# Rust — evidência do refinamento

O atlas Rust mantém 20 clips/71 quadros. Esta rodada altera walk, sweep, throw e air_kick com fontes geradas novas; recalibra somente o pivô ativo de air_punch; limpa o alpha da engrenagem reaproveitada. Fontes e revisões anteriores permanecem em `assets/production/rust/`.

- [Vídeo World: movimento e dez golpes nos dois sentidos](world-motion-and-attacks.mp4).
- [Relatório World completo](world-report.json), [resumo de tempo](world-motion-summary.json) e [índice dos painéis](world-evidence-index.json).
- [Caminhada](evidence-panels/walk_in.png), [soco aéreo e transições](evidence-panels/air_punch-transition.png), [chute aéreo e transições](evidence-panels/air_kick-transition.png).
- Contato no Combat Lab: [sweep](sweep-f012-active.png), [throw](throw-f006-active.png), [air_punch](air_punch-f006-active.png), [air_kick](air_kick-f012-active.png). [Relatório dos 19 contextos](lab-report.json).
- [Especial com projétil limpo](special-clean-projectile.png) e [recaptura específica](projectile-report.json).
- Sprite Studio nativo: [24 quadros em zoom 1.00x](studio/ui-frame-crops.png), [registro da navegação](studio/review-captures.json) e [manifesto mostrado na UI](studio/loaded-ui-manifest.json). Inspeção sequencial, sem alegar playback contínuo nessa ferramenta.
- [Resultado de partida nativa entre CPUs](native-match-result.png): Rust × Rust, 9 × 0, poses finais apoiadas no chão.

O World registrou 650 imagens e 1.845 estados; os dez golpes foram observados com Right e Left, seguidos de retorno ao idle grounded. O vídeo tem 1.845 quadros, 60 fps e 30,75 s, conferidos por ffprobe. As imagens são amostradas nas mudanças de pose/fase e a cada três ticks durante a luta: o vídeo mantém a última captura até a próxima, sem interpolar ou criar arte. Isso não equivale a capturar todos os ticks.

O Lab gerou 276 capturas com o manifesto visual e baseline corretos. A engrenagem recebeu limpeza depois dessa captura e do World; foi recapturada separadamente em 17 imagens, com carregamento do PNG candidato de 237 × 116 confirmado no log. A partida nativa também usa a engrenagem limpa. As caixas, a origem e as regras permaneceram iguais.

A pose de ataque aéreo estende o tronco em relação ao salto encolhido; essa mudança de silhueta permanece. O degrau interno de aproximadamente 22 px entre preparação/contato do soco aéreo foi corrigido. O espelhamento convencional também inverte o `R`, como no renderer anterior; esta rodada não introduz uma folha esquerda independente.

As capturas completas originais ficam localmente em `target/art/rust-finalization-world`, `target/art/rust-finalization-lab`, `target/art/rust-finalization-projectile` e `target/art/rust-finalization-match`. Os painéis apenas recortam pixels das capturas para inspeção; vídeo e PNGs nativos mostram a posição real na arena.
