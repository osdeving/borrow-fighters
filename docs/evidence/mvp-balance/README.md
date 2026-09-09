# Auditoria determinística do combate de MVP

[Antes](before.csv): regras e CPU de `c2f441c`, antes desta rodada. [Depois](after.csv): especiais, postura baixa, guarda, queda e CPU desta revisão. O mesmo [programa de auditoria](../../../examples/balance_audit.rs) produz ambos; para repetir o estado atual, rode na raiz:

```bash
cargo run --quiet --example balance_audit > /tmp/mvp-balance.csv
```

O programa cruza Rust, Duke, C, Python e C++ em ambas as posições: 20 pares ordenados, cada um com três afastamentos iniciais de 0, 32 e 64 pixels adicionais por lutador. São 60 lutas, 24 participações por personagem, com passo de 1/60 s e limite de 120 s. A coluna `spacing` identifica esse afastamento adicional; as outras colunas registram vencedor, duração e vida final.

| Medida | Antes | Depois |
|---|---:|---:|
| Lutas concluídas | 60/60 | 60/60 |
| Duração média | 17,73 s | 21,98 s |
| Vitórias Rust | 12/24 | 11/24 |
| Vitórias Duke | 14/24 | 15/24 |
| Vitórias C | 15/24 | 11/24 |
| Vitórias Python | 5/24 | 12/24 |
| Vitórias C++ | 14/24 | 11/24 |

A comparação inclui alterações nas decisões da CPU, além das regras de combate. Usa as duas seeds fixas de `BasicCpu::for_slot`, dimensões físicas padrão e geometria de fallback; não carrega atlas nem metadata de combate dos sprites. Não representa uma amostra de jogadores, nem estima balanceamento competitivo. Trocar a política da CPU pode mudar bastante as contagens; por isso os resultados não motivaram alterações artificiais de vida ou dano dos golpes anteriores.

A evidência útil é a ausência de travamentos, a conclusão das lutas e o comportamento repetível para comparar mudanças. Os [testes de combate](../../../tests/mvp_combat.rs) verificam separadamente interrupção do startup, defesa correta, punição após bloqueio, pulo contra rasteira/agarrão e recuperação protegida. Os [testes de showcase](../../../tests/move_showcase.rs) verificam contato com os manifests baseline, além da geometria de fallback, e ambos os lados. O próximo corte de balanceamento depende de playtest humano dos cinco personagens.
