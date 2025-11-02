/// Módulo com experimentos para demonstrar funcionalidades do NEN-V
///
/// Cada experimento demonstra um aspecto específico da arquitetura

use crate::network::{ConnectivityType, Network};
use crate::visualization::{generate_all_plots, SimulationSnapshot};
use std::fs::File;
use std::io::Write as IoWrite;

/// Experimento 1: Habituação (implementado em main.rs)
/// Demonstra redução de resposta a estímulo constante

/// Experimento 2: Detecção de Novidade com Priority
///
/// Demonstra como a rede responde diferentemente a padrões familiares vs novos.
///
/// Protocolo:
/// 1. Fase de Familiarização (t=0-100): Apresenta padrão A repetidamente
/// 2. Fase de Teste (t=100-200): Alterna entre padrão A (familiar) e padrão B (novo)
///
/// Comportamento esperado:
/// - Neurónios expostos ao padrão A devem ter priority baixo (≈1.0)
/// - Neurónios expostos ao padrão B devem ter priority alto (>2.0)
/// - Priority alto deve aumentar atividade de disparo
pub fn novelty_detection_experiment() -> std::io::Result<()> {
    println!("=== Experimento 2: Detecção de Novidade ===\n");

    // Configuração
    const NUM_NEURONS: usize = 100;
    const INITIAL_THRESHOLD: f64 = 0.2;
    const MAX_TIME: i64 = 200;

    let mut network = Network::new(
        NUM_NEURONS,
        ConnectivityType::Grid2D,
        0.2,
        INITIAL_THRESHOLD,
    );

    println!("Configuração:");
    println!("  - Neurónios: {}", NUM_NEURONS);
    println!("  - Fase 1 (t=0-100): Padrão A repetido → familiarização");
    println!("  - Fase 2 (t=100-200): Padrões A e B alternados → teste de novidade\n");

    // Neurónios alvo
    const NEURON_A: usize = 33; // Esquerda-superior
    const NEURON_B: usize = 66; // Direita-inferior

    // Cria arquivo de log
    let mut log_file = File::create("novelty_detection_log.csv")?;
    writeln!(
        log_file,
        "time,neuron_a_firing,neuron_a_priority,neuron_a_energy,\
         neuron_b_firing,neuron_b_priority,neuron_b_energy,\
         total_firing,alert_level"
    )?;

    // Vetores para visualização (separados para neurônio A e B)
    let mut snapshots_a = Vec::new();
    let mut snapshots_b = Vec::new();

    // Loop de simulação
    for t in 0..MAX_TIME {
        let external_inputs = generate_novelty_stimulus(NUM_NEURONS, t, NEURON_A, NEURON_B);

        network.update(&external_inputs);

        // Coleta dados
        let neuron_a = &network.neurons[NEURON_A];
        let neuron_b = &network.neurons[NEURON_B];

        // Salva snapshots
        snapshots_a.push(SimulationSnapshot {
            time: t,
            target_firing: neuron_a.is_firing,
            target_energy: neuron_a.glia.energy,
            target_priority: neuron_a.glia.priority,
            total_firing: network.num_firing(),
            avg_energy: network.average_energy(),
            alert_level: network.alert_level,
        });

        snapshots_b.push(SimulationSnapshot {
            time: t,
            target_firing: neuron_b.is_firing,
            target_energy: neuron_b.glia.energy,
            target_priority: neuron_b.glia.priority,
            total_firing: network.num_firing(),
            avg_energy: network.average_energy(),
            alert_level: network.alert_level,
        });

        writeln!(
            log_file,
            "{},{},{:.3},{:.2},{},{:.3},{:.2},{},{:.3}",
            t,
            if neuron_a.is_firing { 1 } else { 0 },
            neuron_a.glia.priority,
            neuron_a.glia.energy,
            if neuron_b.is_firing { 1 } else { 0 },
            neuron_b.glia.priority,
            neuron_b.glia.energy,
            network.num_firing(),
            network.alert_level
        )?;

        // Imprime progresso
        if t % 25 == 0 {
            println!(
                "t={:3} | A: fire={} priority={:.2} | B: fire={} priority={:.2} | alert={:.3}",
                t,
                if neuron_a.is_firing { 1 } else { 0 },
                neuron_a.glia.priority,
                if neuron_b.is_firing { 1 } else { 0 },
                neuron_b.glia.priority,
                network.alert_level
            );
        }
    }

    println!("\n✅ Simulação concluída! Dados salvos em 'novelty_detection_log.csv'");

    // Gera visualizações para neurônio A (familiar)
    println!("📊 Gerando visualizações...");
    if let Err(e) = generate_all_plots(&snapshots_a, "exp2_neuron_a_familiar") {
        eprintln!("⚠️  Erro ao gerar gráficos do neurônio A: {}", e);
    }

    // Gera visualizações para neurônio B (novo)
    if let Err(e) = generate_all_plots(&snapshots_b, "exp2_neuron_b_novel") {
        eprintln!("⚠️  Erro ao gerar gráficos do neurônio B: {}", e);
    } else {
        println!("✅ Gráficos gerados:");
        println!("   Neurônio A (familiar):");
        println!("     - exp2_neuron_a_familiar_priority_alert.png");
        println!("     - exp2_neuron_a_familiar_energy.png");
        println!("     - exp2_neuron_a_familiar_activity.png");
        println!("   Neurônio B (novo):");
        println!("     - exp2_neuron_b_novel_priority_alert.png");
        println!("     - exp2_neuron_b_novel_energy.png");
        println!("     - exp2_neuron_b_novel_activity.png");
    }

    Ok(())
}

/// Gera estímulos para experimento de detecção de novidade
fn generate_novelty_stimulus(
    num_neurons: usize,
    time: i64,
    neuron_a: usize,
    neuron_b: usize,
) -> Vec<f64> {
    let mut inputs = vec![0.0; num_neurons];

    if time < 100 {
        // Fase 1: Apenas padrão A (familiarização)
        inputs[neuron_a] = 2.0;
    } else {
        // Fase 2: Alterna entre A e B a cada 10 passos
        let cycle = ((time - 100) / 10) % 2;
        if cycle == 0 {
            inputs[neuron_a] = 2.0; // Padrão familiar
        } else {
            inputs[neuron_b] = 2.0; // Padrão NOVO
        }
    }

    inputs
}

/// Experimento 3: Resposta a Evento Urgente com Alert Level
///
/// Demonstra como alert_level global acelera a recuperação da rede.
///
/// Protocolo:
/// 1. Fase Normal (t=0-50): Estímulo padrão
/// 2. Evento Urgente (t=50): Ativa alert_level=1.0
/// 3. Fase de Resposta (t=50-150): Observa recuperação acelerada
///
/// Comportamento esperado:
/// - Alert_level decai gradualmente
/// - Energia recupera mais rápido com alert_level alto
/// - Rede mantém maior capacidade de resposta
pub fn urgent_event_experiment() -> std::io::Result<()> {
    println!("=== Experimento 3: Resposta a Evento Urgente ===\n");

    const NUM_NEURONS: usize = 100;
    const INITIAL_THRESHOLD: f64 = 0.2;
    const MAX_TIME: i64 = 150;

    let mut network = Network::new(
        NUM_NEURONS,
        ConnectivityType::Grid2D,
        0.2,
        INITIAL_THRESHOLD,
    );

    println!("Configuração:");
    println!("  - Neurónios: {}", NUM_NEURONS);
    println!("  - t=0-50: Atividade normal");
    println!("  - t=50: EVENTO URGENTE → alert_level=1.0");
    println!("  - t=50-150: Recuperação acelerada\n");

    const TARGET: usize = 55;

    let mut log_file = File::create("urgent_event_log.csv")?;
    writeln!(
        log_file,
        "time,target_firing,target_energy,total_firing,avg_energy,alert_level"
    )?;

    // Vetor para armazenar snapshots para visualização
    let mut snapshots = Vec::new();

    for t in 0..MAX_TIME {
        // Evento urgente em t=50
        if t == 50 {
            println!("⚠️  t=50: EVENTO URGENTE DETECTADO!");
            network.set_alert_level(1.0);
        }

        let mut external_inputs = vec![0.0; NUM_NEURONS];

        // Estímulo contínuo forte
        if t < 60 {
            external_inputs[TARGET] = 2.0;
        }

        network.update(&external_inputs);

        let target = &network.neurons[TARGET];

        // Salva snapshot para visualização
        snapshots.push(SimulationSnapshot {
            time: t,
            target_firing: target.is_firing,
            target_energy: target.glia.energy,
            target_priority: target.glia.priority,
            total_firing: network.num_firing(),
            avg_energy: network.average_energy(),
            alert_level: network.alert_level,
        });

        writeln!(
            log_file,
            "{},{},{:.2},{},{:.2},{:.3}",
            t,
            if target.is_firing { 1 } else { 0 },
            target.glia.energy,
            network.num_firing(),
            network.average_energy(),
            network.alert_level
        )?;

        if t % 15 == 0 {
            println!(
                "t={:3} | Energia={:5.1} | Disparos={:2} | Alert={:.3}",
                t,
                target.glia.energy,
                network.num_firing(),
                network.alert_level
            );
        }
    }

    println!("\n✅ Simulação concluída! Dados salvos em 'urgent_event_log.csv'");

    // Gera visualizações
    println!("📊 Gerando visualizações...");
    if let Err(e) = generate_all_plots(&snapshots, "exp3_urgent_event") {
        eprintln!("⚠️  Erro ao gerar gráficos: {}", e);
    } else {
        println!("✅ Gráficos gerados:");
        println!("   - exp3_urgent_event_priority_alert.png");
        println!("   - exp3_urgent_event_energy.png");
        println!("   - exp3_urgent_event_activity.png");
    }

    Ok(())
}

/// Experimento 4: Integração Novelty-Alert (v0.3.0)
///
/// Demonstra o comportamento emergente da integração entre Priority e Alert Level:
/// Novidade local → Alert global → Recuperação acelerada da rede
///
/// Protocolo:
/// 1. Baseline (t=0-50): Padrão A repetido → familiarização completa
/// 2. Evento Novo (t=50): Introduz padrão B completamente diferente
/// 3. Observação (t=50-150): Monitora cascata de efeitos emergentes
///
/// Comportamento esperado (comportamento EMERGENTE):
/// - t=50: Padrão B → Alta novelty local (neurônios do padrão B)
/// - t=50-55: Alta avg_novelty → Alert_level ativado AUTOMATICAMENTE
/// - t=55-70: Alert_level alto → Recuperação acelerada de TODA a rede
/// - t=70-150: Familiarização com B → avg_novelty cai → alert_level decai
pub fn novelty_alert_integration_experiment() -> std::io::Result<()> {
    println!("=== Experimento 4: Integração Novelty-Alert (v0.3.0) ===\n");

    const NUM_NEURONS: usize = 100;
    const INITIAL_THRESHOLD: f64 = 0.2;
    const MAX_TIME: i64 = 150;

    let mut network = Network::new(
        NUM_NEURONS,
        ConnectivityType::Grid2D,
        0.2,
        INITIAL_THRESHOLD,
    );

    // Configuração da integração (ajustada para rede de 100 neurônios)
    // Threshold baixo para capturar novidade em uma rede pequena
    network.set_novelty_alert_params(0.04, 0.5);

    println!("Configuração:");
    println!("  - Neurónios: {}", NUM_NEURONS);
    println!("  - Novelty threshold: 0.04 (ajustado para rede pequena)");
    println!("  - Alert sensitivity: 0.5");
    println!("  - t=0-50: Padrão A repetido (familiarização)");
    println!("  - t=50: Introduz padrão B NOVO (trigger de novidade)");
    println!("  - t=50-150: Observa cascata emergente\n");

    const NEURON_A: usize = 33;
    const NEURON_B: usize = 66;

    let mut log_file = File::create("integration_experiment_log.csv")?;
    writeln!(
        log_file,
        "time,neuron_a_priority,neuron_b_priority,avg_novelty,alert_level,avg_energy,total_firing"
    )?;

    let mut snapshots = Vec::new();

    for t in 0..MAX_TIME {
        let mut external_inputs = vec![0.0; NUM_NEURONS];

        if t < 50 {
            // Baseline: apenas padrão A
            external_inputs[NEURON_A] = 2.0;
        } else {
            // Evento novo: padrão B (completamente diferente)
            external_inputs[NEURON_B] = 2.0;
        }

        network.update(&external_inputs);

        let neuron_a = &network.neurons[NEURON_A];
        let neuron_b = &network.neurons[NEURON_B];

        // Snapshot para neurônio B (o novo)
        snapshots.push(SimulationSnapshot {
            time: t,
            target_firing: neuron_b.is_firing,
            target_energy: neuron_b.glia.energy,
            target_priority: neuron_b.glia.priority,
            total_firing: network.num_firing(),
            avg_energy: network.average_energy(),
            alert_level: network.alert_level,
        });

        writeln!(
            log_file,
            "{},{:.3},{:.3},{:.3},{:.3},{:.2},{}",
            t,
            neuron_a.glia.priority,
            neuron_b.glia.priority,
            network.average_novelty(),
            network.alert_level,
            network.average_energy(),
            network.num_firing()
        )?;

        if t % 15 == 0 || t == 50 || t == 51 {
            println!(
                "t={:3} | Priority A={:.2} B={:.2} | Novelty={:.3} | Alert={:.3} | Energy={:.1}",
                t,
                neuron_a.glia.priority,
                neuron_b.glia.priority,
                network.average_novelty(),
                network.alert_level,
                network.average_energy()
            );
        }

        // Evento especial em t=50
        if t == 50 {
            println!("    🔥 PADRÃO NOVO INTRODUZIDO! Observando cascata emergente...");
        }
    }

    println!("\n✅ Simulação concluída! Dados salvos em 'integration_experiment_log.csv'");

    // Gera visualizações
    println!("📊 Gerando visualizações...");
    if let Err(e) = generate_all_plots(&snapshots, "exp4_integration") {
        eprintln!("⚠️  Erro ao gerar gráficos: {}", e);
    } else {
        println!("✅ Gráficos gerados:");
        println!("   - exp4_integration_priority_alert.png (KEY: mostra acoplamento)");
        println!("   - exp4_integration_energy.png");
        println!("   - exp4_integration_activity.png");
    }

    println!("\n📈 Análise Esperada:");
    println!("   1. t<50: priority≈1.0, novelty≈0.0, alert≈0.0 (baseline)");
    println!("   2. t=50: novelty dispara (padrão novo)");
    println!("   3. t=51-55: alert_level ativado AUTOMATICAMENTE");
    println!("   4. t=55-70: energia recupera mais rápido (efeito sistêmico)");
    println!("   5. t>70: familiarização → novelty→0, alert→0 (nova baseline)");

    Ok(())
}
