/// Módulo que implementa o neurónio NENV (Neurónio-Entrada-Núcleo-Vasos)
///
/// O NENV é a unidade central da arquitetura, integrando o Dendritoma (entrada),
/// a Glia (modulação metabólica) e memória contextual.

use crate::dendritoma::Dendritoma;
use crate::glia::Glia;

/// Tipo de neurónio: Excitatório ou Inibitório
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeuronType {
    /// Neurónios excitatórios emitem sinais positivos (+1.0)
    Excitatory,
    /// Neurónios inibitórios emitem sinais negativos (-1.0)
    Inhibitory,
}

/// Estrutura principal do neurónio NENV
#[derive(Debug, Clone)]
pub struct NENV {
    /// Identificador único do neurónio
    pub id: usize,

    /// Tipo do neurónio (excitatório ou inibitório)
    pub neuron_type: NeuronType,

    /// Componente de entrada e aprendizado sináptico
    pub dendritoma: Dendritoma,

    /// Componente de modulação metabólica
    pub glia: Glia,

    /// Traço de memória contextual (média móvel exponencial dos inputs)
    pub memory_trace: Vec<f64>,

    /// Passo de tempo do último disparo
    pub last_fire_time: i64,

    /// Limiar de disparo
    pub threshold: f64,

    /// Estado de disparo atual
    pub is_firing: bool,

    /// Sinal de saída (+1.0 para excitatório, -1.0 para inibitório, 0.0 se não disparou)
    pub output_signal: f64,

    // Parâmetros de dinâmica
    refractory_period: i64,
    memory_alpha: f64,
}

impl NENV {
    /// Cria um novo neurónio NENV
    ///
    /// # Argumentos
    /// * `id` - Identificador único
    /// * `num_inputs` - Número de conexões de entrada
    /// * `initial_threshold` - Limiar de disparo inicial
    /// * `neuron_type` - Tipo do neurónio (excitatório ou inibitório)
    pub fn new(
        id: usize,
        num_inputs: usize,
        initial_threshold: f64,
        neuron_type: NeuronType,
    ) -> Self {
        Self {
            id,
            neuron_type,
            dendritoma: Dendritoma::new(num_inputs),
            glia: Glia::new(),
            memory_trace: vec![0.0; num_inputs],
            last_fire_time: -1,
            threshold: initial_threshold,
            is_firing: false,
            output_signal: 0.0,
            refractory_period: 5,
            memory_alpha: 0.1,
        }
    }

    /// Cria um neurónio excitatório
    pub fn excitatory(id: usize, num_inputs: usize, initial_threshold: f64) -> Self {
        Self::new(id, num_inputs, initial_threshold, NeuronType::Excitatory)
    }

    /// Cria um neurónio inibitório
    pub fn inhibitory(id: usize, num_inputs: usize, initial_threshold: f64) -> Self {
        Self::new(id, num_inputs, initial_threshold, NeuronType::Inhibitory)
    }

    /// Decide se o neurónio deve disparar baseado no potencial modulado
    ///
    /// # Argumentos
    /// * `modulated_potential` - Potencial após modulação glial
    /// * `current_time` - Passo de tempo atual da simulação
    pub fn decide_to_fire(&mut self, modulated_potential: f64, current_time: i64) {
        // Verifica período refratário
        // Neurônio nunca disparado (last_fire_time = -1) não está em refratário
        let is_in_refractory = if self.last_fire_time < 0 {
            false
        } else {
            (current_time - self.last_fire_time) < self.refractory_period
        };

        // Reset do estado de disparo
        self.is_firing = false;
        self.output_signal = 0.0;

        // Dispara se o potencial excede o limiar e não está em período refratário
        if modulated_potential > self.threshold && !is_in_refractory {
            self.is_firing = true;
            self.last_fire_time = current_time;

            // O sinal de saída depende do tipo de neurónio
            self.output_signal = match self.neuron_type {
                NeuronType::Excitatory => 1.0,
                NeuronType::Inhibitory => -1.0,
            };
        }
    }

    /// Atualiza a memória contextual do neurónio
    ///
    /// Implementa uma média móvel exponencial dos padrões de entrada,
    /// permitindo que o neurónio "lembre" inputs recentes.
    ///
    /// # Argumentos
    /// * `inputs` - Vetor de sinais de entrada atual
    pub fn update_memory(&mut self, inputs: &[f64]) {
        assert_eq!(
            inputs.len(),
            self.memory_trace.len(),
            "Número de inputs deve ser igual ao tamanho da memória"
        );

        for i in 0..self.memory_trace.len() {
            self.memory_trace[i] =
                (1.0 - self.memory_alpha) * self.memory_trace[i] + self.memory_alpha * inputs[i];
        }
    }

    /// Calcula a novidade do padrão de entrada atual
    ///
    /// Novidade é medida como a diferença absoluta média entre o input atual
    /// e a memória contextual (padrões recentes). Valores altos indicam
    /// padrões inesperados ou não familiares.
    ///
    /// # Argumentos
    /// * `inputs` - Vetor de sinais de entrada atual
    ///
    /// # Retorna
    /// Valor de novidade [0.0, ∞), onde 0 = completamente familiar
    pub fn compute_novelty(&self, inputs: &[f64]) -> f64 {
        assert_eq!(
            inputs.len(),
            self.memory_trace.len(),
            "Número de inputs deve ser igual ao tamanho da memória"
        );

        // Calcula diferença absoluta média entre input e memória
        let total_diff: f64 = inputs
            .iter()
            .zip(self.memory_trace.iter())
            .map(|(input, memory)| (input - memory).abs())
            .sum();

        // Normaliza pelo número de inputs para manter escala consistente
        total_diff / inputs.len() as f64
    }

    /// Atualiza o priority da Glia baseado na novidade do input
    ///
    /// Priority aumenta com novidade, tornando o neurónio mais sensível
    /// a padrões inesperados (mecanismo de atenção emergente).
    ///
    /// Fórmula: priority = 1.0 + novelty * sensitivity_factor
    ///
    /// # Argumentos
    /// * `novelty` - Valor de novidade calculado
    /// * `sensitivity_factor` - Multiplicador de sensibilidade (padrão: 1.0)
    pub fn update_priority(&mut self, novelty: f64, sensitivity_factor: f64) {
        // Priority base é 1.0, aumenta proporcionalmente à novidade
        self.glia.priority = 1.0 + novelty * sensitivity_factor;

        // Limita priority a um máximo razoável para evitar instabilidade
        self.glia.priority = self.glia.priority.min(3.0);
    }

    /// Processa um passo completo de atualização do neurónio
    ///
    /// Esta função encapsula o fluxo completo:
    /// 1. Integração de sinais (Dendritoma)
    /// 2. Modulação (Glia)
    /// 3. Decisão de disparo
    /// 4. Aprendizado (se disparou)
    /// 5. Atualização de estado
    ///
    /// # Argumentos
    /// * `inputs` - Vetor de sinais de entrada
    /// * `current_time` - Passo de tempo atual
    ///
    /// # Retorna
    /// O sinal de saída do neurónio
    pub fn step(&mut self, inputs: &[f64], current_time: i64) -> f64 {
        // Fase 1: Integração
        let integrated_potential = self.dendritoma.integrate(inputs);

        // Fase 2: Modulação glial
        let modulated_potential = self.glia.modulate(integrated_potential);

        // Fase 3: Decisão de disparo
        self.decide_to_fire(modulated_potential, current_time);

        // Fase 4: Aprendizado (se disparou)
        if self.is_firing {
            self.dendritoma.apply_learning(inputs);
        }

        // Fase 5: Atualização de estado
        self.glia.update_state(self.is_firing);
        self.update_memory(inputs);

        self.output_signal
    }

    /// Retorna o potencial modulado atual sem modificar o estado
    ///
    /// Útil para debugging e visualização
    pub fn get_modulated_potential(&self, inputs: &[f64]) -> f64 {
        let integrated = self.dendritoma.integrate(inputs);
        self.glia.modulate(integrated)
    }

    /// Define o período refratário
    pub fn set_refractory_period(&mut self, period: i64) {
        self.refractory_period = period;
    }

    /// Define a taxa de atualização da memória
    pub fn set_memory_alpha(&mut self, alpha: f64) {
        self.memory_alpha = alpha.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_nenv_initialization() {
        let neuron = NENV::excitatory(0, 10, 0.5);

        assert_eq!(neuron.id, 0);
        assert_eq!(neuron.neuron_type, NeuronType::Excitatory);
        assert_eq!(neuron.threshold, 0.5);
        assert_eq!(neuron.memory_trace.len(), 10);
        assert!(!neuron.is_firing);
        assert_eq!(neuron.output_signal, 0.0);
    }

    #[test]
    fn test_excitatory_neuron_output() {
        let mut neuron = NENV::excitatory(0, 2, 1.5); // Limiar ajustado

        // Configura pesos não normalizados para garantir disparo
        // potencial = 1.0*1.0 + 1.0*1.0 = 2.0
        // modulado = 2.0 * 1.0 (energia_max) * 1.0 (priority) = 2.0 > 1.5
        neuron.dendritoma.weights = vec![1.0, 1.0];
        neuron.glia.priority = 1.0;

        let inputs = vec![1.0, 1.0];
        let potential = neuron.get_modulated_potential(&inputs);
        neuron.decide_to_fire(potential, 0);

        assert!(neuron.is_firing);
        assert_eq!(neuron.output_signal, 1.0);
    }

    #[test]
    fn test_inhibitory_neuron_output() {
        let mut neuron = NENV::inhibitory(0, 2, 1.5); // Limiar ajustado

        // Configura pesos não normalizados para garantir disparo
        neuron.dendritoma.weights = vec![1.0, 1.0];
        neuron.glia.priority = 1.0;

        let inputs = vec![1.0, 1.0];
        let potential = neuron.get_modulated_potential(&inputs);
        neuron.decide_to_fire(potential, 0);

        assert!(neuron.is_firing);
        assert_eq!(neuron.output_signal, -1.0);
    }

    #[test]
    fn test_refractory_period() {
        let mut neuron = NENV::excitatory(0, 2, 1.5); // Limiar ajustado
        neuron.dendritoma.weights = vec![1.0, 1.0];
        neuron.glia.priority = 1.0;
        neuron.set_refractory_period(5);

        let inputs = vec![1.0, 1.0];

        // Primeiro disparo no tempo 0
        let potential = neuron.get_modulated_potential(&inputs);
        neuron.decide_to_fire(potential, 0);
        assert!(neuron.is_firing);

        // Tentativa de disparo no tempo 2 (dentro do período refratário)
        let potential = neuron.get_modulated_potential(&inputs);
        neuron.decide_to_fire(potential, 2);
        assert!(!neuron.is_firing);

        // Tentativa de disparo no tempo 6 (fora do período refratário)
        let potential = neuron.get_modulated_potential(&inputs);
        neuron.decide_to_fire(potential, 6);
        assert!(neuron.is_firing);
    }

    #[test]
    fn test_memory_update() {
        let mut neuron = NENV::excitatory(0, 3, 0.5);
        neuron.set_memory_alpha(0.5); // Alta taxa para teste rápido

        let inputs1 = vec![1.0, 0.0, 0.0];
        neuron.update_memory(&inputs1);

        // Após uma atualização, memória deve ser 0.5 * inputs1
        assert_relative_eq!(neuron.memory_trace[0], 0.5, epsilon = 1e-10);
        assert_relative_eq!(neuron.memory_trace[1], 0.0, epsilon = 1e-10);

        let inputs2 = vec![0.0, 1.0, 0.0];
        neuron.update_memory(&inputs2);

        // Memória do primeiro canal decai, segundo canal aumenta
        assert_relative_eq!(neuron.memory_trace[0], 0.25, epsilon = 1e-10);
        assert_relative_eq!(neuron.memory_trace[1], 0.5, epsilon = 1e-10);
    }

    #[test]
    fn test_threshold_prevents_firing() {
        let mut neuron = NENV::excitatory(0, 2, 10.0); // Limiar muito alto
        neuron.dendritoma.weights = vec![0.5, 0.5];

        let inputs = vec![1.0, 1.0];
        let potential = neuron.get_modulated_potential(&inputs);

        neuron.decide_to_fire(potential, 0);
        assert!(!neuron.is_firing);
    }

    #[test]
    fn test_energy_depletion_prevents_firing() {
        let mut neuron = NENV::excitatory(0, 2, 0.1);
        neuron.dendritoma.weights = vec![1.0, 1.0];
        neuron.glia.energy = 0.0; // Sem energia

        let inputs = vec![1.0, 1.0];
        let potential = neuron.get_modulated_potential(&inputs);

        // Potencial integrado é alto, mas modulação reduz a zero
        assert_relative_eq!(potential, 0.0, epsilon = 1e-10);

        neuron.decide_to_fire(potential, 0);
        assert!(!neuron.is_firing);
    }

    // === Testes v0.2.0: Priority & Alert Level ===

    #[test]
    fn test_compute_novelty_zero_for_familiar() {
        let mut neuron = NENV::excitatory(0, 3, 0.5);

        // Define memória como um padrão específico
        neuron.memory_trace = vec![0.5, 0.3, 0.2];

        // Input idêntico à memória
        let inputs = vec![0.5, 0.3, 0.2];
        let novelty = neuron.compute_novelty(&inputs);

        // Novidade deve ser zero (completamente familiar)
        assert_relative_eq!(novelty, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_compute_novelty_high_for_novel() {
        let mut neuron = NENV::excitatory(0, 3, 0.5);

        // Memória com zeros (nenhum input recente)
        neuron.memory_trace = vec![0.0, 0.0, 0.0];

        // Input forte e completamente novo
        let inputs = vec![1.0, 1.0, 1.0];
        let novelty = neuron.compute_novelty(&inputs);

        // Novidade deve ser 1.0 (média de diferenças absolutas)
        assert_relative_eq!(novelty, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_compute_novelty_partial() {
        let mut neuron = NENV::excitatory(0, 4, 0.5);

        neuron.memory_trace = vec![0.5, 0.5, 0.5, 0.5];
        let inputs = vec![1.0, 0.0, 1.0, 0.0];

        let novelty = neuron.compute_novelty(&inputs);

        // Diferenças: |1.0-0.5| + |0.0-0.5| + |1.0-0.5| + |0.0-0.5| = 2.0
        // Média: 2.0 / 4 = 0.5
        assert_relative_eq!(novelty, 0.5, epsilon = 1e-10);
    }

    #[test]
    fn test_update_priority_increases_with_novelty() {
        let mut neuron = NENV::excitatory(0, 2, 0.5);

        // Priority inicial deve ser 1.0
        assert_eq!(neuron.glia.priority, 1.0);

        // Atualiza com novelty=0.5 e sensitivity_factor=1.0
        neuron.update_priority(0.5, 1.0);

        // Priority = 1.0 + 0.5*1.0 = 1.5
        assert_relative_eq!(neuron.glia.priority, 1.5, epsilon = 1e-10);
    }

    #[test]
    fn test_update_priority_sensitivity_factor() {
        let mut neuron = NENV::excitatory(0, 2, 0.5);

        // Sensitivity factor = 2.0 (mais sensível)
        neuron.update_priority(0.5, 2.0);

        // Priority = 1.0 + 0.5*2.0 = 2.0
        assert_relative_eq!(neuron.glia.priority, 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_update_priority_clamps_at_max() {
        let mut neuron = NENV::excitatory(0, 2, 0.5);

        // Novelty muito alto com sensitivity alto
        neuron.update_priority(10.0, 1.0);

        // Priority deve ser limitado a 3.0
        assert_eq!(neuron.glia.priority, 3.0);
    }

    #[test]
    fn test_priority_modulates_potential() {
        let mut neuron = NENV::excitatory(0, 2, 0.1);
        neuron.dendritoma.weights = vec![0.7071067811865475, 0.7071067811865475];
        neuron.glia.energy = 100.0; // Energia máxima
        neuron.glia.priority = 2.0; // Priority dobrado

        let inputs = vec![1.0, 1.0];
        let potential = neuron.get_modulated_potential(&inputs);

        // Sem priority: ~1.41
        // Com priority=2.0: ~2.82
        assert!(potential > 2.5);
        assert!(potential < 3.0);
    }

    #[test]
    fn test_priority_enables_firing() {
        let mut neuron = NENV::excitatory(0, 2, 1.5); // Limiar alto
        neuron.dendritoma.weights = vec![0.7071067811865475, 0.7071067811865475];
        neuron.glia.priority = 1.0; // Priority normal

        let inputs = vec![1.0, 1.0];

        // Sem priority alto, não dispara
        let potential = neuron.get_modulated_potential(&inputs);
        neuron.decide_to_fire(potential, 0);
        assert!(!neuron.is_firing);

        // Com priority alto, dispara
        neuron.glia.priority = 2.0;
        let potential_boosted = neuron.get_modulated_potential(&inputs);
        neuron.decide_to_fire(potential_boosted, 1);
        assert!(neuron.is_firing);
    }
}
