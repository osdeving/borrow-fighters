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
- A preparação da release preservou o gameplay aprovado; a evidência funcional/visual/sonora
  permanece em [Depois do silêncio](../evidence/after-the-silence/README.md).

## Verificação

Checks locais do corte aprovados:

- `cargo fmt --check`.
- `cargo test --locked --all-targets`: 496 aprovados, dois ignorados por
  exigirem dispositivo de áudio, nenhuma falha.
- `cargo clippy --locked --all-targets --all-features -- -D warnings`.
- 19 testes de empacotamento; fronteiras de domínio e suas 56 fixtures.
- 19 YAML e 1.611 links Markdown locais; sintaxe Bash dos smokes e diff limpo.
- Logs locais em `.git/prototype5-audit/`.

## Integração e tag

- Preparação final: `7252ca429ca600402185c6927f29e80a02c5a9aa`.
- [PR #23](https://github.com/osdeving/borrow-fighters/pull/23) integrado por
  squash em 10/09/2026 às 18:02:49 UTC, commit
  `8feb045ca4d34d1ea05d81b7ecb67967cd5115b7`.
- `git diff --exit-code 7252ca4 8feb045` confirmou árvores idênticas.
- Merge pelo administrador após pedido explícito do responsável e checks
  aprovados, conforme exceção operacional da [governança](../05-governance.md).
  Proteções do repositório não foram alteradas.
- [CI do corte](https://github.com/osdeving/borrow-fighters/actions/runs/34510953508):
  Windows e Linux aprovados, inclusive instalação/desinstalação Windows,
  caminhos acentuados, abertura gráfica DEB e RPM/Fedora com prólogo/capítulo.
- [Rust do PR](https://github.com/osdeving/borrow-fighters/actions/runs/34510962115),
  [Docs](https://github.com/osdeving/borrow-fighters/actions/runs/34510962132),
  título e GitGuardian aprovados. Sourcery não revisou o diff por exceder seu
  limite de 20.000 linhas; houve auditoria independente de empacotamento/docs.
- A primeira execução de pacote em `7ded0b1` foi cancelada ao enviar a correção
  documental da contagem dos testes; a execução final acima passou inteira.
- Tag anotada `v0.1.0-prototype.5` enviada, apontando para `8feb045`.
- [Workflow da tag](https://github.com/osdeving/borrow-fighters/actions/runs/34511931462)
  aprovado: Windows, Linux e publicação concluídos.
- [Rust da main](https://github.com/osdeving/borrow-fighters/actions/runs/34511911285)
  e [Docs da main](https://github.com/osdeving/borrow-fighters/actions/runs/34511911270)
  também aprovados. Os testes da tag registraram 496 aprovados e dois ignorados
  em cada plataforma.
- [Pré-release pública](https://github.com/osdeving/borrow-fighters/releases/tag/v0.1.0-prototype.5)
  publicada em 10/09/2026 às 18:13:38 UTC, com cinco pacotes e `SHA256SUMS.txt`.
- Fechamento documental na branch `docs/prototype-5-published`; somente registro
  da publicação, escopo e backlog, preservando a tag e os binários publicados.


A verificação anterior do capítulo registrou 494 testes Rust aprovados e dois
ignorados por exigirem dispositivo de áudio; matriz de features e Clippy,
rota real com derrota/retry/vitória, 21 checks de navegação/checkpoints e prévia
com áudio nativo. [Comandos e limites](after-the-silence.md).

## Downloads verificados

Os cinco downloads públicos foram lidos integralmente e comparados com
`SHA256SUMS.txt` e com os digests publicados pelo GitHub. Os dois portáteis
foram inspecionados: `BUILD-INFO.json` confirma versão `0.1.0-prototype.5`,
commit `8feb045ca4d34d1ea05d81b7ecb67967cd5115b7`, entrada `borrow-story` e
263 assets; todos esses assets e as instruções correspondem ao checkout.

| Arquivo | Bytes | SHA256 |
|---|---:|---|
| `borrow-fighters-0.1.0-prototype.5-1.x86_64.rpm` | 226559746 | `2f63dcf167ddcf3a3604d0ab4f9c8e6d19589ce4d898a989cc63a93874b8c884` |
| `borrow-fighters-0.1.0-prototype.5-linux-x86_64.tar.gz` | 226317995 | `48c264a4ac88bdc7a61bfc46f4257f5640ac185c42489458642222d6d1687df7` |
| `borrow-fighters-0.1.0-prototype.5-windows-x86_64-setup.exe` | 176849045 | `3dbf26b4f7020d3d14e3f5793164700ba5212e8019040bae507f5d5e0765c50d` |
| `borrow-fighters-0.1.0-prototype.5-windows-x86_64.zip` | 177123782 | `2743d4cdec0704f1fb153b1fda282468a3a15bfed7a2cc8c1e31256ba0b02d6e` |
| `borrow-fighters_0.1.0-prototype.5_amd64.deb` | 222523242 | `7379d3a80bd9ec18275ab0970620a743750b584a20d77632adba18de9d235f5c` |

O checkout Windows converte quebras de linha de 53 arquivos de texto para
CRLF. A auditoria confirmou essa transformação exata, sem aceitar outras
alterações; imagens/sons conservaram os bytes, e os assets do ZIP também
conferiram com seu manifesto interno. A primeira comparação usava LF do
checkout Linux e foi ajustada somente após identificar essa diferença.
Não foi necessário mudar ou republicar qualquer pacote.

A verificação usou download sequencial e no máximo um arquivo portátil
temporário por vez; esses arquivos foram removidos ao terminar a inspeção.
Logs, checksums e relatório permanecem em `.git/prototype5-audit/`.

Limites: a CI Windows verifica instalação, manifesto, assets, caminhos com
acentos e carregamento do executável; não comprova renderização ou áudio em
GPU Windows. Playtest de hardware/gamepads continua nos TODO-002/013. O roteiro
nativo Linux do capítulo está registrado na evidência da entrega anterior.

## Recuperação

Antes de retomar, ler este diário, `git status`, `git log` e o estado dos
workflows/PR/releases no GitHub. Não repetir publicação nem mover tag publicada.
O corte está publicado e integrado. Confirmar a integração do fechamento
documental e sincronizar a `main` local antes de nova tarefa. A tag `v0.1.0-prototype.5`
permanece em `8feb045`; correções futuras exigem novo corte, conforme o processo.
