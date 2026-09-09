# Verificação da distribuição do playtest

Versão: `v0.1.0-prototype.1`.

- 360 testes Rust passaram; fmt e Clippy com warnings como erros passaram.
- Build otimizado local concluído; staging real contém 185 assets (~106 MiB).
- Testes em subprocesso movem o executável para pasta com espaços e iniciam
  fora do checkout; validam assets, áudio, lore, tuning e diretório gravável.
- [Primeira abertura](first-launch.png) e [menu na abertura seguinte](main-menu.png)
  renderizados e conferidos em WSLg. Navegação e modos verificados em testes.
- YAML, links Markdown locais e actionlint aprovados.

O [workflow de release](../../../.github/workflows/release.yml) registra a
verificação dos instaladores Windows, DEB/RPM e inicialização Linux em Ubuntu
22.04/Fedora 44. O smoke Windows verifica carga do executável, imports e
instalação/desinstalação. Compatibilidade com outros PCs e gamepads físicos
ainda precisa de playtest humano. Não houve controle físico disponível neste ambiente.

## Correção de caminhos com acentos no Windows

A [captura do framebuffer](windows-unicode-fight.png) mostra Rust, Java e a arena
Sirius renderizados no Windows 11 nativo, executando de uma pasta temporária com
acentos e com outro diretório de trabalho. O teste usou uma cópia do executável
publicado em `v0.1.0-prototype.1`, alterada somente para incorporar o manifest
`activeCodePage=UTF-8` e `longPathAware`. É evidência da correção isolada; ainda
não representa o binário final de `v0.1.0-prototype.2`.

Antes do manifest, o mesmo caminho causou 145 falhas de leitura de assets.
Depois, foram carregadas 52 imagens, 58 texturas e 86 sons, sem warnings. A
captura foi solicitada por F12 dirigido exclusivamente à janela do processo de
teste, com sua identidade verificada; não houve captura do desktop nem envio
de teclas globais. O processo de teste foi encerrado após salvar a imagem.
