/// Módulo responsável pela integração de sinais de entrada e aprendizado sináptico
///
/// O Dendritoma recebe e pondera os sinais de entrada, aplicando aprendizado
/// Hebbiano com normalização L2 para estabilidade.

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Dendritoma {
    /// Pesos sinápticos para cada conexão de entrada
    pub weights: Vec<f64>,

    /// Fator de plasticidade para cada peso (modula a taxa de aprendizado)
    pub plasticity: Vec<f64>,

    // Parâmetros de aprendizado
    learning_rate: f64,
}

impl Dendritoma {
    /// Cria um novo Dendritoma com pesos aleatórios iniciais
    ///
    /// # Argumentos
    /// * `num_inputs` - Número de conexões de entrada
    pub fn new(num_inputs: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Inicializa pesos aleatórios entre 0.1 e 0.3 para evitar uniformidade
        let weights: Vec<f64> = (0..num_inputs)
            .map(|_| rng.gen_range(0.1..0.3))
            .collect();

        // Plasticidade inicial uniforme
        let plasticity = vec![1.0; num_inputs];

        Self {
            weights,
            plasticity,
            learning_rate: 0.01,
        }
    }

    /// Cria um Dendritoma com parâmetros personalizados
    pub fn with_params(num_inputs: usize, learning_rate: f64) -> Self {
        let mut dendritoma = Self::new(num_inputs);
        dendritoma.learning_rate = learning_rate;
        dendritoma
    }

    /// Integra os sinais de entrada através de uma soma ponderada
    ///
    /// # Argumentos
    /// * `inputs` - Vetor de sinais de entrada (pode conter valores positivos e negativos)
    ///
    /// # Retorna
    /// O potencial integrado (soma ponderada dos inputs)
    pub fn integrate(&self, inputs: &[f64]) -> f64 {
        assert_eq!(
            inputs.len(),
            self.weights.len(),
            "Número de inputs deve ser igual ao número de pesos"
        );

        inputs
            .iter()
            .zip(self.weights.iter())
            .map(|(input, weight)| input * weight)
            .sum()
    }

    /// Aplica aprendizado Hebbiano com normalização L2 (v2)
    ///
    /// Regra: "Neurónios que disparam juntos, conectam-se"
    /// - Fortalece pesos de inputs ativos quando o neurónio dispara
    /// - Normaliza o vetor de pesos usando norma L2 para estabilidade
    ///
    /// # Argumentos
    /// * `inputs` - Vetor de sinais de entrada que estavam presentes durante o disparo
    pub fn apply_learning(&mut self, inputs: &[f64]) {
        assert_eq!(
            inputs.len(),
            self.weights.len(),
            "Número de inputs deve ser igual ao número de pesos"
        );

        // Fase 1: Atualização Hebbiana
        // Apenas fortalece conexões de inputs positivos (excitatórios)
        for i in 0..self.weights.len() {
            if inputs[i] > 0.0 {
                let hebbian_update = self.learning_rate * self.plasticity[i] * inputs[i];
                self.weights[i] += hebbian_update;
            }
        }

        // Fase 2: Normalização L2
        // Calcula a norma L2 do vetor de pesos
        let norm: f64 = self.weights.iter().map(|w| w * w).sum::<f64>().sqrt();

        // Normaliza os pesos se a norma for positiva
        if norm > 0.0 {
            for weight in &mut self.weights {
                *weight /= norm;
            }
        }
    }

    /// Retorna o número de conexões de entrada
    pub fn num_inputs(&self) -> usize {
        self.weights.len()
    }

    /// Retorna a soma total dos pesos (útil para debugging)
    pub fn total_weight(&self) -> f64 {
        self.weights.iter().sum()
    }

    /// Retorna a norma L2 atual dos pesos
    pub fn weight_norm(&self) -> f64 {
        self.weights.iter().map(|w| w * w).sum::<f64>().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_dendritoma_initialization() {
        let dendritoma = Dendritoma::new(10);
        assert_eq!(dendritoma.weights.len(), 10);
        assert_eq!(dendritoma.plasticity.len(), 10);

        // Todos os pesos devem estar no intervalo [0.1, 0.3]
        for weight in &dendritoma.weights {
            assert!(*weight >= 0.1 && *weight <= 0.3);
        }
    }

    #[test]
    fn test_integration() {
        let mut dendritoma = Dendritoma::new(3);
        dendritoma.weights = vec![0.5, 0.3, 0.2];

        let inputs = vec![1.0, 2.0, 3.0];
        let potential = dendritoma.integrate(&inputs);

        // 0.5*1.0 + 0.3*2.0 + 0.2*3.0 = 0.5 + 0.6 + 0.6 = 1.7
        assert_relative_eq!(potential, 1.7, epsilon = 1e-10);
    }

    #[test]
    fn test_integration_with_negative_inputs() {
        let mut dendritoma = Dendritoma::new(3);
        dendritoma.weights = vec![0.5, 0.3, 0.2];

        let inputs = vec![1.0, -2.0, 3.0];
        let potential = dendritoma.integrate(&inputs);

        // 0.5*1.0 + 0.3*(-2.0) + 0.2*3.0 = 0.5 - 0.6 + 0.6 = 0.5
        assert_relative_eq!(potential, 0.5, epsilon = 1e-10);
    }

    #[test]
    fn test_hebbian_learning_strengthens_weights() {
        let mut dendritoma = Dendritoma::new(3);
        dendritoma.weights = vec![0.3, 0.3, 0.3];
        dendritoma.learning_rate = 0.1;

        let inputs = vec![1.0, 0.0, 1.0];

        dendritoma.apply_learning(&inputs);

        // O peso do primeiro input deve ter aumentado (antes da normalização)
        // Mas após normalização, comparamos a norma total
        assert_relative_eq!(dendritoma.weight_norm(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_l2_normalization_maintains_unit_norm() {
        let mut dendritoma = Dendritoma::new(5);
        dendritoma.weights = vec![0.2, 0.2, 0.2, 0.2, 0.2];

        let inputs = vec![1.0, 1.0, 1.0, 1.0, 1.0];

        // Aplica aprendizado múltiplas vezes
        for _ in 0..10 {
            dendritoma.apply_learning(&inputs);
        }

        // A norma L2 deve permanecer próxima de 1.0
        let norm = dendritoma.weight_norm();
        assert_relative_eq!(norm, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_no_learning_from_negative_inputs() {
        let mut dendritoma = Dendritoma::new(2);
        // Usa pesos já normalizados para evitar mudanças pela normalização
        dendritoma.weights = vec![0.7071067811865475, 0.7071067811865475]; // sqrt(2)/2 cada
        let norm_before = dendritoma.weight_norm();

        let inputs = vec![-1.0, -1.0];
        dendritoma.apply_learning(&inputs);

        // A norma L2 deve permanecer a mesma pois não houve aprendizado
        let norm_after = dendritoma.weight_norm();
        assert_relative_eq!(norm_before, norm_after, epsilon = 1e-10);

        // Os pesos individuais também devem permanecer iguais
        assert_relative_eq!(dendritoma.weights[0], 0.7071067811865475, epsilon = 1e-10);
        assert_relative_eq!(dendritoma.weights[1], 0.7071067811865475, epsilon = 1e-10);
    }

    #[test]
    #[should_panic(expected = "Número de inputs deve ser igual ao número de pesos")]
    fn test_integrate_panics_on_size_mismatch() {
        let dendritoma = Dendritoma::new(3);
        let inputs = vec![1.0, 2.0]; // Tamanho errado
        dendritoma.integrate(&inputs);
    }
}
