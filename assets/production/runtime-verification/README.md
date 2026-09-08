# Verificação da seleção de arte no runtime

O [relatório do carregador](loader-report.json) registra dez execuções nativas de
`GameAssets::load`, com Raylib e texturas reais na GPU. Cada execução usa um
diretório de fixtures separado: os assets originais do projeto não foram
renomeados nem corrompidos para testar fallback.

Foram conferidos: variável ausente, `1`, `0`, valor inválido, candidato ausente,
JSON inválido, conjunto sem `throw`, atlas ausente, atlas inválido e projétil
inválido. O caso sem variável carrega o Rust revisado de 71 quadros e sua
engrenagem de 237 × 116 px; o modo original usa 51 quadros e 229 × 108 px.

Além da comparação integral do manifesto visual carregado, todas as execuções
comparam `combat_manifest` com o baseline, conferem texturas e limites dos
recortes e verificam o fallback independente de Duke. O projétil continua
independente do atlas: um JSON de lutador inválido não desativa uma textura de
projétil válida. Isso preserva o comportamento anterior.

O harness temporário e os logs completos estão em
`target/art/default-loader-smoke/check.rs` e nos `.log` desse diretório. Ele foi
compilado como exemplo local contra a biblioteca atual e executado em processos
separados, sem alterar variáveis globais dentro de testes Rust concorrentes.
Esta prova verifica carregamento e isolamento do combate; os laudos de arte de
cada personagem contêm a revisão visual.

O [relatório dos assets atuais](final-assets-report.json) confere os seis conjuntos
canônicos: 120 clips, 378 quadros, alpha real, recortes válidos e ausência de aliases
entre ações. Registra também os 234 testes Rust após a integração do novo Go, sete
testes Python, formatação, Clippy e a preservação dos diretórios de combate,
personagens, World, placeholders e métricas físicas. As pastas históricas de Go
não entram na contagem do elenco.
