/// Módulo de visualização para experimentos NEN-V
///
/// Gera gráficos para análise de priority, alert_level e outras métricas.

use plotters::prelude::*;
use std::error::Error;

/// Dados de um passo de simulação para visualização
#[derive(Debug, Clone)]
pub struct SimulationSnapshot {
    pub time: i64,
    pub target_firing: bool,
    pub target_energy: f64,
    pub target_priority: f64,
    pub total_firing: usize,
    pub avg_energy: f64,
    pub alert_level: f64,
}

/// Gera gráfico de priority e alert_level ao longo do tempo
///
/// Cria um gráfico com duas séries temporais:
/// - Priority do neurônio alvo (linha azul)
/// - Alert level global da rede (linha vermelha)
///
/// # Argumentos
/// * `snapshots` - Vetor de snapshots da simulação
/// * `output_path` - Caminho para salvar o gráfico PNG
/// * `title` - Título do gráfico
pub fn plot_priority_and_alert(
    snapshots: &[SimulationSnapshot],
    output_path: &str,
    title: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_time = snapshots.last().map(|s| s.time).unwrap_or(100);
    let max_priority = snapshots
        .iter()
        .map(|s| s.target_priority)
        .fold(1.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..max_time, 0.0..max_priority.max(1.5))?;

    chart
        .configure_mesh()
        .x_desc("Tempo (passos)")
        .y_desc("Valor")
        .draw()?;

    // Linha de priority (azul)
    chart
        .draw_series(LineSeries::new(
            snapshots
                .iter()
                .map(|s| (s.time, s.target_priority)),
            &BLUE,
        ))?
        .label("Priority (neurônio alvo)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Linha de alert_level (vermelho)
    chart
        .draw_series(LineSeries::new(
            snapshots.iter().map(|s| (s.time, s.alert_level)),
            &RED,
        ))?
        .label("Alert Level (global)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// Gera gráfico de energia ao longo do tempo
///
/// Mostra:
/// - Energia do neurônio alvo (linha azul)
/// - Energia média da rede (linha verde)
///
/// # Argumentos
/// * `snapshots` - Vetor de snapshots da simulação
/// * `output_path` - Caminho para salvar o gráfico PNG
/// * `title` - Título do gráfico
pub fn plot_energy(
    snapshots: &[SimulationSnapshot],
    output_path: &str,
    title: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_time = snapshots.last().map(|s| s.time).unwrap_or(100);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..max_time, 0.0..100.0)?;

    chart
        .configure_mesh()
        .x_desc("Tempo (passos)")
        .y_desc("Energia")
        .draw()?;

    // Linha de energia do alvo (azul)
    chart
        .draw_series(LineSeries::new(
            snapshots.iter().map(|s| (s.time, s.target_energy)),
            &BLUE,
        ))?
        .label("Energia (neurônio alvo)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Linha de energia média (verde)
    chart
        .draw_series(LineSeries::new(
            snapshots.iter().map(|s| (s.time, s.avg_energy)),
            &GREEN,
        ))?
        .label("Energia (média da rede)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// Gera gráfico de atividade (número de neurônios disparando)
///
/// # Argumentos
/// * `snapshots` - Vetor de snapshots da simulação
/// * `output_path` - Caminho para salvar o gráfico PNG
/// * `title` - Título do gráfico
pub fn plot_firing_activity(
    snapshots: &[SimulationSnapshot],
    output_path: &str,
    title: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_time = snapshots.last().map(|s| s.time).unwrap_or(100);
    let max_firing = snapshots.iter().map(|s| s.total_firing).max().unwrap_or(100);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..max_time, 0..(max_firing + 10))?;

    chart
        .configure_mesh()
        .x_desc("Tempo (passos)")
        .y_desc("Neurônios disparando")
        .draw()?;

    // Área de atividade
    chart.draw_series(AreaSeries::new(
        snapshots
            .iter()
            .map(|s| (s.time, s.total_firing)),
        0,
        &BLUE.mix(0.3),
    ))?;

    // Linha de atividade
    chart
        .draw_series(LineSeries::new(
            snapshots.iter().map(|s| (s.time, s.total_firing)),
            &BLUE,
        ))?
        .label("Neurônios disparando");

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// Gera todos os gráficos padrão de um experimento
///
/// Cria três gráficos:
/// 1. priority_and_alert.png - Priority e Alert Level
/// 2. energy.png - Energia do alvo e média
/// 3. firing_activity.png - Atividade da rede
///
/// # Argumentos
/// * `snapshots` - Vetor de snapshots da simulação
/// * `prefix` - Prefixo para os nomes dos arquivos (ex: "experiment1")
pub fn generate_all_plots(
    snapshots: &[SimulationSnapshot],
    prefix: &str,
) -> Result<(), Box<dyn Error>> {
    plot_priority_and_alert(
        snapshots,
        &format!("{}_priority_alert.png", prefix),
        &format!("{} - Priority e Alert Level", prefix),
    )?;

    plot_energy(
        snapshots,
        &format!("{}_energy.png", prefix),
        &format!("{} - Energia", prefix),
    )?;

    plot_firing_activity(
        snapshots,
        &format!("{}_activity.png", prefix),
        &format!("{} - Atividade da Rede", prefix),
    )?;

    Ok(())
}
