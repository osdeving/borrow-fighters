# 06 — Processo de Release

## Status

Proposto para o dia 0.

O objetivo é criar um processo simples o bastante para não travar o protótipo, mas forte o bastante para manter histórico, versão e escopo sob controle.

## Tipos de release

| Tipo | Exemplo | Objetivo |
|---|---|---|
| Docs foundation | `v0.0.1-docs` | Marcar base de documentação e governança |
| Prototype | `v0.1.0-prototype.1` | Validar mecânica central jogável |
| Vertical slice | `v0.2.0-slice.1` | Demonstrar identidade mínima de gameplay e arte |
| Demo | `v0.3.0-demo.1` | Empacotar experiência compartilhável |

Enquanto o jogo estiver em pré-produção, versões podem quebrar formato, escopo e estrutura. O compromisso é registrar a mudança.

## Fonte de verdade

- **Milestones do GitHub** agrupam escopo.
- **Issues** descrevem trabalho e discussão.
- **PRs** carregam revisão e histórico.
- **Conventional Commits** tornam o histórico legível e ajudam a gerar notas de release.
- **Tags** marcam versões.
- **GitHub Releases** comunicam o que mudou.
- **CHANGELOG.md** resume marcos importantes.

## Convenção de versão

Usar tags no formato:

```text
vMAJOR.MINOR.PATCH-sufixo.N
```

Exemplos:

- `v0.0.1-docs`
- `v0.1.0-prototype.1`
- `v0.1.0-prototype.2`
- `v0.2.0-slice.1`

## Fluxo de release

1. Criar ou revisar milestone no GitHub.
2. Definir objetivo da versão em uma frase.
3. Cortar escopo explicitamente.
4. Fechar ou mover issues que não entram.
5. Criar branch `release/vX.Y.Z` apenas quando houver estabilização real.
6. Abrir PR de release para atualizar docs, checklist e changelog.
7. Revisar com Core Steward e Production / Release.
8. Criar tag.
9. Publicar GitHub Release usando as notas geradas.
10. Registrar aprendizados e próximos riscos.

## Commits e changelog

O projeto usa Conventional Commits para facilitar leitura de histórico, squash merge e release notes.

Mapeamento inicial:

| Commit | Release notes |
|---|---|
| `feat` | Novidades |
| `fix` | Correções |
| `docs` | Documentação |
| `art` | Arte e direção visual |
| `ci` | Automação |
| `build` | Build e empacotamento |
| `release` | Preparação de versão |
| `BREAKING CHANGE` ou `!` | Mudanças quebráveis |

No dia 0, a validação é manual durante revisão de PR. Automação pode entrar quando houver CI.

Automação inicial:

- `Validate docs and GitHub YAML`: valida YAML em `.github/` e links Markdown locais.
- `Conventional Commit title`: valida título de PR antes do merge.

## Critério de pronto para release

Para qualquer release:

- objetivo da versão descrito;
- milestone revisada;
- changelog atualizado;
- issues fora de escopo movidas;
- riscos conhecidos documentados;
- PR de release aprovado.

Para release jogável:

- build reproduzível no ambiente principal;
- instruções de execução atualizadas;
- teste manual mínimo registrado;
- assets licenciados ou marcados como placeholder;
- bugs bloqueantes triados.

## Release branches

Como seguimos Trunk-Based Development adaptado, criar `release/*` somente quando o projeto precisar estabilizar algo antes de publicar. No dia 0, provavelmente tags em `main` bastam.

Regras:

- não adicionar feature nova em `release/*`;
- aceitar apenas docs de release, correções e cortes de escopo;
- merge de volta para `main` via PR;
- deletar branch depois da release, se não houver manutenção ativa.

## GitHub Releases

O arquivo `.github/release.yml` organiza categorias de changelog geradas pelo GitHub.

Labels importantes para release notes:

- `area: gameplay`
- `area: art`
- `area: docs`
- `area: process`
- `type: bug`
- `type: release`
- `breaking`

## Primeira release sugerida

Primeiro marco recomendado:

```text
v0.0.1-docs
```

Objetivo: congelar uma base inicial de visão, governança, contribuição, templates e processo de release antes de iniciar código de produção.
