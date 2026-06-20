# 05 — Governança do Projeto

## Status

Proposto para o dia 0.

Este documento define como o projeto toma decisões, recebe contribuição e usa o GitHub para manter o trabalho pequeno, revisável e rastreável.

## Princípios

1. **Docs-first**: ideias relevantes começam em issue, discussão, documento ou ADR antes de virarem implementação grande.
2. **Escopo pequeno**: PR bom resolve uma coisa clara.
3. **Main protegida**: a branch principal deve representar o estado mais confiável do projeto.
4. **Decisões rastreáveis**: decisões técnicas, criativas ou de produção que mudam direção devem ter registro.
5. **Contribuição aberta, curadoria explícita**: qualquer pessoa pode propor; aceitar é decisão do projeto.

## Papéis iniciais

Papéis não são cargos fixos. Uma pessoa pode ocupar mais de um papel enquanto o projeto for pequeno.

| Papel | Responsabilidade |
|---|---|
| Core Steward | Guardar visão, escopo, prioridades e coerência geral |
| Gameplay | Propor e validar mecânicas de luta, protótipo e sensação de combate |
| Art Direction | Cuidar de mood, linguagem visual, legibilidade e identidade dos personagens |
| Production / Release | Organizar milestones, releases, checklist e cortes de escopo |
| Docs / ADR | Manter documentação, índice, templates e decisões rastreáveis |
| Community / QA | Triar feedback, issues, testes manuais e playtest notes |

## Squads sugeridas

No dia 0, as squads são frentes de trabalho, não organograma.

| Squad | Foco |
|---|---|
| Core | Visão, backlog, governança, escopo e priorização |
| Prototype | Mecânica central do protótipo 0.1 |
| Art & Mood | Direção de arte, moodboards, placeholders e identidade visual |
| Docs & Process | README, CONTRIBUTING, ADRs, releases, templates e GitHub hygiene |

## Estratégia de branches

O projeto segue **Trunk-Based Development adaptado**, não GitFlow clássico.

`main` é o trunk: a branch principal, protegida e sempre próxima de um estado publicável. Todo trabalho deve sair de `main` e voltar por PR pequeno. Branches long-lived devem ser evitadas.

| Branch | Regra |
|---|---|
| `main` | Protegida. Recebe somente PR aprovado |
| `docs/*` | Documentação, processos, ADRs e briefing |
| `proposal/*` | Explorações ainda abertas, normalmente sem código de produção |
| `feature/*` | Funcionalidade de jogo ou ferramenta |
| `art/*` | Arte, moodboards, placeholders e assets |
| `release/*` | Estabilização de uma versão específica |
| `hotfix/*` | Correção urgente depois de release |

Não usar branch `develop` por padrão. `release/*` só deve existir quando uma versão precisar de estabilização separada. No dia 0, tags em `main` provavelmente bastam.

## Conventional Commits

Commits e títulos de PR devem seguir **Conventional Commits** para manter histórico legível e facilitar changelog/release notes.

Formato:

```text
tipo(escopo opcional): descrição curta no imperativo
```

Tipos aceitos inicialmente:

| Tipo | Uso |
|---|---|
| `docs` | documentação, GDD, ADRs, README |
| `feat` | funcionalidade nova |
| `fix` | correção de bug |
| `chore` | manutenção sem mudança de produto |
| `refactor` | mudança interna sem comportamento novo |
| `test` | testes |
| `build` | build, dependências e empacotamento |
| `ci` | automações e GitHub Actions |
| `art` | assets, moodboards e direção visual |
| `release` | preparação de versão |

Exemplos:

```text
docs(governance): define trunk-based development
art(mood): propose terminal arcade direction
feat(gameplay): add basic attack state
fix(input): prevent jump buffering regression
release: prepare v0.0.1-docs
```

Mudanças quebráveis devem usar `!` no tipo/escopo ou rodapé `BREAKING CHANGE:`.

## Regras de PR

Todo PR deve:

- usar o template de PR;
- ter objetivo claro;
- apontar issue, discussão ou documento relacionado quando existir;
- explicar impacto em escopo, arte, docs ou release;
- ser pequeno o bastante para revisão humana;
- atualizar documentação quando mudar processo, visão, contrato criativo ou decisão;
- criar ou atualizar ADR quando mudar uma decisão estrutural.

Merge padrão recomendado: **squash merge**.

O título do PR deve ser escrito como Conventional Commit, porque no squash ele provavelmente vira o commit final em `main`.

## Proteção de branch no GitHub

Essas regras precisam ser configuradas em **Settings > Rules > Rulesets** ou **Settings > Branches**. Arquivos no repositório documentam a política, mas não bloqueiam branches sozinhos.

### Ruleset `protect-main`

Alvo: `main`.

Regras recomendadas:

- bloquear force push;
- bloquear deleção da branch;
- exigir pull request antes de merge;
- exigir pelo menos 1 aprovação no dia 0;
- aumentar para 2 aprovações quando houver código jogável ou assets finais;
- exigir resolução de conversas antes do merge;
- descartar aprovações antigas quando novos commits entrarem;
- exigir revisão de CODEOWNERS quando os times reais existirem;
- exigir status checks `Validate docs and GitHub YAML` e `Conventional Commit title`;
- exigir histórico linear se o projeto mantiver squash/rebase como padrão.
- exigir nome de PR compatível com Conventional Commits quando houver automação para isso.

Durante o dia 0, admins podem ter bypass das proteções para evitar bloqueio operacional enquanto ainda não existe equipe de revisão. Quando houver pelo menos duas pessoas ativas no projeto, reavaliar se `enforce_admins` deve ser ligado.

### Ruleset `protect-release`

Alvo: `release/*`.

Regras recomendadas:

- bloquear force push;
- bloquear deleção;
- exigir PR para alterações;
- exigir aprovação do papel Production / Release ou Core Steward;
- exigir checklist de release preenchido.

Durante o dia 0, admins podem ter bypass desse ruleset pelo mesmo motivo operacional.

## CODEOWNERS

O arquivo `.github/CODEOWNERS` está como molde. Antes de ativar "Require review from Code Owners", substituir os placeholders por usuários ou times reais do GitHub.

## Taxonomia de labels

Labels sugeridas:

| Prefixo | Exemplos |
|---|---|
| `type:` | `type: proposal`, `type: bug`, `type: docs`, `type: adr`, `type: release` |
| `area:` | `area: gameplay`, `area: art`, `area: docs`, `area: process`, `area: infra` |
| `status:` | `status: triage`, `status: accepted`, `status: blocked`, `status: parked` |
| `size:` | `size: xs`, `size: s`, `size: m`, `size: l`, `size: xl` |
| `release:` | `release: 0.1`, `release: later` |

## Fluxo de issue

1. **Triage**: entender se a ideia ajuda o momento atual.
2. **Accepted**: aceita para backlog, milestone ou documento.
3. **Parked**: boa ideia, momento errado.
4. **Blocked**: depende de decisão, pessoa, pesquisa ou protótipo.
5. **Closed**: resolvida, duplicada, fora de escopo ou substituída.

## Quando usar ADR

Usar ADR quando a decisão:

- muda stack, arquitetura, ferramenta ou plataforma;
- cria padrão que outros contribuidores terão que seguir;
- afeta pipeline de release;
- congela direção criativa importante;
- remove alternativa relevante.

Usar issue ou doc comum quando a decisão for pequena, reversível ou exploratória.

## Questões em aberto

- Nome final do projeto.
- Organização GitHub e nomes reais dos times.
- Nível exato de exigência para revisão de arte.
- Quando transformar mood aprovado em guia visual formal.
- Quando introduzir CI obrigatório.
- Quando automatizar validação de Conventional Commits.
