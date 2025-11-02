/// Módulo responsável pela modulação metabólica do neurónio
///
/// A Glia modula a atividade do neurónio com base no seu estado metabólico,
/// implementando dinâmicas homeostáticas através da gestão de energia.

#[derive(Debug, Clone)]
pub struct Glia {
    /// Energia atual do neurónio
    pub energy: f64,

    /// Prioridade do neurónio (para uso futuro em mecanismos de atenção)
    pub priority: f64,

    /// Nível de alerta (para uso futuro em estados globais da rede)
    pub alert_level: f64,

    // Constantes metabólicas
    max_energy: f64,
    energy_cost_fire: f64,
    energy_cost_maintenance: f64,
    energy_recovery_rate: f64,
}

impl Glia {
    /// Cria uma nova instância de Glia com parâmetros padrão
    pub fn new() -> Self {
        Self {
            energy: 100.0,
            priority: 1.0,
            alert_level: 0.0,
            max_energy: 100.0,
            energy_cost_fire: 10.0,
            energy_cost_maintenance: 0.1,
            energy_recovery_rate: 2.0,
        }
    }

    /// Cria uma Glia com parâmetros personalizados
    pub fn with_params(
        max_energy: f64,
        energy_cost_fire: f64,
        energy_cost_maintenance: f64,
        energy_recovery_rate: f64,
    ) -> Self {
        Self {
            energy: max_energy,
            priority: 1.0,
            alert_level: 0.0,
            max_energy,
            energy_cost_fire,
            energy_cost_maintenance,
            energy_recovery_rate,
        }
    }

    /// Modula o potencial integrado baseado na energia disponível e priority
    ///
    /// Fórmula v2: potencial_modulado = potencial_integrado * energy_factor * priority
    ///
    /// - energy_factor: [0.0, 1.0] - Reduz potencial quando energia está baixa
    /// - priority: [1.0, 3.0] - Aumenta sensibilidade para inputs novos/importantes
    ///
    /// # Argumentos
    /// * `integrated_potential` - Potencial calculado pelo Dendritoma
    ///
    /// # Retorna
    /// Potencial modulado pronto para decisão de disparo
    pub fn modulate(&self, integrated_potential: f64) -> f64 {
        let energy_factor = (self.energy / self.max_energy).max(0.0);

        // Priority modula a sensibilidade do neurónio
        // Priority > 1.0 aumenta o potencial (neurónio mais reativo)
        // Priority = 1.0 não altera (comportamento padrão)
        integrated_potential * energy_factor * self.priority
    }

    /// Atualiza o estado metabólico da Glia após um passo de simulação
    ///
    /// - Se o neurónio disparou: consome energia
    /// - Se está em repouso: recupera energia (afetada por alert_level)
    /// - Sempre: aplica custo de manutenção
    ///
    /// O alert_level global aumenta a taxa de recuperação quando a rede
    /// está em estado de alerta, permitindo respostas mais rápidas.
    pub fn update_state(&mut self, did_fire: bool) {
        if did_fire {
            // Consome energia ao disparar
            self.energy -= self.energy_cost_fire;
        } else {
            // Recupera energia em repouso (taxa proporcional ao déficit)
            let base_recovery = self.energy_recovery_rate * (1.0 - self.energy / self.max_energy);

            // Alert_level aumenta a recuperação (1.0 = +100% de recuperação)
            let alert_boost = base_recovery * self.alert_level;

            self.energy += base_recovery + alert_boost;
        }

        // Custo de manutenção constante
        self.energy -= self.energy_cost_maintenance;

        // Garante que a energia permaneça dentro dos limites
        self.energy = self.energy.clamp(0.0, self.max_energy);
    }

    /// Retorna a fração de energia atual (0.0 a 1.0)
    pub fn energy_fraction(&self) -> f64 {
        self.energy / self.max_energy
    }
}

impl Default for Glia {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_glia_initialization() {
        let glia = Glia::new();
        assert_eq!(glia.energy, 100.0);
        assert_eq!(glia.priority, 1.0);
        assert_eq!(glia.alert_level, 0.0);
    }

    #[test]
    fn test_modulation_full_energy() {
        let glia = Glia::new();
        let modulated = glia.modulate(50.0);
        assert_relative_eq!(modulated, 50.0, epsilon = 1e-10);
    }

    #[test]
    fn test_modulation_half_energy() {
        let mut glia = Glia::new();
        glia.energy = 50.0;
        let modulated = glia.modulate(50.0);
        assert_relative_eq!(modulated, 25.0, epsilon = 1e-10);
    }

    #[test]
    fn test_energy_consumption_on_fire() {
        let mut glia = Glia::new();
        let initial_energy = glia.energy;
        glia.update_state(true);

        // Energia deve ter diminuído pelo custo de disparo + manutenção
        assert!(glia.energy < initial_energy);
        assert_relative_eq!(
            glia.energy,
            initial_energy - glia.energy_cost_fire - glia.energy_cost_maintenance,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_energy_recovery_at_rest() {
        let mut glia = Glia::new();
        glia.energy = 50.0;
        let initial_energy = glia.energy;

        glia.update_state(false);

        // Energia deve ter aumentado (recuperação > manutenção neste caso)
        assert!(glia.energy > initial_energy);
    }

    #[test]
    fn test_energy_bounds() {
        let mut glia = Glia::new();

        // Testa limite superior
        glia.energy = glia.max_energy;
        for _ in 0..10 {
            glia.update_state(false);
        }
        assert!(glia.energy <= glia.max_energy);

        // Testa limite inferior
        glia.energy = 0.0;
        glia.update_state(true);
        assert!(glia.energy >= 0.0);
    }
}
