# Auditoria de CPU: especiais e arremessos

[bouts.csv](bouts.csv) registra **60 lutas**, todas encerradas antes do limite de 120 segundos: média **19,30 s**, máximo **30,92 s**. Vitórias: Duke 18, C++ 12, Rust 11, C 9 e Python 8; dois empates por ambos chegarem a zero HP.

Reprodução, na raiz do repositório:

```sh
cargo run --quiet --example balance_audit > /tmp/signature-balance.csv
```

O [exemplo](../../../examples/balance_audit.rs) percorre os 20 pares ordenados dos cinco personagens, com três deslocamentos iniciais por par. A coluna `spacing` desloca cada lutador em 0, 32 ou 64 pixels de runtime para fora de sua posição inicial; a distância adicional total é o dobro. Cada personagem aparece 24 vezes. A simulação usa passo fixo de 1/60 s, duas sementes determinísticas associadas aos slots da CPU, métricas corporais de fallback e nenhum manifesto de sprites.

Essa amostra detecta travamentos e assimetrias grosseiras da CPU. Não estima equilíbrio competitivo, variedade de decisões humanas ou colisões com todos os atlas. Repetir o comando com o mesmo código reproduz as mesmas lutas; não acrescenta amostras independentes. Os testes de showcase e de combate cobrem separadamente metadata, defesa, trajetórias e recuperação.

A auditoria foi refeita após corrigir a resolução de especiais simultâneos e a origem frontal da barreira de Rust. O CSV permaneceu idêntico nas 60 lutas. Esta matriz exclui confrontos do mesmo personagem: o caso Python contra Python que revelou o viés de slot é coberto separadamente pelas regressões de simultaneidade em [signature_effects.rs](../../../tests/signature_effects.rs), junto das barreiras simultâneas e da interrupção por jab antes da emissão.
