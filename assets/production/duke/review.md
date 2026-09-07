# Duke — revisão do candidato completo

20 ações próprias e 61 desenhos selecionados em 24 chamadas reais do imagegen integrado. Fontes e prompts, incluindo revisões rejeitadas, permanecem em cada diretório de ação. O master original e o novo idle foram referências em todas as chamadas seguintes. O fundo magenta foi removido pelo helper comum, sem tratamento de contorno específico de Rust.

O atlas foi preparado e exportado pelos três scripts compartilhados. Os 61 recortes têm limites e pivots válidos, alpha real e margem transparente nas quatro bordas. A altura do primeiro idle foi normalizada para 251 px. Os nove relógios de golpes coincidem com os limites do contrato de combate; `cargo test --test sprite_candidates` passou os dois testes. Nenhuma metadata de colisão ou origem foi substituída.

A execução histórica de `capture_sprite_review` produziu 267 PNGs, 19 contextos e 57 desenhos, com manifesto visual carregado idêntico ao candidato daquela execução. A captura usa a orientação nativa Right. Os nove golpes usam MoveSpec, portanto a inspeção de seus contatos continua válida. Porém, o Lab antigo criava o especial por `Projectile::from_fighter`, sem consultar a origem baseline; essa captura não comprova a saída do projétil no World.

A [recaptura corrigida do especial](../../candidates/duke/review/runtime-baseline-special/README.md), com 16 amostras e ambos os manifestos conferidos, confirma a origem baseline e os grãos junto à boca da arma. Essa pendência técnica foi encerrada. A revisão técnica não significa aprovação artística final.

Uma captura World posterior verificou entrada/contagem, avanço/recuo, retorno ao idle e salto completo nas duas orientações: 125 imagens e 585 ticks, com retorno ao chão no tick 543. [Vídeo e medidas](../../candidates/duke/review/runtime-motion/README.md) registram o movimento real com inputs determinísticos. Esse cenário não exercita o especial nem transições de resultado de combate.

| Ação | Captura histórica do Lab | Leitura |
| --- | --- | --- |
| idle | [idle-f000-idle.png](../../../target/art/duke-combat-lab-review/idle-f000-idle.png) | Nariz, emblema, vapor e apoio preservados; corpo sólido perto da caixa em pé. |
| crouch | [crouch-f012-idle.png](../../../target/art/duke-combat-lab-review/crouch-f012-idle.png) | Agachamento baixo: cabeça sólida próxima ao teto da caixa; vapor fica acima. |
| jump | [jump-f045-idle.png](../../../target/art/duke-combat-lab-review/jump-f045-idle.png) | Último desenho mantém pernas estendidas no ar; o Lab fixa a altura, sem trajetória completa. |
| block | [block-f012-idle.png](../../../target/art/duke-combat-lab-review/block-f012-idle.png) | Palma à frente coincide com a área de defesa; apoio no chão. |
| crouch_block | [crouch_block-f012-idle.png](../../../target/art/duke-combat-lab-review/crouch_block-f012-idle.png) | Defesa baixa legível, mas cone da cabeça excede a caixa de 128 px em cerca de 37 px. |
| hit | [hit-f006-idle.png](../../../target/art/duke-combat-lab-review/hit-f006-idle.png) | Recuo legível com um pé de apoio; sem nova lesão ou rosto humano. |
| punch_light | [punch_light-f006-active.png](../../../target/art/duke-combat-lab-review/punch_light-f006-active.png) | Luva ativa dentro da caixa e alcance curto preservado. |
| punch_heavy | [punch_heavy-f018-active.png](../../../target/art/duke-combat-lab-review/punch_heavy-f018-active.png) | Luva ativa dentro da caixa; braço fino longo diferencia o golpe forte. |
| kick | [kick-f012-active.png](../../../target/art/duke-combat-lab-review/kick-f012-active.png) | Sapato ativo dentro da caixa; apoio permanece no chão. |
| sweep | [sweep-f018-active.png](../../../target/art/duke-combat-lab-review/sweep-f018-active.png) | Sola cruza a caixa e mão apoia o corpo; extremidades do sapato ultrapassam ligeiramente a altura da caixa. |
| overhead | [overhead-f018-active.png](../../../target/art/duke-combat-lab-review/overhead-f018-active.png) | Luva dentro da caixa; preparação alta e contato descendente curto, próximo de um gancho horizontal. |
| anti_air | [anti_air-f012-active.png](../../../target/art/duke-combat-lab-review/anti_air-f012-active.png) | Punho elevado dentro da caixa; mão inteira preservada, pés no chão. |
| air_punch | [air_punch-f006-active.png](../../../target/art/duke-combat-lab-review/air_punch-f006-active.png) | Luva dentro da caixa aérea; pernas recolhidas e núcleo do corpo estável. |
| air_kick | [air_kick-f012-active.png](../../../target/art/duke-combat-lab-review/air_kick-f012-active.png) | Sola inteira dentro da caixa aérea; perna aponta para frente e para baixo. |
| throw | [throw-f009-active.png](../../../target/art/duke-combat-lab-review/throw-f009-active.png) | Dedos dentro do alcance existente; duas mãos de agarrão distintas, sem vítima embutida. |
| special | [special-f001-idle.png](../../../target/art/duke-combat-lab-review/special-f001-idle.png) | Projétil original separado visível; a origem baseline foi confirmada na recaptura corrigida vinculada acima, não nesta imagem histórica. |
| spawn | [spawn-f090-idle.png](../../../target/art/duke-combat-lab-review/spawn-f090-idle.png) | Gag original do café/mesa preservada. Mesa e fragmentos avançam alguns pixels abaixo da linha dos pés como objetos em primeiro plano. |
| victory | [victory-f030-idle.png](../../../target/art/duke-combat-lab-review/victory-f030-idle.png) | Brinde com caneca inteira, vapor e pés apoiados. |
| defeat | [defeat-f045-idle.png](../../../target/art/duke-combat-lab-review/defeat-f045-idle.png) | Mascote sentado e abatido, mãos de apoio no chão; sem novo estado de combate. |

O ciclo de caminhada tem contatos primeiro/terceiro com silhuetas semelhantes; a alternância continua limitada mesmo com o deslocamento conferido no World. A introdução tem primeiro desenho mais ereto que a guarda final, conservando uma escala uniforme por folha. O salto usa 60/340/400 ms e não traz agachamento de pouso em pleno ar. A colisão de Sweep continua usando o corpo em pé durante o ataque, comportamento anterior do jogo.

SHA-256 do atlas na captura histórica do Lab: `dd926b5d45f084f081e3e435bd8d8b0795b795feb1937e22c18adfb5990a3179`. Relatório bruto: [capture-report.json](../../../target/art/duke-combat-lab-review/capture-report.json).
