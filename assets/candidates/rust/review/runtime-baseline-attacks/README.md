# Rust — caixas baseline no Combat Lab corrigido

Captura GPU nativa com combat_manifest baseline, após corrigir o Lab para projetar a mesma metadata da partida. Primeiro frame ativo de jab, heavy e kick inspecionado: punho ou pé cruza a caixa efetiva. Jab concentra a mão na metade próxima da caixa; heavy toca a metade superior/final; kick tem a sola quase toda dentro, com pequena borda inferior fora. Nenhuma caixa/tempo/dano foi alterado.

[Relatório completo](capture-report.json) e três amostras selecionadas preservados aqui: [jab](punch_light-f004-active.png), [heavy](punch_heavy-f011-active.png) e [kick](kick-f009-active.png); demais PNGs em `target/art/rust-lab-baseline-attacks-final`. As capturas históricas anteriores usavam MoveSpec puro e não comprovavam esses três contatos do Rust.
