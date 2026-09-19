# 05 — Governança

O projeto usa branches curtas e PRs com intenção clara. `main` permanece
protegida; squash merge e títulos Conventional Commit são o padrão.
A configuração real de proteção vive no GitHub; este documento não a altera.

## Decisões e contribuição

- Pedido explícito do usuário autoriza a tarefa; não exigir issue/triagem extra
  para mudanças já acordadas. Propostas abertas podem começar em issue.
- ADR somente para decisão duradoura difícil de reverter. Experimentos de arte,
  ajustes locais e refactors reversíveis usam o PR ou diário existente.
- Atualizar docs quando comportamento, comandos ou contratos mudarem. Não criar
  um documento por entrega nem exigir varredura de todo o repositório.
- Goals longos mantêm um checkpoint curto; preservar trabalho após interrupções.
- Fontes de arte têm origem/licença. Revisão visual é necessária para animação.

## Verificação e entrega

O [workflow](09-ai-collaboration.md) separa iteração focada, verificação de
domínio e CI completa. Testes úteis ficam preservados. Matriz de features e
checagens globais pertencem à CI/merge; pacotes e distribuição à
[release](06-release-process.md). Não publicar nem fazer merge sem autorização.

PRs descrevem problema, resultado e validação. Referenciar issue quando existir;
não repetir histórico da conversa ou checklists sem relação com a mudança.
Regras e skills dos agentes seguem [AGENTS.md](../AGENTS.md).
