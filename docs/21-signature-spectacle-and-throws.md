# 21 — Arremessos, reações e especiais de assinatura

## Pedido e critério de entrega

Rodada solicitada após o MVP `88ffcaf`: o agarrão deve arremessar de verdade e trocar os lados, os impactos devem produzir reações apropriadas e os cinco especiais precisam representar as linguagens com humor e impacto visual. Rust, Duke/Java, C, Python e C++ são o elenco desta produção. Os especiais permanecem livres de medidor; disponibilidade não elimina antecipação e recuperação.

## Direção aprovada para implementação

| Personagem | Especial | Ação e leitura |
|---|---|---|
| Rust | Borrow Fortress | Constrói uma fortaleza/escudo hexagonal laranja, bloqueia pressão frontal por uma janela curta e investe com a barreira. |
| Duke/Java | System.out.println! | Invoca uma impressora/funil de papel absurdo e dispara três pulsos de recibos e código; termina com uma pane cômica. |
| C | Segmentation Fault | Carrega o livro e bate no chão, abrindo uma ruptura de memória que explode em blocos vermelhos/cyan e lança a vítima. |
| C++ | Undefined Bazooka | Retira uma bazuca enorme da bolsa, mira os pés, dispara um foguete e sofre recoil; explosão exagerada, sem ferimento gráfico. |
| Python | import antigravity | Levita com expressão surpresa enquanto a cobra azul/amarela forma um grande vórtice ascendente. |

Cada especial possui oito poses principais do personagem e um atlas de efeitos separado com seis quadros. Efeitos e corpo são sprites produzidos por `imagegen`; o texto exato, partículas secundárias e apresentação usam o renderer. A identidade, figurino e acessórios existentes permanecem reconhecíveis. As fontes anteriores estão preservadas.

## Arremessos e reações

O agarrão pune a guarda no chão. Depois do contato, há captura de 12 frames, levantamento visível e lançamento balístico de 40 frames por cima do atacante, com pouso no lado oposto quando há espaço. Perto do limite, o destino seguro em direção ao centro tem prioridade sobre a inversão. Não há teleporte para simular o lançamento. KO aéreo aguarda aterrissagem.

As reações separam impacto leve, impacto pesado, lançamento por gancho/anti-air, vítima arremessada e recuperação no chão. Novos clips `heavy_hit`, `launched` e `thrown` complementam `hit` e `knockdown`; o relógio e a física determinam a fase visual. Lançamentos têm proteção contra novos hits até o pouso nesta rodada, evitando juggle infinito.

## Controles e showcase

```sh
cargo run --bin borrow-fighters -- --showcase --character rust --move signature_special --repeat
```

Troque `rust` por `duke`, `c`, `python` ou `cpp`. `Tab`/`Shift+Tab` mudam a situação, `PgUp`/`PgDown` mudam o personagem, `X` inverte os lados, `Enter` repete, `Space` pausa e `.` avança um frame. Use `--move throw` ou `--move anti_air` para inspecionar arremessos e ganchos diretamente.

Na luta, `T` (P1), `\` (P2) ou `RT` acionam o especial, sempre sem medidor. Agarrão usa guarda + soco fraco (`Q+F` / `U+O`). Antecipação, distância e recuperação continuam valendo. A barreira de Rust protege por uma janela frontal curta; agarrão e ataque pelas costas a vencem. C e C++ exigem defesa baixa. Java dispara três contatos, de 8 HP cada; os demais especiais acertam uma vez.

Cada situação dura 260 frames e executa um `World` real. A distância de Rust e Python deixa a barreira/vórtice visível por 7–12 frames antes do contato, incluindo os dois quadros de formação. Java soma os três acertos no painel. Captura, lançamento, queda e recuperação cabem no ciclo; o painel inferior deixa livre o espaço aéreo.

## Arte e coerência

Foram produzidos **156 desenhos novos**: 126 poses de ator e 30 efeitos. Os cinco selecionáveis possuem 25 clips cada; Go conserva seus 20 clips e a arte anterior. O total dos seis atlas é 145 clips / 510 quadros de ator, mais os cinco atlas de efeitos.

As fontes, prompts, versões de matte, recortes explícitos e pivôs permanecem em `assets/production/`. O render usa pivô de chão para efeitos terrestres e pivô corporal para voo. Na aproximação do piso, a vítima converge visualmente para o chão; a queda começa no quadro deitado e o KO mantém essa pose. O flash de impacto dura quatro frames, deixando as novas reações legíveis. Os efeitos continuam sua animação após o contato, sem causar dano adicional.

Laudos e evidências: [Rust](../assets/production/rust/spectacle-review.md), [Duke](../assets/production/duke/spectacle-review.md), [C](../assets/production/c/spectacle-review.md), [Python](../assets/production/python/extraordinary-review.md) e [C++](../assets/production/cpp/extraordinary-review.md). A [ADR 0014](adr/0014-throws-launches-and-signature-effects.md) registra as decisões que atravessam combate e renderização.

## Verificação

A suíte cobre todos os golpes dos cinco em ambas as direções, com metadata de luta e com fallback, além de guarda alta/baixa, cantos, entrada durante captura, voo, aterrissagem, KO e fim dos efeitos. A matriz contextual soma 220 cenários de ataque e 40 de defesa. Os testes de sprites verificam completude, fases e duração por tick.

O [ensaio do App real](evidence/signature-runtime-controls/README.md) aprovou 25 verificações de teclado, navegação do showcase, arremessos, especiais, debug em Options e fechamento nativo da janela. A [auditoria de balanceamento](evidence/signature-balance/README.md) concluiu 60 lutas de CPU, média de 19,30 segundos, máximo de 30,92; serve como regressão do MVP, sem pretender medir equilíbrio competitivo.

A revisão gráfica cobriu **40 cenas dirigidas**: cinco personagens × especial/arremesso/gancho/soco forte × duas orientações. Os outros onze cenários de Rust também foram capturados na orientação invertida. Laudos registram os quadros reais selecionados, fontes e hashes. A checagem de alpha e retângulos aprovou todos os 510 quadros de ator e 30 efeitos.

**[Checks finais aprovados](evidence/signature-validation/README.md):** `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets` (**290 testes**) e sete testes Python do exportador. A revisão de simultaneidade corrigiu prioridade de P1 e direção da proteção de Rust; espelhos Python/Python e Rust/Rust agora têm regressões nos dois lados, com metadata e fallback. Uma captura adicional confirmou a vítima inteira no canto e o pouso sem voltar à pose em pé.

[Vídeo da entrega](../assets/showcase/signature-spectacle-2026-09-08.mp4): 21 segundos, 1280×720 a 60 fps, sem áudio. São sete trechos da simulação real, sem interpolação de poses: os cinco especiais, arremesso e gancho. A [metadata](../assets/showcase/signature-spectacle-2026-09-08.json) registra os frames usados.
