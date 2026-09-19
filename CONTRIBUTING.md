# Contributing

Borrow Fighters possui aventura e Versus jogáveis. Comece pelo pedido/issue
e pelo índice do [README](README.md); leia somente o guia da área afetada.
[AGENTS.md](AGENTS.md) e [workflow](docs/09-ai-collaboration.md) definem o fluxo
comum e os níveis de verificação.

1. Preserve trabalho existente e use uma branch curta. Issue é útil para
   propostas abertas; não é pré-requisito de uma correção já autorizada.
2. Implemente uma intenção clara. Mantenha aventura/luta independentes e
   gameplay separado da apresentação. Reutilize ferramentas existentes.
3. Confira o que mudou. Ao concluir Rust, use `./dev check adventure` ou
   `fighting`. Arte precisa de validação do asset e revisão em movimento.
4. Atualize docs somente quando comportamento, comando ou contrato mudou.
   ADR para decisão duradoura difícil de reverter; journal para trabalho longo.
5. Abra PR com problema, resultado e validação. Use Conventional Commits
   (`fix:`, `feat:`, `refactor:`, `docs:`, `art:`) e prefira squash merge.

CI/merge conserva a matriz completa; [release](docs/06-release-process.md)
acrescenta empacotamento e distribuição. Preserve testes úteis e indique origem
e licença dos assets. `main` continua protegida; publicar exige autorização.
