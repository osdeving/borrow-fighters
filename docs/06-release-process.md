# 06 — Processo de Release

## Estado atual

O corte **v0.1.0-prototype.5 — Depois do silêncio** foi integrado à `main` pelo
[PR #23](https://github.com/osdeving/borrow-fighters/pull/23), no commit
`8feb045ca4d34d1ea05d81b7ecb67967cd5115b7`: primeiro capítulo de Rust,
checkpoints, conversa por celular, rua brasileira, evacuação e chegada
cinematográfica. A tag anotada aponta para essa revisão. A
[pré-release](https://github.com/osdeving/borrow-fighters/releases/tag/v0.1.0-prototype.5)
foi publicada em 10 de setembro de 2026, às 18:13 UTC, com cinco pacotes e
`SHA256SUMS.txt`. Os builds Windows/Linux e a publicação passaram no
[workflow da tag](https://github.com/osdeving/borrow-fighters/actions/runs/34511931462).
[Notas da versão](releases/v0.1.0-prototype.5.md) e
[diário da release](worklogs/release-prototype-5.md) acompanham integração,
builds, pacotes e verificação dos downloads. Os cinco arquivos públicos
conferiram com os checksums; os dois portáteis incluem os 263 assets esperados.

A versão publicada anterior é a
[prototype.4](https://github.com/osdeving/borrow-fighters/releases/tag/v0.1.0-prototype.4),
com cinco pacotes e checksums. A [ADR 0019](adr/0019-playtest-distribution.md)
registra plataformas e distribuição; a [ADR 0023](adr/0023-story-to-terminal-menu.md)
registra a entrada conjunta. A correção de imagens e áudio em caminhos Windows
com acentos continua incluída.

## Downloads

| Plataforma | Pacotes | Base de compatibilidade |
|---|---|---|
| Windows x86_64 | Instalador Inno Setup e ZIP portátil | Windows 10 1903+ ou Windows 11, driver OpenGL 3.3 |
| Debian/Ubuntu x86_64 | DEB | Debian 12+, Ubuntu 22.04+ e derivados |
| Fedora x86_64 | RPM | Fedora 44 como ambiente de verificação |
| Linux x86_64 | tar.gz portátil | glibc 2.35+, X11/XWayland e driver OpenGL 3.3 |

Windows usa Raylib e CRT estáticos, com instalação por usuário e desinstalador.
O executável incorpora um manifesto com `activeCodePage=UTF-8`, para as APIs
nativas abrirem imagens e áudio em caminhos como `Jogos/ação çãõ`.
Linux inclui bibliotecas redistribuíveis de X11/áudio; libc e drivers gráficos
permanecem no sistema. Os pacotes incluem os assets usados pelo runtime e suas
referências transitivas, incluindo cenas, sprites, áudio e textos da aventura,
sem vídeos de revisão e materiais de produção. O jogo distribuído compõe
`borrow-story`, com aventura e luta habilitadas, e conserva o nome público
`borrow-fighters` (`borrow-fighters.exe` no Windows).
FFmpeg é opcional e não acompanha a release. Sprite Studio é ferramenta separada.

Créditos, textos de licença e fontes correspondentes acompanham os pacotes.
Os textos `LICENSE-MIT`/`LICENSE-APACHE` formalizam a escolha já declarada no
`Cargo.toml`; assets e bibliotecas mantêm seus próprios créditos e termos.

## Versão e branch

Manter a versão idêntica em `Cargo.toml`, `Cargo.lock` e tag, por exemplo:

```text
0.1.0-prototype.5
v0.1.0-prototype.5
release/v0.1.0-prototype.5
```

A tag e o pacote avançam para cada correção publicada; a branch de estabilização
pode continuar com o nome do corte inicial até o merge do PR.

Conventional Commits continuam sendo usados. A branch de release recebe apenas
empacotamento, instruções, correções e estabilização do corte jogável. O retorno
para `main` é feito por PR; não manter uma `develop` ou um fork do jogo.

## Fluxo automatizado

O workflow [Playtest Release](../.github/workflows/release.yml) faz:

1. Validar versão e presença de notas em `docs/releases/vVERSAO.md`.
2. Compilar/testar a entrada conjunta em Ubuntu 22.04 e Windows 2022 com `Cargo.lock`.
3. Executar fmt, testes com as features padrão, Clippy com todas as features
   e testes do empacotador em ambas as plataformas.
4. Selecionar assets, empacotar dependências e preparar os cinco downloads.
5. Verificar instalação/desinstalação no Windows e carregamento do executável;
   verificar DEB com inicialização gráfica por Xvfb e instalação RPM no Fedora,
   incluindo abertura do primeiro capítulo fora do checkout.
6. Em push de `release/*`, disponibilizar os pacotes como artefatos do Actions.
7. Em push de tag, criar release inicialmente em rascunho, anexar todos os
   downloads e `SHA256SUMS.txt`, e então publicar como pré-release.

O workflow [Rust Check](../.github/workflows/rust-check.yml), executado em Ubuntu
nos PRs e na `main`, verifica também as features isoladas de aventura, luta e
núcleo compartilhado, além das fronteiras entre os domínios. Essa matriz
complementar não roda no job Windows do Playtest Release.

O job de publicação recebe `contents: write`; builds usam apenas leitura.
Uma release já publicada não é sobrescrita pelo workflow. Correções posteriores
recebem nova versão/tag; um rascunho de tentativa interrompida pode ser retomado.
Os smoke tests automatizados não substituem playtest humano com GPU e gamepads.
No Windows, validar também o carregamento nativo de PNG em caminhos com espaços
e acentos: conferir apenas `std::fs`, JSON ou existência de arquivos não detecta
a diferença de codificação nas chamadas C usadas pelo Raylib.

## Preparar e publicar

1. Criar `release/vVERSAO` a partir do estado jogável revisado.
2. Atualizar versões, changelog, backlog e notas para jogadores.
3. Executar `cargo fmt`, `cargo test --locked --all-targets` e
   `cargo clippy --locked --all-targets --all-features -- -D warnings`.
4. Validar links Markdown, YAML e scripts de empacotamento.
5. Commitar, enviar a branch e abrir PR de retorno para `main`.
6. Acompanhar o workflow e corrigir falhas antes de criar a tag.
7. Criar e enviar a tag anotada; conferir a release e os cinco downloads.
8. Registrar o resultado e recolher feedback dos jogadores.

Exemplo de publicação após os checks:

```sh
git tag -a v0.1.0-prototype.5 -m 'release: publish After the Silence story chapter'
git push origin v0.1.0-prototype.5
```

A solicitação explícita do responsável por publicar a release autoriza esse
fluxo. A revisão do PR preserva a integração posterior em `main`.

## Gerar pacotes localmente

As instruções dos scripts estão em [packaging/README.md](../packaging/README.md).
É preciso ambiente de build e ferramentas de empacotamento apenas para quem
produz os pacotes. O jogador instala/extrai o download pronto.

Para verificar integridade depois do download no Linux:

```sh
sha256sum --check SHA256SUMS.txt --ignore-missing
```

No PowerShell, use `Get-FileHash CAMINHO -Algorithm SHA256` e compare com a linha
correspondente em `SHA256SUMS.txt`.

## Critérios de pronto

- Versão e escopo registrados; changelog e notas coerentes com o código.
- Builds, testes e empacotamento aprovados em ambas as plataformas.
- Executável abre fora do checkout, com assets completos e dados graváveis.
- No Windows, PNG e áudio carregam em uma pasta com espaços e acentos.
- Modo História abre o capítulo com assets completos; continuar/recomeçar e
  rever prólogo permanecem disponíveis, com save no diretório de dados do usuário.
- Aventura chega ao menu na mesma janela, com avanço por trecho, skip total
  e confirmação final; `Como jogar` explica como assumir um jogador, reiniciar e sair.
- Créditos e limitações conhecidos acompanham o download.
- Cinco pacotes e checksums disponíveis na mesma GitHub pré-release.
- PR de retorno para `main` aprovado e integrado, com evidência do que foi
  verificado antes da publicação.

A configuração [release.yml](../.github/release.yml) preserva categorias para
notas técnicas geradas pelo GitHub. As notas públicas deste playtest são
curadas: primeiro download e controles, depois novidades e limitações.
