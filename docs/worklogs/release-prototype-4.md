# Release v0.1.0-prototype.4

## Pedido e estado

O usuário autorizou fechar a nova release, commitar todas as alterações,
integrar em `main` e publicar. Isso inclui as edições narrativas que as
rodadas anteriores preservaram fora dos commits.

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
- [ ] Commitar tudo, enviar branch e abrir PR para `main`.
- [ ] Aprovar builds/pacotes Windows e Linux no Actions.
- [ ] Integrar PR e atualizar checkout local para `main`.
- [ ] Criar tag anotada no commit integrado e publicar os cinco downloads/checksums.
- [ ] Conferir release publicada e deixar árvore limpa em `main`.

## Recuperação

Ler este diário, `docs/06-release-process.md`, `git status` e `git log` antes
de repetir etapas. Consultar os runs do workflow Playtest Release e o PR:
não recriar tags nem sobrescrever release publicada. Continuar acompanhando
os runs já iniciados; corrigir falhas na branch antes de publicar a tag.

## Verificação

As rodadas anteriores passaram 432 testes conjuntos, 395 luta, 38 aventura/core,
3 core e 56 fixtures de fronteira. Skip/menu foram verificados em janela X11;
os registros estão em `docs/evidence/story-terminal-menu/navigation/`.
Esta preparação repete os checks pertinentes e valida o executável distribuído.

## Checkpoints

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
