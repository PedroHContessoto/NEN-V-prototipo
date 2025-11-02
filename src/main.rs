use nen_v_prototipo::{experiments, ConnectivityType, Network};
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("=== NEN-V Protótipo v0.2.0 ===");
    println!("Experimento 2: Detecção de Novidade\n");

    // Executa experimento de detecção de novidade
    experiments::novelty_detection_experiment()?;

    Ok(())
}

/// Experimento de Habituação (Secção 6 do Guia v2)
///
/// Testa se um neurónio reduz sua resposta a um estímulo constante
/// devido ao esgotamento de energia (modulação glial).
fn run_habituation_experiment() -> std::io::Result<()> {
    // Configuração da rede
    const NUM_NEURONS: usize = 100; // Grade 10x10
    const INHIBITORY_RATIO: f64 = 0.2; // 20% inibitórios
    const INITIAL_THRESHOLD: f64 = 0.2; // Limiar mais baixo para permitir disparos
    const MAX_TIME: i64 = 200;

    // Cria a rede
    let mut network = Network::new(
        NUM_NEURONS,
        ConnectivityType::Grid2D,
        INHIBITORY_RATIO,
        INITIAL_THRESHOLD,
    );

    println!("Rede criada:");
    println!("  - {} neurónios (grade 10x10)", NUM_NEURONS);
    println!("  - {}% inibitórios", (INHIBITORY_RATIO * 100.0) as usize);
    println!("  - Limiar inicial: {}\n", INITIAL_THRESHOLD);

    // Neurónio alvo: centro da grade (índice 55 em grade 10x10)
    const TARGET_NEURON: usize = 55;

    println!("Neurónio alvo: {} (centro da grade)", TARGET_NEURON);
    println!("Estímulo aplicado: t=10 até t=100\n");

    // Cria arquivo de log
    let mut log_file = File::create("habituation_log.csv")?;
    writeln!(
        log_file,
        "time,target_firing,target_energy,total_firing,avg_energy"
    )?;

    // Loop de simulação
    for t in 0..MAX_TIME {
        // Gera inputs externos
        let external_inputs = generate_habituation_stimulus(NUM_NEURONS, t, TARGET_NEURON);

        // Atualiza a rede
        network.update(&external_inputs);

        // Coleta dados para análise
        let target_neuron = &network.neurons[TARGET_NEURON];
        let target_firing = if target_neuron.is_firing { 1 } else { 0 };
        let target_energy = target_neuron.glia.energy;
        let total_firing = network.num_firing();
        let avg_energy = network.average_energy();

        // Salva no log
        writeln!(
            log_file,
            "{},{},{:.2},{},{}",
            t, target_firing, target_energy, total_firing, avg_energy
        )?;

        // Imprime progresso a cada 20 passos
        if t % 20 == 0 {
            println!(
                "t={:3} | Alvo: firing={} energia={:5.1} | Rede: firing={:2} energia={:5.1}",
                t, target_firing, target_energy, total_firing, avg_energy
            );
        }
    }

    Ok(())
}

/// Gera estímulo para experimento de habituação
///
/// Aplica um sinal constante forte ao neurónio alvo durante um período específico.
fn generate_habituation_stimulus(
    num_neurons: usize,
    time: i64,
    target_neuron: usize,
) -> Vec<f64> {
    let mut inputs = vec![0.0; num_neurons];

    // Aplica estímulo forte ao neurónio alvo entre t=10 e t=100
    if time > 10 && time < 100 {
        inputs[target_neuron] = 2.0; // Estímulo mais forte para garantir disparo inicial
    }

    inputs
}
