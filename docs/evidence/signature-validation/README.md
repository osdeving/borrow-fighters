# Verificação final — assinatura e arremessos

[Checks Rust](result.json): formatação aprovada, Clippy em todos os targets/features sem warnings e **290 testes aprovados**, nenhum ignorado ou falho. A suíte inclui os espelhos simultâneos corrigidos na revisão final, os cenários contextuais e a metadata dos atlas finais.

[Auditoria de atlas](atlas-audit.json): 11 manifests de ator/efeitos; **510 quadros de ator + 30 efeitos**, todos dentro da imagem, com alpha real e conteúdo visível. Os sete testes de `tools/art/test_build_reviewed_sprite_atlas.py` também passaram nesta rodada.

A [entrega](../../21-signature-spectacle-and-throws.md) liga os laudos visuais dos cinco, 40 cenas gráficas dirigidas, o vídeo, os [controles reais](../signature-runtime-controls/README.md) e as [60 lutas de CPU](../signature-balance/README.md). Os testes de física são independentes da renderização; a inspeção gráfica verifica a apresentação que os testes de dano não demonstram.

As capturas de sprites antecedem a última correção de simultaneidade; seus cenários usam um atacante por vez, portanto essa correção não altera os estados registrados. A auditoria de CPU foi repetida após a correção e o CSV permaneceu idêntico. Ajustes finais de metadados de revisão não alteraram os pixels aprovados.
