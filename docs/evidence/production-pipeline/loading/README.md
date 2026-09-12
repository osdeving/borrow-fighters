# Carga por contexto — Rust

O capítulo Rust passou de **72 aberturas de PNG (71 caminhos únicos)** para
**58 (57 únicos)**. Saíram 14 imagens exclusivas do prólogo: Ada, quarto,
ambientes da manhã, cinco retratos e seis pinturas de abertura/biografias.
As aberturas de fontes passaram de quatro para três. São contagens de arquivos
abertos, não medições de memória GPU, latência ou FPS.

O [resumo com caminhos e hashes](summary.json) registra o antes no commit
`c6a5c9d` e o depois da separação `SharedAssets`/`Assets`. Quatro capturas do
capítulo e cinco poses de pesar do prólogo ficaram byte a byte idênticas.
O [capítulo na rua](chapter-street.png) e o [pesar no prólogo](prologue-remorse.png)
preservam duas dessas capturas em 1280×720; os PNGs duplicados do antes e os
logs brutos permanecem fora do repositório.

A verificação também montou uma árvore de assets sem 43 arquivos exclusivos
da abertura/manhã, carregou `ChapterAssets` de verdade em Raylib e comparou as
quatro cenas com a carga normal. Isso cobre dependências de imagem indiretas,
além de referências visíveis no código. O prólogo continua usando seu bundle
completo e renderiza `Remorse` com o atlas da manhã.

Reproduzir em Linux com sessão gráfica, `strace`, Rust e dependências locais:

```sh
python3 docs/evidence/production-pipeline/loading/verify.py --output /tmp/borrow-loading-check
```

O diretório de saída precisa estar vazio. O script compila uma cópia temporária
do [harness nativo](capture.rs), usa janelas ocultas e escreve logs, PNGs e
`scope-check.json` somente no destino solicitado. `--check` de outro sistema
não é usado como substituto de carga real.

Limites mantidos: os nove PNGs do catálogo de rua e os 34 do catálogo de
locomoção ainda são carregados em conjunto; metadados de textos/manhã também
continuam compartilhados. `street/props.png` ainda é aberto por dois catálogos.
Esta mudança separa contextos; não acrescenta cache global ou streaming.
