# ADR 0014 — Arremessos, lançamentos e efeitos de assinatura

## Status

Aceito para a rodada de identidade de combate solicitada pelo usuário.

## Contexto

O agarrão anterior aplicava dano, pushback e queda no mesmo lado. Anti-air usava a reação comum sem impulso de lançamento. Os especiais eram golpes próximos com pouca diferença visual. O novo pedido exige trajetória, interação dos dois atores e efeitos ofensivos que persistem além da pose do personagem.

## Decisão

`World` mantém uma sequência explícita de arremesso com captor, vítima, fase, tempo e destino seguro. A captura trava entradas e conecta os atores; a soltura inicia velocidade balística e a recuperação só começa ao tocar o chão. O destino cruza o atacante no centro da arena e se adapta aos limites sem teleporte. `Fighter` expõe estado de captura e reação aérea; `HitReactionKind` seleciona reação leve, pesada, lançada, arremessada ou caída. KO aguarda a resolução da queda.

Os especiais continuam ligados a `MoveSpec` e ao input existente de assinatura, sem medidor. Efeitos ofensivos temporários tipados, com dono, posição, fase, relógio e volumes de colisão, ficam sob `World`. Não se introduz ECS, sistema de plugins ou scripts. Renderização consome esse estado sem aplicar dano ou decidir o alvo. Projéteis, barreiras, erupções e vórtices têm apresentação própria e respostas defensivas explícitas.

Cada personagem possui um atlas de efeito no schema de sprite existente, com clip `projectile` de dois quadros e `impact` de quatro. Atlas de luta mantém a ação de oito poses e as reações. O carregador opcional mantém o jogo diagnosticável se um asset faltar, mas a validação da entrega rejeita fallback nos cinco conjuntos produzidos.

O showcase continua usando apenas entradas e preparação do `World`; atualiza seus cenários, distâncias e resultados para lançamentos e múltiplos contatos reais. A verificação cobre cantos, lados, recuperação, KO aéreo, bloqueio, interrupção e frames visualmente adequados.

## Consequências

- A física e as fases ficam verificáveis sem Raylib; renderer e animação acompanham os mesmos estados.
- Há mais sprites e efeitos, exigidos explicitamente por esta rodada; a produção reaproveita o schema e os helpers existentes.
- Reações aéreas protegidas adiam sistemas de juggle e combos complexos.
- As ações de assinatura anteriores deixam de ser a apresentação final, mas fontes e chaves de entrada antigas podem ser preservadas como histórico/aliases.

Ver [direção e critérios](../21-signature-spectacle-and-throws.md).
