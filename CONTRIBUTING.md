# Contributing

## Fase atual

Dia 0 / pré-produção.

O foco atual é documentação, escopo e validação da ideia.

## Antes de contribuir

Leia:

1. [`README.md`](README.md)
2. [`docs/00-vision.md`](docs/00-vision.md)
3. [`docs/01-mini-gdd.md`](docs/01-mini-gdd.md)
4. [`docs/05-governance.md`](docs/05-governance.md)
5. [`docs/07-art-direction.md`](docs/07-art-direction.md), se a contribuição envolver arte, mood ou personagem.
6. [`docs/08-code-architecture.md`](docs/08-code-architecture.md), se a contribuição envolver código.

## Contribuições úteis agora

- Abra issues pequenas.
- Sugira cortes de escopo.
- Proponha personagens com mecânica associada.
- Proponha moodboards ou direção visual com riscos claros.
- Para código, siga o esboço de arquitetura antes de criar módulos novos.
- Evite sugerir sistemas grandes antes do protótipo 0.1.
- Discuta decisões técnicas via ADR.

## Fluxo recomendado

1. Abra uma issue usando o template mais próximo.
2. Espere triagem quando a mudança afetar escopo, arte, processo ou stack.
3. Crie uma branch curta a partir de `main`.
4. Preencha o template de PR.
5. Atualize docs ou ADR se a mudança alterar direção do projeto.

## Modelo de branches

Seguimos **Trunk-Based Development adaptado**:

- `main` é o trunk e deve ficar protegida;
- todo trabalho entra por PR pequeno;
- branches devem durar pouco e voltar para `main` rapidamente;
- `release/*` existe apenas quando uma versão precisar de estabilização;
- não usamos GitFlow clássico com branches long-lived como `develop`.

Prefixos sugeridos:

| Prefixo | Uso |
|---|---|
| `docs/*` | documentação, processo, ADRs |
| `proposal/*` | exploração ainda aberta |
| `feature/*` | gameplay, ferramenta ou funcionalidade |
| `art/*` | arte, mood, referência ou asset |
| `release/*` | estabilização de versão |
| `hotfix/*` | correção urgente depois de release |

## Conventional Commits

Commits devem seguir **Conventional Commits**:

```text
tipo(escopo opcional): descrição curta no imperativo
```

Exemplos:

```text
docs(governance): define trunk-based development
docs(art): add mood proposal template
feat(gameplay): add basic attack state
fix(input): prevent jump buffering regression
chore(github): add issue templates
```

Tipos iniciais:

| Tipo | Uso |
|---|---|
| `docs` | documentação, GDD, ADRs, README |
| `feat` | funcionalidade nova |
| `fix` | correção de bug |
| `chore` | manutenção sem mudança de produto |
| `refactor` | mudança interna sem comportamento novo |
| `test` | testes |
| `build` | build, dependências, empacotamento |
| `ci` | automações e GitHub Actions |
| `art` | assets, moodboards e direção visual |
| `release` | preparação de versão |

Commits com quebra de compatibilidade ou direção devem usar `!` ou rodapé `BREAKING CHANGE:`.

## Regras de PR

- Um PR deve resolver uma intenção clara.
- PR grande deve ser quebrado antes de revisão.
- Mudança estrutural precisa de ADR ou issue de decisão.
- Arte deve indicar fonte, licença ou status de placeholder.
- O merge recomendado é squash merge.
- O título do PR deve poder virar um commit Conventional Commit no squash.

## Moldes úteis

- [`docs/templates/mood-proposal.md`](docs/templates/mood-proposal.md)
- [`docs/templates/character-concept.md`](docs/templates/character-concept.md)
- [`docs/templates/adr-template.md`](docs/templates/adr-template.md)
- [`docs/templates/release-checklist.md`](docs/templates/release-checklist.md)

## Padrão simples de issue

```md
## Ideia

## Por que isso ajuda o protótipo?

## Escopo mínimo

## Riscos
```

## O que evitar agora

- Sistemas grandes antes de validar o combate básico.
- Arte final antes de validar silhueta, legibilidade e mood.
- Discussões sem proposta testável.
- PR sem relação clara com visão, backlog, arte, ADR ou release.
