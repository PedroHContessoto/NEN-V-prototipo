# NEN-V Protótipo v0.3.0

**Neurónio-Entrada-Núcleo-Vasos**: Uma arquitetura de rede neural bioinspirada implementada em Rust.

## 🧠 Visão Geral

Este projeto implementa o **Protótipo NEN-V v2** conforme descrito no guia técnico. NEN-V é uma arquitetura que integra princípios biológicos de neurociência para criar redes neurais adaptativas com dinâmicas metabólicas realistas.

### Componentes Principais

1. **Glia** - Modulação metabólica
   - Gestão de energia (consumo, recuperação, manutenção)
   - Homeostase neural
   - Preparado para mecanismos de atenção (`priority`, `alert_level`)

2. **Dendritoma** - Processamento sináptico
   - Integração de sinais (excitatórios e inibitórios)
   - Aprendizado Hebbiano com normalização L2
   - Plasticidade adaptativa

3. **NENV** - Neurónio completo
   - Tipos excitatórios (+1.0) e inibitórios (-1.0)
   - Período refratário
   - Memória contextual (média móvel exponencial)

4. **Network** - Orquestração
   - Topologias: Grade 2D (Moore neighborhood) e totalmente conectada
   - Ciclo de simulação completo
   - Proporção configurável de neurónios inibitórios

## 🚀 Instalação

### Pré-requisitos

- **Rust 1.91+** (Edition 2024)
- **Python 3.8+** (para visualização)
- **pip** (para dependências Python)

### Configuração

```bash
# 1. Clone o repositório
git clone <url-do-repo>
cd NEN-V-prototipo

# 2. Compile o projeto Rust
cargo build --release

# 3. Instale dependências Python
pip install pandas matplotlib
```

## 🧪 Execução

### Executar Experimento de Habituação

```bash
# Compila e executa a simulação
cargo run --release

# Saída esperada:
# - Progresso no terminal
# - Arquivo 'habituation_log.csv' gerado
```

### Visualizar Resultados

```bash
# Gera gráficos a partir do CSV
python visualize.py

# Saída:
# - 'habituation_analysis.png' com 4 gráficos
# - Estatísticas no terminal
```

### Executar Testes

```bash
# Testa todos os componentes (27 testes)
cargo test

# Testa com output detalhado
cargo test -- --nocapture
```

## 📊 Experimento de Habituação

O experimento padrão demonstra **habituação neural**: a redução da resposta a um estímulo constante devido ao esgotamento metabólico.

### Configuração

- **Rede**: 100 neurónios (grade 10x10)
- **Neurónios inibitórios**: 20%
- **Limiar de disparo**: 0.2
- **Neurónio alvo**: 55 (centro da grade)
- **Estímulo**: Sinal de amplitude 2.0 aplicado de t=10 até t=100
- **Duração**: 200 passos de tempo

### Comportamento Esperado

1. **Fase Inicial (t=0-10)**: Rede em repouso
2. **Fase de Estimulação (t=10-100)**:
   - Neurónio alvo recebe input constante
   - Energia diminui gradualmente
   - Taxa de disparo reduz (habituação)
3. **Fase de Recuperação (t=100-200)**:
   - Estímulo cessa
   - Energia recupera gradualmente
   - Rede retorna ao estado de repouso

## 📁 Estrutura do Projeto

```
NEN-V-prototipo/
├── src/
│   ├── lib.rs           # Módulo principal da biblioteca
│   ├── glia.rs          # Modulação metabólica
│   ├── dendritoma.rs    # Processamento sináptico
│   ├── nenv.rs          # Neurónio completo
│   ├── network.rs       # Orquestração da rede
│   └── main.rs          # Experimento de habituação
├── Cargo.toml           # Configuração do projeto Rust
├── visualize.py         # Script de visualização Python
└── README.md            # Este arquivo
```

## 🔬 Parâmetros Configuráveis

### Glia
- `MAX_ENERGY`: 100.0 - Capacidade máxima de energia
- `ENERGY_COST_FIRE`: 10.0 - Custo por disparo
- `ENERGY_COST_MAINTENANCE`: 0.1 - Custo passivo
- `ENERGY_RECOVERY_RATE`: 2.0 - Taxa de recuperação

### Dendritoma
- `LEARNING_RATE`: 0.01 - Velocidade de aprendizado Hebbiano
- Pesos iniciais: Aleatórios [0.1, 0.3]
- Normalização: L2 (mantém norma unitária)

### NENV
- `REFRACTORY_PERIOD`: 5 passos - Período refratário
- `MEMORY_ALPHA`: 0.1 - Taxa de atualização da memória
- `THRESHOLD`: 0.2 - Limiar de disparo

### Network
- Topologia: `Grid2D` (8 vizinhos) ou `FullyConnected`
- `inhibitory_ratio`: 0.2 (20% inibitórios)

## 🧬 Detalhes da Implementação

### Aprendizado Hebbiano com Normalização L2 (v2)

```rust
// 1. Atualização Hebbiana (apenas para inputs positivos)
for i in 0..weights.len() {
    if inputs[i] > 0.0 {
        weights[i] += LEARNING_RATE * plasticity[i] * inputs[i];
    }
}

// 2. Normalização L2
let norm = sqrt(sum(w^2 for w in weights));
for w in weights {
    *w /= norm;
}
```

**Vantagens**:
- Mantém "energia total" das conexões constante
- Evita saturação de pesos
- Competição implícita entre pesos

### Modulação Glial

```rust
modulated_potential = integrated_potential * (energy / MAX_ENERGY)
```

Neurónios com baixa energia têm sua capacidade de disparo reduzida proporcionalmente.

## 📈 Resultados Esperados

O script `visualize.py` gera 4 gráficos:

**A) Atividade de Disparo - Neurónio Alvo**
- Mostra quando o neurónio 55 dispara
- Zona sombreada: período de estímulo

**B) Energia do Neurónio Alvo**
- Depleção durante estimulação
- Recuperação após cessação do estímulo

**C) Atividade Total da Rede**
- Número de neurónios disparando por passo
- Mostra propagação de atividade

**D) Energia Média da Rede**
- Homeostase global
- Efeito do custo de manutenção

## 🔧 Próximos Passos

Conforme o guia v2, as extensões planejadas incluem:

1. **Ativar parâmetros da Glia**:
   - `priority`: Sensibilidade a novidade
   - `alert_level`: Estados globais de alerta

2. **STDP (Spike-Timing-Dependent Plasticity)**:
   - Substituir Hebbiano simples
   - Capturar relações temporais

3. **Plasticidade Metabólica**:
   - Glia aprende parâmetros ótimos
   - Eficiência energética adaptativa

4. **Plasticidade Inibitória**:
   - Atualmente simplificada
   - Importante para dinâmicas complexas

## 🧪 Testes

O projeto inclui **27 testes unitários** cobrindo:

- ✅ Glia: Modulação, consumo/recuperação de energia
- ✅ Dendritoma: Integração, aprendizado Hebbiano, normalização L2
- ✅ NENV: Disparo, período refratário, memória contextual
- ✅ Network: Conectividade, atualização, estatísticas

Execute com:
```bash
cargo test
```

## 📚 Referências

- **Guia de Implementação Prática v2: Protótipo NEN-V**
- Princípios de neurociência computacional
- Spike-Timing-Dependent Plasticity (STDP)
- Redes neurais spiking (SNNs)

## 📄 Licença

MIT License

## 👥 Autor

**Pedro Henrique Cavalhieri Contessoto**

- **Guia Técnico**: Manus AI
- **Implementação**: Pedro Henrique Cavalhieri Contessoto

## 🗂️ Histórico de Versões

### v0.3.0 - Integração Novelty-Alert (branch: `feature/novelty-alert-integration`) ✅ COMPLETO
**Objetivo**: Criar comportamento emergente conectando Priority (local) com Alert Level (global)

**Arquitetura da Integração**:
```
Input Novo → ↑ Novelty (neurônios) → ↑ Priority (local)
                      ↓
              ↑ avg_novelty (rede) → ↑ Alert Level (global AUTOMÁTICO)
                      ↓
           Toda rede → ↑ Recuperação de energia (efeito sistêmico)
```

**Funcionalidades implementadas**:
- ✅ `current_avg_novelty`: Novidade média da rede (calculada a cada update)
- ✅ `novelty_alert_threshold`: Threshold para ativar alert automaticamente (padrão: 0.04)
- ✅ `alert_sensitivity`: Sensibilidade do boost (padrão: 0.5)
- ✅ Boost automático em `Network::update()` (Fase 5)
- ✅ `set_novelty_alert_params()`: Configuração dos parâmetros
- ✅ **Experimento 4**: Validação da cascata emergente
- ✅ 39 testes unitários passando
- ✅ Visualizações mostrando acoplamento

**Comportamento Emergente Observado** (Experimento 4):
1. **t<50**: Baseline - priority≈1.0, novelty≈0.0, alert≈0.0
2. **t=50**: Padrão novo → novelty=0.053 (↑ detectado)
3. **t=51**: Alert ativado AUTOMATICAMENTE (0.060) 🔥
4. **t=51-75**: Energia recupera mais rápido (34→59) devido ao alert
5. **t>75**: Familiarização → novelty→0, alert→0 (nova baseline)

**Insight Chave**:
> "A rede não precisa de controle externo para entrar em estado de alerta.
> A própria detecção de novidade ativa automaticamente mecanismos globais de resposta."

**Aplicações**:
- Detecção de anomalias com resposta adaptativa automática
- Sistemas de vigilância que "acordam" com eventos inesperados
- Redes que coordenam atenção local com prontidão global

### v0.2.0 - Priority & Alert Level (branch: `feature/glia-priority-alert`) ✅ COMPLETO
**Objetivo**: Ativar parâmetros `priority` e `alert_level` da Glia para atenção emergente

Funcionalidades implementadas:
- ✅ **Priority baseado em novidade**: Neurónios detectam padrões inesperados
- ✅ **Alert_level global**: Modula recuperação de energia de toda a rede
- ✅ **Experimento 2**: Detecção de novidade (padrão familiar vs. novo)
- ✅ **Experimento 3**: Resposta a evento urgente
- ✅ Integração no ciclo de simulação
- ✅ Testes validados com sucesso

**Comportamentos emergentes observados:**
- Priority aumenta automaticamente para inputs novos (atenção seletiva)
- Alert_level acelera recuperação energética (+100% com alert=1.0)
- Decaimento exponencial retorna sistema ao baseline
- Rede responde mais rápido após eventos urgentes

### v0.1.0 - Protótipo Base (branch: `master`) ✅
- ✅ Implementação completa do guia v2
- ✅ Componentes: Glia, Dendritoma, NENV, Network
- ✅ Neurónios excitatórios e inibitórios
- ✅ Aprendizado Hebbiano com normalização L2
- ✅ Experimento de habituação
- ✅ 27 testes unitários
- ✅ Visualização Python

---

**Branch Atual**: `feature/glia-priority-alert`
**Status**: ✅ Priority & Alert Level implementados e validados

**Última atualização**: 02 de novembro de 2025

## 🆕 Novidades v0.2.0

### Priority (Prioridade Neural)

**Conceito**: Neurónios ajustam automaticamente sua sensibilidade baseado na novidade do input.

```rust
// Cálculo de novidade (diferença com memória)
let novelty = neuron.compute_novelty(inputs);

// Priority aumenta com novidade
neuron.update_priority(novelty, sensitivity_factor);

// Modula o potencial
modulated = potential * energy_factor * priority
```

**Fórmula**: `priority = 1.0 + novelty * sensitivity_factor` (limitado a 3.0)

**Aplicações**:
- Detecção de anomalias
- Atenção seletiva emergente
- Resposta aumentada a padrões inesperados

### Alert Level (Nível de Alerta Global)

**Conceito**: Estado global da rede que acelera recuperação energética em situações urgentes.

```rust
// Ativa alerta máximo
network.set_alert_level(1.0);

// Recuperação acelerada
recovery = base_recovery * (1.0 + alert_level)

// Decai automaticamente
alert_level *= (1.0 - decay_rate)
```

**Parâmetros**:
- Range: [0.0, 1.0]
- Decay rate: 0.05 (5% por passo)
- Efeito: +100% recuperação com alert=1.0

**Aplicações**:
- Resposta rápida a eventos críticos
- Coordenação global da rede
- Simulação de estados de vigilância

### Experimentos Disponíveis

Execute com: `cargo run --release`

**Experimento 3: Resposta a Evento Urgente** (atual)
- Demonstra alert_level em ação
- Estímulo forte → Evento urgente (t=50)
- Observa recuperação acelerada
