# ADR 0002 — Fluxo de versionamento: Trunk-Based Development e Conventional Commits

## Status

Proposto.

## Contexto

O projeto está no dia 0 e precisa receber contribuições de documentação, arte, gameplay e processo sem criar burocracia pesada.

Ao mesmo tempo, o histórico precisa ser legível desde o começo. Branches long-lived, commits sem padrão e merges grandes tendem a dificultar revisão, release notes e rastreio de decisões.

## Decisão

Adotar:

- **Trunk-Based Development adaptado**;
- `main` como trunk protegido;
- PRs pequenos e revisáveis;
- branches curtas por intenção;
- squash merge como merge padrão;
- **Conventional Commits** para commits e títulos de PR;
- `release/*` apenas quando uma versão precisar de estabilização real;
- nenhuma branch `develop` por padrão.

Prefixos de branch sugeridos:

- `docs/*`
- `proposal/*`
- `feature/*`
- `art/*`
- `release/*`
- `hotfix/*`

## Alternativas consideradas

### GitFlow

Prós:

- Modelo conhecido.
- Separa desenvolvimento, release e hotfix.
- Pode ajudar em produtos com várias versões mantidas em paralelo.

Contras:

- Cria branches long-lived cedo demais.
- Aumenta cerimônia antes de haver código jogável.
- Pode atrasar feedback e revisão.
- Não combina bem com o estágio docs-first e protótipo pequeno.

### Branch livre sem padrão

Prós:

- Menos regras iniciais.
- Mais rápido para experimentos individuais.

Contras:

- Histórico fica difícil de ler.
- Releases e changelog ficam manuais demais.
- Contribuidores novos recebem menos orientação.

## Consequências

### Positivas

- `main` tende a ficar sempre revisada.
- PRs menores facilitam contribuição e revisão.
- Conventional Commits ajudam release notes e changelog.
- O fluxo continua leve durante pré-produção.
- Release branches continuam disponíveis quando forem realmente necessárias.

### Negativas

- Requer disciplina em título de PR e commit.
- Squash merge pode esconder granularidade de commits intermediários.
- Validação manual pode falhar até existir CI.
- Mudanças grandes precisarão ser quebradas em PRs menores.

## Critério de revisão

Revisar esta decisão se:

- o projeto precisar manter várias versões jogáveis em paralelo;
- houver equipe maior trabalhando com ciclos longos;
- releases começarem a exigir estabilização frequente fora de `main`;
- automação de release exigir outro padrão;
- Conventional Commits se mostrar ruído maior que benefício.
