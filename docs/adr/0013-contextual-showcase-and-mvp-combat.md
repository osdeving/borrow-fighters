# ADR 0013 — Showcase contextual e combate de MVP

## Status

Aceito para a rodada de MVP solicitada pelo usuário, limitada a Rust, Duke/Java, C, Python e C++.

## Contexto

O showcase anterior reproduzia a pose de um lutador sozinho. Ele não demonstrava contato, defesa, agarrão nem a aproximação aérea necessária para um anti-air. A revisão também identificou rasteiras com postura e colisão altas, perda da defesa baixa durante blockstun e ausência de reação de queda.

## Decisão

`MoveShowcase` controla entradas e posicionamento inicial de dois lutadores em um `World` comum. Cada cenário descreve uma situação de combate e fornece entradas determinísticas. Movimento, colisão, dano, defesa, áudio e reações seguem o mesmo caminho da luta. O resultado exibido vem do log de contatos; cenários nunca aplicam dano para simular sucesso. Reset entre apresentações restaura os dois atores.

Os cinco especiais entram como `MoveSpec` e `MoveInputKind::SignatureSpecial`, com um botão simples e um golpe por loadout. Projétil conserva seu input e clip `special`; a nova ação usa `signature_special`. Não há medidor ou subsistema de especiais.

Rasteira e slide preservam postura baixa até terminar. Seus volumes ofensivos, os dos novos especiais e as hurtboxes das ações baixas usam a geometria autoritativa do combate. Isso evita que metadata baseline de poses anteriores reintroduza uma rasteira na altura do peito. Os demais golpes conservam a prioridade de metadata descrita na ADR 0007. O debug deve desenhar os mesmos volumes usados pelo `World`.

Rasteiras, agarrões e slide bem-sucedidos no chão iniciam queda com recuperação protegida de 36 frames. `HitReactionKind` seleciona `hit` ou `knockdown`; o relógio visual começa no impacto e não controla a duração física. O período no chão não aceita novos hits. Agarrões exigem alvo no chão, fora de stun e fora dos seis frames de proteção ao levantar.

## Consequências

- Cenários e respostas defensivas são verificáveis sem abrir Raylib; capturas gráficas verificam sua leitura visual.
- Os cinco atlas desta rodada precisam de `signature_special` e `knockdown`, além dos 20 clips anteriores. Go conserva sua cobertura anterior e usa fallback de reação.
- Defesa baixa permanece baixa durante blockstun, chip não encerra a luta e conjuração não pode se sobrepor a outro ataque.
- Os resets são próprios de treino; o combate não recebe regras especiais para favorecer uma demonstração.
- A arte nova segue a produção revisada da ADR 0010, com fontes, prompts, recortes e pivots explícitos.

Ver [escopo, validação e balanceamento](../20-mvp-combat-showcase.md) e [guia técnico](../12-technical-combat-guide.md).
