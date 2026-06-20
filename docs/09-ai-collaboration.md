# 09 — Colaboração com IA

## Status

Proposto.

Este projeto usa instruções leves e skills repo-local para ajudar Codex, Claude Code e outros agentes a trabalharem com menos contexto desperdiçado.

## Arquivos de instrução

| Arquivo | Ferramenta | Uso |
|---|---|---|
| `AGENTS.md` | Codex | Regras persistentes do repositório |
| `CLAUDE.md` | Claude Code | Memória/instruções persistentes do projeto |
| `.agents/skills/*/SKILL.md` | Codex | Skills repo-local com carregamento sob demanda |
| `.claude/skills/*/SKILL.md` | Claude Code | Skills de projeto invocáveis por `/skill-name` |

## Princípios

- Instruções globais devem ser curtas.
- Procedimentos repetíveis viram skills.
- Referências detalhadas ficam em `references/` dentro da skill.
- O agente deve ler o menor conjunto de docs que resolve a tarefa.
- Código novo deve ter descrição de módulo no topo.
- Decisões estruturais devem virar ADR.

## Skills iniciais

| Skill | Quando usar |
|---|---|
| `borrow-fighters-repo-atlas` | Antes de navegar o repo, escolher docs ou localizar área de mudança |
| `borrow-fighters-rust-gamedev` | Ao planejar ou implementar código Rust/Raylib |
| `borrow-fighters-gameplay-design` | Ao propor ou revisar mecânicas de luta |
| `borrow-fighters-art-direction` | Ao propor ou revisar mood, personagem, sprite ou asset |

## Como pedir trabalho para IA

Exemplos:

```text
Use $borrow-fighters-repo-atlas e me diga quais docs devo alterar para uma proposta de personagem.
```

```text
Use $borrow-fighters-rust-gamedev para planejar o scaffold do Prototype 0.1 sem criar código ainda.
```

```text
Use /borrow-fighters-gameplay-design para revisar esta ideia de golpe e apontar riscos de escopo.
```

## Rotas de contexto

- Produto e escopo: `docs/00-vision.md`, `docs/01-mini-gdd.md`, `docs/02-prototype-scope.md`.
- Governança: `docs/05-governance.md`, `CONTRIBUTING.md`.
- Código futuro: `docs/08-code-architecture.md`, ADRs em `docs/adr/`.
- Arte: `docs/07-art-direction.md`, `assets/references/`, `assets/placeholder/`.
- Release: `docs/06-release-process.md`, `CHANGELOG.md`.

## Fontes usadas para o formato

- [Codex — Custom instructions with AGENTS.md](https://developers.openai.com/codex/guides/agents-md): `AGENTS.md` para orientação persistente de repositório.
- [Codex — Agent Skills](https://developers.openai.com/codex/skills): `.agents/skills/` para skills repo-local e carregamento progressivo.
- [Codex — Customization](https://developers.openai.com/codex/concepts/customization): `AGENTS.md`, skills, memórias, MCP e subagents como camadas complementares.
- [Claude Code — Skills](https://docs.anthropic.com/en/docs/claude-code/skills): `.claude/skills/*/SKILL.md` para skills de projeto.
- [Claude Code SDK overview](https://docs.anthropic.com/en/docs/claude-code/sdk): `CLAUDE.md` e `.claude/skills/` como configuração filesystem-based.
