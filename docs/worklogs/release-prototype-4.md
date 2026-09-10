# Release v0.1.0-prototype.4

## Pedido e estado

O usuário autorizou fechar a nova release, commitar todas as alterações,
integrar em `main` e publicar. Isso inclui as edições narrativas que as
rodadas anteriores preservaram fora dos commits.

A [pré-release](https://github.com/osdeving/borrow-fighters/releases/tag/v0.1.0-prototype.4)
foi publicada em **2026-09-10 às 12:46:55 UTC**. O [PR #21](https://github.com/osdeving/borrow-fighters/pull/21)
foi integrado à `main`; a tag anotada `v0.1.0-prototype.4` aponta para
`20ef404ec4e3e44cc2dd9277cd10a9180d80d319`. Os cinco pacotes e
`SHA256SUMS.txt` estão publicados e os cinco downloads passaram na conferência
de integridade. A documentação de fechamento foi consolidada na branch
`docs/prototype-4-published` para integração à `main`.

- Branch de preparação: `release/v0.1.0-prototype.4`.
- Base da aventura: `feature/rust-adventure-prologue`, último commit `f90304b`.
- Base pública: `main` / `v0.1.0-prototype.3`, commit `ca63b25`.
- Escopo: aventura → apresentação → menu como entrada padrão; skip por trecho,
  skip total, confirmação final, textos editáveis e menu terminal.
- Distribuição: compilar `borrow-story` e instalá-lo como `borrow-fighters`;
  incluir assets/créditos próprios da aventura. Luta isolada continua no fonte.
- Nenhum texto/asset preexistente foi descartado na preparação.

## Etapas

- [x] Recuperar estado e ler processo de release.
- [x] Criar branch de preparação, atualizar Cargo.toml/Cargo.lock para prototype.4.
- [x] Integrar notas, coletor de assets e workflow.
- [x] Validar Rust, fronteiras, documentos e staging local fora do checkout.
- [x] Commitar tudo, enviar branch e abrir PR para `main`.
- [x] Aprovar builds/pacotes Windows e Linux no Actions.
- [x] Integrar PR e atualizar checkout local para `main`.
- [x] Criar tag anotada no commit integrado e publicar os cinco downloads/checksums.
- [x] Conferir pré-release publicada e os seis assets enviados.
- [x] Conferir os hashes dos pacotes baixados.
- [x] Consolidar a documentação de fechamento para integração em `main`.

## Recuperação

Ler este diário, `docs/06-release-process.md`, `git status` e `git log` antes
de repetir etapas. A release e sua tag já estão publicadas: não recriar nem
mover a tag, nem sobrescrever os pacotes. Os downloads já foram conferidos.
Para retomar o encerramento após uma interrupção, verificar se o PR da branch
`docs/prototype-4-published` foi integrado e sincronizar o checkout local com
`origin/main`, preservando qualquer trabalho novo. Correções futuras no jogo
ou nos pacotes devem receber outra versão/tag.

## Verificação

As rodadas anteriores passaram 432 testes conjuntos, 395 luta, 38 aventura/core,
3 core e 56 fixtures de fronteira. Skip/menu foram verificados em janela X11;
os registros estão em `docs/evidence/story-terminal-menu/navigation/`.
A preparação repetiu os checks pertinentes e validou o executável distribuído.
Os resultados automatizados não substituem playtest humano com diferentes GPUs,
gamepads físicos e avaliação auditiva.

## Checkpoints

- Auditoria dos downloads públicos aprovada: cinco pacotes baixados e todos
  os SHA256 iguais a `SHA256SUMS.txt`. ZIP Windows e tar.gz Linux contêm
  `BUILD-INFO.json` com versão `0.1.0-prototype.4`, `cargo_binary=borrow-story`,
  commit `20ef404ec4e3e44cc2dd9277cd10a9180d80d319` e 225 assets; o executável
  conserva o nome público `borrow-fighters`. Downloads locais em
  `.git/prototype4-downloads`; relatório em `.git/prototype4-release-audit.json`.

- Publicação concluída em 2026-09-10T12:46:55Z, com `isDraft=false`,
  `isPrerelease=true` e seis assets em estado `uploaded`: instalador e ZIP
  Windows, DEB, RPM, tar.gz Linux e `SHA256SUMS.txt`.
  O [workflow da tag](https://github.com/osdeving/borrow-fighters/actions/runs/34477806614)
  concluiu os builds Windows/Linux e a publicação com sucesso.
  A `main` também passou [Rust](https://github.com/osdeving/borrow-fighters/actions/runs/34477777358)
  e [Docs](https://github.com/osdeving/borrow-fighters/actions/runs/34477777381).
  README, changelog, backlog, processo e diários foram atualizados para refletir
  a publicação e preservar as pendências de playtest humano no backlog.

- PR [#21](https://github.com/osdeving/borrow-fighters/pull/21) integrado por
  squash em `main`: `20ef404ec4e3e44cc2dd9277cd10a9180d80d319`. Árvore idêntica
  à preparação `82371af`; todos os checks do PR passaram, incluindo os
  [pacotes Windows/Linux](https://github.com/osdeving/borrow-fighters/actions/runs/34476956396)
  e a [matriz de features](https://github.com/osdeving/borrow-fighters/actions/runs/34477055472).
  Tag anotada `v0.1.0-prototype.4` enviada no commit integrado; a publicação
  terminou no checkpoint acima. Não recriar nem mover a tag.
  Branch `docs/prototype-4-published` reservada para registrar o resultado final.

- Validação local aprovada: 432 testes Rust, Fmt, Clippy, fronteiras e 56
  fixtures; dez testes de empacotamento; YAML e 1204 links Markdown.
  Logs Rust em `/tmp/borrow-prototype4-checks`. Staging real verificado em
  `/tmp/borrow-prototype4-stage`, com 225 assets (36 próprios da aventura).
  O launcher distribuído abriu Ada e o menu fora do checkout, sem override
  de assets; encerrou limpo. Resultado em
  `/tmp/borrow-prototype4-smoke-_2mk7cgc/result.json`.
- Preparação iniciada: manifesto Windows UTF-8 se aplica a todos os binários;
  o rename de `borrow-story` mantém a busca de assets pela instalação.
  Workflow atualizado para --help com código 0, staging do host conjunto,
  smoke de Ada e menu em Linux/Fedora. Builds de revisão de luta continuam
  apontando para o executável isolado.
