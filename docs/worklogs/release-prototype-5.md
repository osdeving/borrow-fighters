# Diário — release v0.1.0-prototype.5

## Pedido e corte

O responsável aprovou o capítulo jogável e pediu integrar tudo à `main`, atualizar
o protótipo e publicar a release. Esse pedido autoriza push, PR, merge, tag e
publicação dos pacotes conforme o [processo](../06-release-process.md).

- Branch de preparação: `release/v0.1.0-prototype.5`.
- Base jogável: `3ee8e93` em `feature/prologue-scene-improvements`.
- `origin/main` na preparação: `ac05e4b`; base ancestral, 13 commits novos.
- Recuperação anterior ao capítulo: `81c50e6`, preservado no histórico.
- Versão anterior publicada: `v0.1.0-prototype.4`.
- Corte: prólogo revisado e capítulo **Depois do silêncio**, com Rust,
  conversas presenciais/celular, três regiões, combate e checkpoints;
  Versus e seus personagens continuam disponíveis.

## Preparação

- Cargo.toml/Cargo.lock avançam juntos para `0.1.0-prototype.5`.
- README, escopo, backlog, changelog, processo, instruções dos pacotes e
  [notas para jogadores](../releases/v0.1.0-prototype.5.md) atualizados.
- Auditoria do coletor confirmou 263 assets, 166.410.794 bytes, incluindo
  todas as imagens, JSONs e sons do capítulo. Save fica nos dados do usuário.
- Smokes DEB/Fedora ampliados para abrir o capítulo instalado fora do checkout;
  DEB também valida três frames de telemetria, sem menu aberto.
- Nenhuma alteração de gameplay neste corte; a evidência funcional/visual/sonora
  permanece em [Depois do silêncio](../evidence/after-the-silence/README.md).

## Verificação

Checks locais do corte aprovados:

- `cargo fmt --check`.
- `cargo test --locked --all-targets`: 494 aprovados, dois ignorados por
  exigirem dispositivo de áudio, nenhuma falha.
- `cargo clippy --locked --all-targets --all-features -- -D warnings`.
- 19 testes de empacotamento; fronteiras de domínio e suas 56 fixtures.
- 19 YAML e 1.611 links Markdown locais; sintaxe Bash dos smokes e diff limpo.
- Logs locais em `.git/prototype5-audit/`.

Pendente: PR, CI Windows/Linux, integração, tag, publicação com cinco pacotes
e comparação dos downloads com `SHA256SUMS.txt`.

A verificação anterior do capítulo registrou 494 testes Rust aprovados e dois
ignorados por exigirem dispositivo de áudio; matriz de features e Clippy,
rota real com derrota/retry/vitória, 21 checks de navegação/checkpoints e prévia
com áudio nativo. [Comandos e limites](after-the-silence.md).

## Recuperação

Antes de retomar, ler este diário, `git status`, `git log` e o estado dos
workflows/PR/releases no GitHub. Não repetir publicação nem mover tag publicada.
A branch de release deve entrar por PR na `main`; a tag anotada deve apontar
para a revisão integrada e verificada. Após a publicação, registrar URLs,
commits, checksums e o resultado final, mantendo a `main` local sincronizada.
