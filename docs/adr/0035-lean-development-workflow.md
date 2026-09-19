# ADR 0035 — Workflow proporcional à mudança

Status: aceito em 19/09/2026, por pedido do usuário.

## Decisão

O desenvolvimento normal usa leitura mínima e checagens focadas. Entrega de
código verifica o domínio afetado uma vez; CI/merge conserva a matriz completa
e release acrescenta distribuição. Não apagar testes úteis para reduzir custo.
ADRs ficam para decisões duradouras difíceis de reverter; tarefas comuns não
precisam de novos documentos. Goals longos usam um único checkpoint curto.

`./dev` reúne comandos existentes. Sprite Studio substitui o antigo Viewer
Raylib (paridade registrada no doc 18), mantendo Combat Lab e Actor Lab.
O contrato Rust de sprites gera schema/tipos do Studio. Um builder Python
existente, consolidado, usa receitas: fonte → candidato gerado → asset aceito.
Diretórios legados são migrados apenas quando necessário, sem quebrar gameplay.

## Consequências

Menos contexto e ferramentas duplicadas; verificações amplas continuam na CI.
Mudanças de arte exigem revisão em movimento, mas não uma captura integral
por iteração. O experimento Blender permanece na sua branch; a retomada 2D
é reversível e será registrada no diário, sem outra decisão arquitetural.

[Workflow](../09-ai-collaboration.md) · [Retomada](../worklogs/lean-workflow-augusta-sprites.md).
