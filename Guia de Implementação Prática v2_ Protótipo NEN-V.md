# Guia de Implementação Prática v2: Protótipo NEN-V

**Autor:** Manus AI  
**Data:** 02 de novembro de 2025

---

## 1. Introdução

Este documento é a **versão 2** do guia técnico para a construção de um protótipo funcional da arquitetura NEN-V. Esta versão incorpora o feedback técnico recebido, aprimorando a robustez e o realismo biológico do protótipo inicial. As principais melhorias incluem a **introdução de neurónios inibitórios**, um **mecanismo de normalização de pesos mais sofisticado** e uma expansão das ideias para os próximos passos.

O objetivo continua a ser a tradução de conceitos teóricos em algoritmos concretos, focando nos componentes essenciais para observar a dinâmica de uma pequena rede adaptativa.

**Melhorias nesta versão:**
- Adição de tipos de neurónios (Excitatório/Inibitório).
- Substituição do *clipping* de pesos por **Normalização L2**.
- Refinamento dos algoritmos para incorporar a inibição.
- Adição de uma secção sobre Boas Práticas de Implementação (Logging e Testes).

---

## 2. Estruturas de Dados Essenciais (v2)

As estruturas de dados são refinadas para incluir a distinção entre tipos de neurónios.

### 2.1. O Neurónio: `NENV`

O `NENV` agora inclui um tipo, que determina se o seu sinal de saída será positivo (excitatório) ou negativo (inibitório).

**Pseudocódigo da Estrutura `NENV` (v2):**

```pseudocode
ENUM NeuronType:
    EXCITATORY
    INHIBITORY

CLASS NENV:
    id: Integer
    type: NeuronType // NOVO: Tipo do neurónio

    dendritoma: Dendritoma
    glia: Glia

    memory_trace: Array<Float>
    last_fire_time: Integer
    threshold: Float

    is_firing: Boolean
    output_signal: Float

    FUNCTION initialize(id, num_inputs, initial_threshold, neuron_type):
        self.id = id
        self.type = neuron_type // NOVO
        self.dendritoma = new Dendritoma(num_inputs)
        self.glia = new Glia()
        self.memory_trace = array of zeros with size num_inputs
        self.last_fire_time = -1
        self.threshold = initial_threshold
        self.is_firing = false
        self.output_signal = 0.0
```

### 2.2. A Rede: `Network`

A inicialização da rede agora distribui os tipos de neurónios.

**Pseudocódigo da Estrutura `Network` (v2):**

```pseudocode
CLASS Network:
    neurons: Array<NENV>
    connectivity_matrix: Matrix<Integer>
    current_time_step: Integer

    FUNCTION initialize(num_neurons, connectivity_type, inhibitory_ratio):
        self.neurons = create num_neurons NENVs
        self.connectivity_matrix = generate_connectivity(num_neurons, connectivity_type)
        self.current_time_step = 0

        // NOVO: Define uma percentagem de neurónios como inibitórios
        num_inhibitory = floor(num_neurons * inhibitory_ratio)
        FOR i FROM 0 TO num_inhibitory - 1:
            self.neurons[i].type = NeuronType.INHIBITORY
```

*Nota: As estruturas `Dendritoma` e `Glia` permanecem as mesmas da v1.* 

---

## 3. O Ciclo de Simulação (v2)

O ciclo de simulação permanece conceitualmente o mesmo, mas os algoritmos subjacentes são atualizados para lidar com a nova dinâmica excitatória/inibitória.

**Pseudocódigo da Função `Network.update()` (v2):**

```pseudocode
FUNCTION Network.update(external_inputs):
    self.current_time_step += 1
    // A saída agora pode ser negativa (dos neurónios inibitórios)
    all_neuron_outputs = get_outputs_from_previous_step(self.neurons)

    // Fase 1: Calcular o potencial de entrada para cada neurónio
    FOR EACH neuron IN self.neurons:
        input_signals = gather_inputs_for_neuron(neuron, all_neuron_outputs, external_inputs)
        integrated_potential = neuron.dendritoma.integrate(input_signals)

        // Fase 2: A Glia modula o potencial
        modulated_potential = neuron.glia.modulate(integrated_potential)

        // Fase 3: O NENV decide se dispara
        neuron.decide_to_fire(modulated_potential, self.current_time_step)

    // Fase 4: Atualizar estados e aplicar aprendizado
    FOR EACH neuron IN self.neurons:
        IF neuron.is_firing:
            input_signals = gather_inputs_for_neuron(neuron, all_neuron_outputs, external_inputs)
            // O algoritmo de aprendizado foi APRIMORADO
            neuron.dendritoma.apply_learning_v2(input_signals)
        
        neuron.glia.update_state(neuron.is_firing)
        neuron.update_memory(input_signals)
```

---

## 4. Algoritmos Fundamentais (v2)

Esta secção detalha os algoritmos atualizados, incorporando as melhorias sugeridas.

### 4.1. Integração de Sinais (Dendritoma)

Este algoritmo não muda, mas é importante notar que os `inputs` que ele recebe agora podem ser negativos, permitindo que a rede processe sinais inibitórios.

### 4.2. Decisão de Disparo (NENV)

O sinal de saída agora depende do tipo de neurónio.

**Pseudocódigo da Função `NENV.decide_to_fire()` (v2):**

```pseudocode
FUNCTION NENV.decide_to_fire(modulated_potential, current_time):
    REFRACTORY_PERIOD = 5
    is_in_refractory = (current_time - self.last_fire_time) < REFRACTORY_PERIOD

    self.is_firing = false
    self.output_signal = 0.0

    IF modulated_potential > self.threshold AND NOT is_in_refractory:
        self.is_firing = true
        self.last_fire_time = current_time

        // APRIMORADO: O sinal de saída depende do tipo de neurónio
        IF self.type == NeuronType.EXCITATORY:
            self.output_signal = 1.0
        ELSE: // NeuronType.INHIBITORY
            self.output_signal = -1.0
```

### 4.3. Aprendizado e Atualização de Pesos (Dendritoma) - **MELHORIA CRÍTICA**

Substituímos o *clipping* simples pela **Normalização L2** do vetor de pesos. Isso previne a saturação e mantém a energia total das conexões de entrada estável, resultando em uma dinâmica de aprendizado mais estável e biologicamente plausível.

**Pseudocódigo da Função `Dendritoma.apply_learning_v2()`:**

```pseudocode
FUNCTION Dendritoma.apply_learning_v2(inputs):
    LEARNING_RATE = 0.01

    // Apenas pesos de conexões de neurónios excitatórios são modificados
    // para simplificar o protótipo. A plasticidade inibitória é mais complexa.
    FOR i FROM 0 TO length(self.weights) - 1:
        // A regra Hebbiana aplica-se a inputs positivos (excitatórios)
        IF inputs[i] > 0:
            hebbian_update = LEARNING_RATE * self.plasticity[i] * inputs[i]
            self.weights[i] += hebbian_update

    // APRIMORADO: Normalização L2 em vez de decaimento e clipping
    // Após a atualização Hebbiana, normalizamos o vetor de pesos.
    norm = sqrt(sum(w^2 for w in self.weights))
    IF norm > 0:
        FOR i FROM 0 TO length(self.weights) - 1:
            self.weights[i] = self.weights[i] / norm
```

*Nota: O decaimento passivo foi removido, pois a normalização L2 já serve a um propósito de estabilização. Esta é uma escolha de design para o protótipo que pode ser revisitada.* 

*As funções `Glia.modulate()`, `Glia.update_state()` e `NENV.update_memory()` permanecem as mesmas da v1.* 

---

## 5. Configuração e Inicialização (v2)

A tabela de parâmetros é atualizada para incluir a proporção de neurónios inibitórios.

### 5.1. Tabela de Parâmetros do Protótipo (v2)

| Classe | Parâmetro | Valor Sugerido | Descrição |
| :--- | :--- | :--- | :--- |
| **Network** | `num_neurons` | 100 (grid 10x10) | Número total de neurónios. |
| | `inhibitory_ratio` | **0.2 (20%)** | **NOVO:** Proporção de neurónios inibitórios. |
| **Dendritoma** | `LEARNING_RATE` | 0.01 | Velocidade de aprendizado. |
| ... | *(outros parâmetros da v1)* | ... | ... |

---

## 6. Boas Práticas de Implementação

Para uma implementação bem-sucedida, as seguintes práticas são fortemente recomendadas.

### 6.1. Logging Extensivo

Não registe apenas os disparos. Para depuração e análise, é crucial guardar o estado da rede a cada passo de tempo. Crie um ficheiro de log (ex: `.csv` ou `.json`) que, para cada `t`, armazene:
- O estado de disparo de cada neurónio.
- O nível de `energy` de cada Glia.
- O `potential_modulado` de cada NENV.
- Um snapshot periódico da matriz de `weights` (ex: a cada 100 passos).

### 6.2. Testes Unitários

Antes de executar simulações complexas, valide cada componente isoladamente:
- **Teste da Glia:** Crie um neurónio, não o dispare e verifique se a sua energia recupera até `MAX_ENERGY`. Depois, force o disparo e verifique se `ENERGY_COST_FIRE` é subtraído corretamente.
- **Teste do Dendritoma:** Forneça um vetor de `inputs` e `weights` conhecido e verifique se o `integrated_potential` é calculado corretamente.
- **Teste de Aprendizado:** Dispare um neurónio repetidamente com o mesmo input e verifique se o peso correspondente aumenta e se o vetor de pesos permanece normalizado.

---

## 7. Conclusão e Próximos Passos (v2)

Este guia v2 fornece uma base técnica mais robusta e biologicamente plausível para o protótipo NEN-V. A inclusão de **neurónios inibitórios** e a **normalização L2** são passos cruciais para garantir a estabilidade da rede e permitir a emergência de dinâmicas mais complexas, como oscilações e competição entre grupos de neurónios.

Com este protótipo aprimorado, os próximos passos podem focar-se em explorar o potencial total da arquitetura:

1.  **Ativar o Potencial da Glia:** Implementar a influência dos parâmetros `priority` e `alert_level`. Por exemplo:
    -   `alert_level` poderia ser um valor global que aumenta temporariamente a `ENERGY_RECOVERY_RATE` de todos os neurónios, simulando um estado de "alerta" na rede.
    -   `priority` poderia ser ajustado com base na "novidade" de um padrão de input (diferença em relação à `memory_trace`), fazendo com que o neurónio seja mais sensível a estímulos inesperados.

2.  **Implementar Plasticidade STDP:** Substituir a regra Hebbiana pela *Spike-Timing-Dependent Plasticity* para que a rede possa aprender sequências e relações causais.

3.  **Plasticidade Metabólica:** Uma ideia avançada seria permitir que a própria Glia "aprenda", ajustando os seus parâmetros (`ENERGY_COST_FIRE`, `RECOVERY_RATE`) com base na atividade histórica do seu neurónio, otimizando a eficiência energética localmente.

Este protótipo v2 não é apenas um passo em direção à implementação, mas um refinamento que torna a arquitetura NEN-V uma plataforma de investigação ainda mais promissora.
