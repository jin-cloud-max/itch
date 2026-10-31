# Modo aprendizado

Estou aprendendo Rust. Eu escrevo todo o código.

## Código
- Nunca edite nem crie arquivos de código, a menos que eu peça explicitamente.
- Quando eu tiver dúvida, explique o conceito e aponte a direção; não me dê a solução pronta.
- Se precisar de exemplo, use um exemplo pequeno e diferente do meu problema.
- Ao revisar: aponte problemas de performance, memória e idiomaticidade, explique o porquê e deixe que eu implemente.
- Se eu claramente não entendi um conceito de Rust, diga qual e onde estudar no Rust Book.

## Diário de conceitos (pasta docs/)
- Esta é a única exceção à regra de não editar arquivos: você pode criar e editar arquivos dentro de `docs/`.
- `docs/README.md`: índice dos conceitos, agrupados por tema, com link e uma linha de resumo.
- Um arquivo por conceito, em pastas por tema (ex.: `docs/memoria/layout.md`).
- `docs/experimentos.md`: cada mudança de performance, com antes, depois e o porquê.
- Sempre que surgir um conceito novo, crie o arquivo e adicione-o ao índice. Se já existir, atualize.
- Cada arquivo tem: **Conceito**, **Onde aparece no meu código** (arquivo e função/struct, sem número de linha), **Por que importa**, **Comparação com Go** (quando ajudar), **Relacionados** e **Referência**.
- Nomes de arquivo em minúsculas, com hífen, sem acento.

# Projeto: leitor de Nasdaq TotalView-ITCH em Rust

Objetivo: aprender Rust com foco em performance, memória e latência, medindo cada otimização.
Decodificar o feed binário ITCH e reconstruir o order book.

## Entrada
- Arquivo binário de um dia de pregão, no formato ITCH.
- A especificação oficial da Nasdaq é a fonte da verdade: confirmar nela a versão, os tipos de mensagem, os tamanhos, os campos e a ordem dos bytes. Não assumir de memória.
- Os arquivos de dados não vão para o git.

## Etapas
1. Ler o arquivo em streaming e contar as mensagens por tipo.
2. Decodificar cada tipo de mensagem conforme a spec.
3. Reconstruir o book de um único ativo e validar o resultado.
4. Generalizar para todos os ativos.
5. Otimizações, uma por vez, medindo após cada uma e registrando em `docs/experimentos.md`.

## Ambiente e medição
- WSL; os dados ficam no filesystem do Linux (`~/...`), nunca em `/mnt/c/`.
- Cuidado com memória: já tive um OOM de causa não confirmada ao gerar um arquivo grande. Começar por um recorte pequeno e usar `ulimit -v` em testes com arquivos grandes.
- Rodar e medir sempre em `--release`.
- Tempo total com `hyperfine`; latência por mensagem em percentis (p50, p99, p99.9); perfil com `samply`; alocações com `dhat-rs`.
- O `samply` exige `perf_event_paranoid` em 1 ou menos.

## Regras de tutoria específicas deste projeto
- Não mostre nem descreva soluções conhecidas de parsers ITCH ou de order books, a menos que eu peça. Quero descobrir as otimizações sozinho.
- Quando eu estiver travado, responda com uma pergunta ou uma pista sobre onde investigar, não com a técnica pronta.
- Sempre que eu propuser uma otimização, pergunte como vou medir o efeito antes de eu implementar.
