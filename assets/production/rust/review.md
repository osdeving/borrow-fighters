# Rust — revisão do piloto candidato

O piloto contém **20 clips e 71 desenhos selecionados**. As fontes foram geradas com a ferramenta de imagem real, uma ação por folha, mantendo o personagem original. Revisões rejeitadas, prompts, recortes explícitos, pivôs e durações permanecem em cada pasta de ação.

- [Atlas e manifesto](../../candidates/rust/rust-fighter.sprite.json)
- [Panorama](../../candidates/rust/review/overview.png), [caminhada](../../candidates/rust/review/walk.gif), [golpe](../../candidates/rust/review/punch_light.gif)
- [Vídeo do World nas duas orientações](../../candidates/rust/review/runtime-motion/runtime-motion.mp4) e [medidas do salto](../../candidates/rust/review/runtime-motion/motion-summary.json)
- [Plano de revisão](review-plan.json) e [produção explícita](production.json)

## Transparência e fontes

Duas tentativas iniciais de idle retornaram xadrez pintado em RGB. O usuário autorizou remoção local e revisão; o idle final remove o fundo e o halo neutro adjacente, preservando olhos e solas. Nas ações seguintes, a geração usa magenta sólido e a preparação remove/decontamina essa cor antes de exportar RGBA. O renderer faz mistura alpha normal; não há teste de magenta por pixel durante o desenho.

O ajuste opcional `--rust-contour` remove contaminação azul/vermelha de borda da paleta laranja do Rust. Ele não é aplicado aos demais personagens, cujas identidades incluem essas cores. Originais de referência e placeholders permanecem preservados.

## Evidência funcional

O Sprite Studio nativo carregou o manifesto real e os quatro idles foram percorridos a zoom 1. O guia desatualizado do editor foi corrigido para as métricas documentadas atuais; nenhuma física foi modificada. Compilação web e nativa do Studio passaram. Essa observação não é alegação de reprodução contínua no Studio.

O Combat Lab produziu 277 capturas GPU em 19 contextos, com manifesto visual carregado idêntico ao candidato daquela execução. Essa evidência histórica está em `target/art/rust-combat-lab-final` e antecede a última calibração de jump, que reduziu o atlas de 72 para 71 desenhos. Os nove golpes e as demais poses foram inspecionados, mas a captura usava as caixas de MoveSpec e `Projectile::from_fighter`, sem consultar os metadados de combate baseline. Portanto, ela continua útil para os seis golpes que usam MoveSpec e para a leitura das poses; não comprova as caixas próprias de punch_light, punch_heavy e kick, nem a origem do especial no World. A conferência do manifesto visual não verificava esse caminho de combate.

Esses quatro contextos foram recapturados após a correção: [jab, heavy e kick com caixas baseline](../../candidates/rust/review/runtime-baseline-attacks/README.md) têm contato visual com as caixas efetivas; o [especial com origem baseline](../../candidates/rust/review/runtime-baseline-special/README.md) mantém a trilha da engrenagem junto à mão. Os novos relatórios conferem tanto o manifesto visual quanto o de combate, encerrando essa pendência técnica.

O cenário World percorreu entrada/contagem, avanço e recuo, retorno ao idle e um salto completo, com Rust à direita e à esquerda simultaneamente. As 129 imagens e 585 estados ficam em `target/sprite-production/rust-motion-final`; o vídeo preserva o tempo de simulação. O primeiro frame aéreo ocorre no tick 495; ápice 518; última fase aérea 542; idle grounded 543. A calibração dos pivôs virtuais removeu a translação vertical extra entre subida e queda. O quarto desenho de pouso da fonte não é exportado: o jogo volta ao idle quando grounded.

Uma partida nativa Rust × Rust chegou ao resultado real, HP 9 × 0, com vencedor e derrotado visíveis no chão. As capturas `target/sprite-production/rust-final-actual-match-01.png` e `02.png` confirmam countdown e resultado. Essa observação motivou e confirmou as correções de espelhamento do atlas e apoio visual das poses de resultado. Nenhum dano, timing, alcance, corpo físico ou origem de projétil foi alterado.

## Limitações artísticas

- Sweep usa varredura ascendente: canela/tornozelo passam pela caixa existente, mas a sola fica em grande parte acima dela.
- AirKick cruza a borda da caixa com a parte inferior do pé; a leitura do contato pode melhorar.
- Throw alcança a caixa com antebraços, enquanto mãos avançam além do fim físico. Precisa de refinamento de alcance desenhado.
- A caminhada possui quatro poses selecionadas e ainda tem alternância limitada; variações discretas de rosto, mãos e volume entre ações seguem em revisão.
- A separação entre imagem candidata e metadados baseline preserva exatamente as caixas antigas do Rust, incluindo os cinco frames com hitboxes próprias. O projétil de engrenagem é original reaproveitado, separado do corpo.

O conjunto está integrado por `BORROW_FIGHTERS_SPRITE_CANDIDATES=1` e permanece candidato; a revisão técnica não equivale a aprovação artística final.
