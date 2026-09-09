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
instalação/desinstalação; renderização e gamepads no hardware Windows ainda
precisam de playtest humano. Não houve controle físico disponível neste ambiente.
