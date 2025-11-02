/// Módulo que implementa a rede NEN-V
///
/// A Network orquestra a simulação, gerindo os neurónios e suas conexões.

use crate::nenv::{NeuronType, NENV};

/// Tipo de topologia de rede
#[derive(Debug, Clone, Copy)]
pub enum ConnectivityType {
    /// Todos os neurónios conectados a todos
    FullyConnected,
    /// Grade 2D onde cada neurónio conecta aos 8 vizinhos (Moore neighborhood)
    Grid2D,
}

/// Estrutura principal da rede NEN-V
#[derive(Debug)]
pub struct Network {
    /// Vetor de todos os neurónios na rede
    pub neurons: Vec<NENV>,

    /// Matriz de conectividade (1 se conectado, 0 se não)
    /// connectivity_matrix[i][j] = 1 significa que neurónio i recebe input de neurónio j
    pub connectivity_matrix: Vec<Vec<u8>>,

    /// Passo de tempo atual da simulação
    pub current_time_step: i64,

    /// Dimensões da grade (para topologia Grid2D)
    pub grid_width: usize,
    pub grid_height: usize,

    /// Nível de alerta global da rede [0.0, 1.0]
    /// 0.0 = estado normal, 1.0 = alerta máximo
    /// Afeta a recuperação de energia de todos os neurónios
    pub alert_level: f64,

    /// Taxa de decaimento do alert_level (retorna gradualmente ao baseline)
    alert_decay_rate: f64,

    /// Novidade média atual da rede (calculada no último update)
    current_avg_novelty: f64,

    /// Threshold de novidade para ativar alert_level automaticamente
    novelty_alert_threshold: f64,

    /// Sensibilidade do boost de alert baseado em novidade
    alert_sensitivity: f64,
}

impl Network {
    /// Cria uma nova rede
    ///
    /// # Argumentos
    /// * `num_neurons` - Número total de neurónios
    /// * `connectivity_type` - Tipo de topologia
    /// * `inhibitory_ratio` - Proporção de neurónios inibitórios (0.0 a 1.0)
    /// * `initial_threshold` - Limiar de disparo inicial para todos os neurónios
    pub fn new(
        num_neurons: usize,
        connectivity_type: ConnectivityType,
        inhibitory_ratio: f64,
        initial_threshold: f64,
    ) -> Self {
        // Calcula dimensões da grade (para Grid2D)
        let (grid_width, grid_height) = match connectivity_type {
            ConnectivityType::Grid2D => {
                let side = (num_neurons as f64).sqrt().ceil() as usize;
                (side, side)
            }
            ConnectivityType::FullyConnected => (0, 0),
        };

        // Gera matriz de conectividade
        let connectivity_matrix =
            Self::generate_connectivity(num_neurons, connectivity_type, grid_width);

        // Cria neurónios
        let mut neurons = Vec::with_capacity(num_neurons);
        let num_inhibitory = (num_neurons as f64 * inhibitory_ratio).floor() as usize;

        for i in 0..num_neurons {
            // Primeiros neurónios são inibitórios, resto é excitatório
            let neuron_type = if i < num_inhibitory {
                NeuronType::Inhibitory
            } else {
                NeuronType::Excitatory
            };

            let neuron = NENV::new(i, num_neurons, initial_threshold, neuron_type);
            neurons.push(neuron);
        }

        Self {
            neurons,
            connectivity_matrix,
            current_time_step: 0,
            grid_width,
            grid_height,
            alert_level: 0.0, // Estado normal inicial
            alert_decay_rate: 0.05, // Decai 5% por passo
            current_avg_novelty: 0.0,
            novelty_alert_threshold: 0.5, // Ativa alert quando novelty > 0.5
            alert_sensitivity: 0.3, // Boost = novelty * 0.3
        }
    }

    /// Gera a matriz de conectividade baseada no tipo
    fn generate_connectivity(
        num_neurons: usize,
        connectivity_type: ConnectivityType,
        grid_width: usize,
    ) -> Vec<Vec<u8>> {
        match connectivity_type {
            ConnectivityType::FullyConnected => {
                // Todos conectados a todos (exceto si mesmo)
                vec![vec![1; num_neurons]; num_neurons]
            }
            ConnectivityType::Grid2D => {
                Self::generate_2d_grid_connectivity(num_neurons, grid_width)
            }
        }
    }

    /// Gera conectividade de grade 2D (Moore neighborhood - 8 vizinhos)
    fn generate_2d_grid_connectivity(num_neurons: usize, width: usize) -> Vec<Vec<u8>> {
        let mut matrix = vec![vec![0; num_neurons]; num_neurons];

        for i in 0..num_neurons {
            let (row, col) = (i / width, i % width);

            // Conecta aos 8 vizinhos (Moore neighborhood)
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue; // Não conecta a si mesmo
                    }

                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    // Verifica limites
                    if new_row >= 0
                        && new_row < width as i32
                        && new_col >= 0
                        && new_col < width as i32
                    {
                        let j = (new_row as usize) * width + (new_col as usize);
                        if j < num_neurons {
                            matrix[i][j] = 1;
                        }
                    }
                }
            }
        }

        matrix
    }

    /// Coleta os inputs para um neurónio específico baseado nas conexões
    ///
    /// # Argumentos
    /// * `neuron_idx` - Índice do neurónio alvo
    /// * `all_outputs` - Vetor com saídas de todos os neurónios
    /// * `external_inputs` - Vetor com inputs externos (opcional)
    ///
    /// # Retorna
    /// Vetor de inputs combinados (rede + externos)
    fn gather_inputs(
        &self,
        neuron_idx: usize,
        all_outputs: &[f64],
        external_inputs: &[f64],
    ) -> Vec<f64> {
        let mut inputs = vec![0.0; self.neurons.len()];

        // Coleta inputs da rede baseado na matriz de conectividade
        for j in 0..self.neurons.len() {
            if self.connectivity_matrix[neuron_idx][j] == 1 {
                inputs[j] = all_outputs[j];
            }
        }

        // Adiciona inputs externos
        for (i, &external_input) in external_inputs.iter().enumerate() {
            if i < inputs.len() {
                inputs[i] += external_input;
            }
        }

        inputs
    }

    /// Executa um passo de atualização da rede
    ///
    /// Este é o coração da simulação, implementando o algoritmo do guia v2:
    /// 1. Coleta saídas do passo anterior
    /// 2. Para cada neurónio: integra, modula, decide disparar
    /// 3. Para cada neurónio: aplica aprendizado e atualiza estado
    ///
    /// # Argumentos
    /// * `external_inputs` - Vetor de inputs externos (um valor por neurónio)
    pub fn update(&mut self, external_inputs: &[f64]) {
        self.current_time_step += 1;

        // Fase 0: Atualiza alert_level (decaimento gradual)
        self.update_alert_level();

        // Coleta todas as saídas do passo anterior
        let all_neuron_outputs: Vec<f64> = self.neurons.iter().map(|n| n.output_signal).collect();

        // Cria vetores temporários para armazenar resultados da Fase 1-3
        let mut integrated_potentials = Vec::with_capacity(self.neurons.len());
        let mut modulated_potentials = Vec::with_capacity(self.neurons.len());
        let mut gathered_inputs = Vec::with_capacity(self.neurons.len());

        // Fase 1-2: Calcular potenciais para todos os neurónios
        for (idx, neuron) in self.neurons.iter().enumerate() {
            let inputs = self.gather_inputs(idx, &all_neuron_outputs, external_inputs);

            let integrated = neuron.dendritoma.integrate(&inputs);
            let modulated = neuron.glia.modulate(integrated);

            integrated_potentials.push(integrated);
            modulated_potentials.push(modulated);
            gathered_inputs.push(inputs);
        }

        // Fase 3: Decisão de disparo para todos os neurónios
        for (neuron, &modulated_potential) in
            self.neurons.iter_mut().zip(modulated_potentials.iter())
        {
            neuron.decide_to_fire(modulated_potential, self.current_time_step);
        }

        // Fase 4: Aprendizado e atualização de estado
        let mut total_novelty = 0.0;

        for (neuron, inputs) in self.neurons.iter_mut().zip(gathered_inputs.iter()) {
            // Calcula novidade ANTES de atualizar memória
            let novelty = neuron.compute_novelty(inputs);
            total_novelty += novelty;

            // Atualiza priority baseado na novidade (sensitivity_factor = 1.0 por padrão)
            neuron.update_priority(novelty, 1.0);

            // Aprendizado (se disparou)
            if neuron.is_firing {
                neuron.dendritoma.apply_learning(inputs);
            }

            // Atualização de estado metabólico
            neuron.glia.update_state(neuron.is_firing);

            // Atualiza memória DEPOIS de calcular novelty
            neuron.update_memory(inputs);
        }

        // Fase 5: Integração Novelty-Alert (v0.3.0)
        // Calcula novidade média da rede
        self.current_avg_novelty = total_novelty / self.neurons.len() as f64;

        // Se novidade excede threshold, boost alert_level automaticamente
        if self.current_avg_novelty > self.novelty_alert_threshold {
            let alert_boost = self.current_avg_novelty * self.alert_sensitivity;
            self.boost_alert_level(alert_boost);
        }
    }

    /// Retorna o número de neurónios na rede
    pub fn num_neurons(&self) -> usize {
        self.neurons.len()
    }

    /// Retorna o número de neurónios que estão disparando no momento
    pub fn num_firing(&self) -> usize {
        self.neurons.iter().filter(|n| n.is_firing).count()
    }

    /// Retorna a energia média da rede
    pub fn average_energy(&self) -> f64 {
        let total_energy: f64 = self.neurons.iter().map(|n| n.glia.energy).sum();
        total_energy / self.neurons.len() as f64
    }

    /// Retorna vetor com estado de disparo de todos os neurónios
    pub fn get_firing_states(&self) -> Vec<bool> {
        self.neurons.iter().map(|n| n.is_firing).collect()
    }

    /// Retorna vetor com níveis de energia de todos os neurónios
    pub fn get_energy_levels(&self) -> Vec<f64> {
        self.neurons.iter().map(|n| n.glia.energy).collect()
    }

    /// Converte índice linear para coordenadas (row, col) na grade
    pub fn index_to_coords(&self, index: usize) -> Option<(usize, usize)> {
        if self.grid_width > 0 && index < self.neurons.len() {
            Some((index / self.grid_width, index % self.grid_width))
        } else {
            None
        }
    }

    /// Converte coordenadas (row, col) para índice linear
    pub fn coords_to_index(&self, row: usize, col: usize) -> Option<usize> {
        if self.grid_width > 0 && row < self.grid_height && col < self.grid_width {
            let index = row * self.grid_width + col;
            if index < self.neurons.len() {
                return Some(index);
            }
        }
        None
    }

    /// Define o nível de alerta global da rede
    ///
    /// O alert_level afeta a recuperação de energia de todos os neurónios.
    /// Valores altos fazem a rede responder mais rapidamente a eventos.
    ///
    /// # Argumentos
    /// * `level` - Nível de alerta [0.0, 1.0]
    pub fn set_alert_level(&mut self, level: f64) {
        self.alert_level = level.clamp(0.0, 1.0);

        // Propaga alert_level para todos os neurónios
        for neuron in &mut self.neurons {
            neuron.glia.alert_level = self.alert_level;
        }
    }

    /// Aumenta o alert_level baseado na atividade global da rede
    ///
    /// Chamado automaticamente quando detecta alta atividade ou novidade.
    /// O alert_level decai gradualmente a cada passo de simulação.
    ///
    /// # Argumentos
    /// * `boost` - Quantidade para aumentar o alert_level
    pub fn boost_alert_level(&mut self, boost: f64) {
        self.alert_level = (self.alert_level + boost).min(1.0);

        // Propaga para todos os neurónios
        for neuron in &mut self.neurons {
            neuron.glia.alert_level = self.alert_level;
        }
    }

    /// Atualiza o alert_level (decaimento gradual para baseline)
    ///
    /// Chamado automaticamente a cada passo de update()
    fn update_alert_level(&mut self) {
        // Decai gradualmente para zero (estado normal)
        self.alert_level *= 1.0 - self.alert_decay_rate;

        // Propaga para neurónios
        for neuron in &mut self.neurons {
            neuron.glia.alert_level = self.alert_level;
        }
    }

    /// Retorna a novidade média da rede (calculada no último update)
    ///
    /// A novidade é a diferença média entre inputs atuais e memória contextual
    /// de todos os neurônios. Valores altos indicam eventos inesperados.
    ///
    /// # Retorna
    /// Novidade média [0.0, ∞), calculada automaticamente durante update()
    pub fn average_novelty(&self) -> f64 {
        self.current_avg_novelty
    }

    /// Configura os parâmetros da integração novelty-alert
    ///
    /// # Argumentos
    /// * `threshold` - Novidade mínima para ativar alert_level [0.0, ∞)
    /// * `sensitivity` - Multiplicador para calcular boost (boost = novelty * sensitivity)
    pub fn set_novelty_alert_params(&mut self, threshold: f64, sensitivity: f64) {
        self.novelty_alert_threshold = threshold.max(0.0);
        self.alert_sensitivity = sensitivity.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_initialization() {
        let network = Network::new(100, ConnectivityType::Grid2D, 0.2, 0.5);

        assert_eq!(network.num_neurons(), 100);
        assert_eq!(network.current_time_step, 0);

        // Verifica proporção de inibitórios
        let num_inhibitory = network
            .neurons
            .iter()
            .filter(|n| n.neuron_type == NeuronType::Inhibitory)
            .count();
        assert_eq!(num_inhibitory, 20);
    }

    #[test]
    fn test_grid_dimensions() {
        let network = Network::new(100, ConnectivityType::Grid2D, 0.2, 0.5);

        assert_eq!(network.grid_width, 10);
        assert_eq!(network.grid_height, 10);
    }

    #[test]
    fn test_coords_conversion() {
        let network = Network::new(100, ConnectivityType::Grid2D, 0.2, 0.5);

        // Testa conversão de ida e volta
        let index = network.coords_to_index(5, 5).unwrap();
        assert_eq!(index, 55);

        let (row, col) = network.index_to_coords(55).unwrap();
        assert_eq!(row, 5);
        assert_eq!(col, 5);
    }

    #[test]
    fn test_2d_grid_connectivity() {
        let network = Network::new(9, ConnectivityType::Grid2D, 0.0, 0.5);

        // Grade 3x3: neurónio central (idx 4) deve ter 8 conexões
        let connections: usize = network.connectivity_matrix[4].iter().map(|&x| x as usize).sum();
        assert_eq!(connections, 8);

        // Neurónio de canto (idx 0) deve ter 3 conexões
        let connections: usize = network.connectivity_matrix[0].iter().map(|&x| x as usize).sum();
        assert_eq!(connections, 3);
    }

    #[test]
    fn test_fully_connected() {
        let network = Network::new(10, ConnectivityType::FullyConnected, 0.0, 0.5);

        // Cada neurónio deve conectar a todos os outros
        for i in 0..10 {
            let connections: usize = network.connectivity_matrix[i]
                .iter()
                .map(|&x| x as usize)
                .sum();
            assert_eq!(connections, 10); // Conecta a todos (incluindo si mesmo na matriz)
        }
    }

    #[test]
    fn test_network_update_increments_time() {
        let mut network = Network::new(10, ConnectivityType::Grid2D, 0.2, 0.5);
        let external_inputs = vec![0.0; 10];

        assert_eq!(network.current_time_step, 0);

        network.update(&external_inputs);
        assert_eq!(network.current_time_step, 1);

        network.update(&external_inputs);
        assert_eq!(network.current_time_step, 2);
    }

    #[test]
    fn test_network_stats() {
        let network = Network::new(10, ConnectivityType::Grid2D, 0.2, 0.5);

        // Energia inicial deve ser 100% (MAX_ENERGY)
        assert_eq!(network.average_energy(), 100.0);

        // Nenhum neurónio deve estar disparando inicialmente
        assert_eq!(network.num_firing(), 0);
    }
}
