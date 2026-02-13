import init, { compute_combat_damage } from './pkg/rs_aos_stats.js';

let chart = null;

const HIT_RULES = {
    'normal': { name: 'Normal', color: 'rgba(102, 126, 234, 0.8)' },
    'crit_auto_wound': { name: 'Crit Auto-Wound', color: 'rgba(240, 147, 251, 0.8)' },
    'crit_mortal_wound': { name: 'Crit Mortal Wound', color: 'rgba(255, 107, 107, 0.8)' },
    'crit_double_hit': { name: 'Crit Double Hit', color: 'rgba(81, 207, 102, 0.8)' }
};

// Initialize the WASM module
async function initWasm() {
    const loadingEl = document.getElementById('loading');
    const errorEl = document.getElementById('error-message');

    try {
        loadingEl.classList.add('active');
        await init();
        loadingEl.classList.remove('active');
        setupEventListeners();
        updateCalculation();
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
        loadingEl.classList.remove('active');
        errorEl.textContent = `Failed to load calculator: ${error.message}`;
        errorEl.classList.add('active');
    }
}

function setupEventListeners() {
    const inputs = ['attacks', 'to-hit', 'to-wound', 'rend', 'damage', 'save', 'ward'];

    inputs.forEach(id => {
        const element = document.getElementById(id);
        element.addEventListener('input', (e) => {
            const value = parseInt(e.target.value);
            const displayId = `${id}-value`;

            if (id === 'to-hit' || id === 'to-wound' || id === 'save') {
                document.getElementById(displayId).textContent = `${value}+`;
            } else if (id === 'ward') {
                document.getElementById(displayId).textContent = value >= 7 ? 'None' : `${value}+`;
            } else {
                document.getElementById(displayId).textContent = value;
            }

            updateCalculation();
        });
    });
}

function updateCalculation() {
    const errorEl = document.getElementById('error-message');

    try {
        const attacks = parseInt(document.getElementById('attacks').value);
        const toHit = parseInt(document.getElementById('to-hit').value);
        const toWound = parseInt(document.getElementById('to-wound').value);
        const rend = parseInt(document.getElementById('rend').value);
        const damage = parseInt(document.getElementById('damage').value);
        const save = parseInt(document.getElementById('save').value);
        const wardValue = parseInt(document.getElementById('ward').value);
        const ward = wardValue >= 7 ? undefined : wardValue;

        const results = {};

        // Calculate for each hit rule type
        for (const [ruleType, ruleInfo] of Object.entries(HIT_RULES)) {
            const result = compute_combat_damage(
                attacks, toHit, toWound, rend, damage, save, ward, ruleType
            );
            results[ruleType] = {
                data: result,
                info: ruleInfo
            };
        }

        // Update statistics display
        updateStats(results);

        // Update chart
        updateChart(results);

        // Show stats grid
        document.getElementById('stats-grid').style.display = 'grid';

        // Hide error if it was showing
        errorEl.classList.remove('active');

    } catch (error) {
        console.error('Calculation error:', error);
        errorEl.textContent = `Calculation error: ${error.message}`;
        errorEl.classList.add('active');
    }
}

function updateStats(results) {
    const statsGrid = document.getElementById('stats-grid');
    statsGrid.innerHTML = '';

    for (const [ruleType, { data, info }] of Object.entries(results)) {
        const card = document.createElement('div');
        card.className = `stat-card ${ruleType}`;
        card.innerHTML = `
            <div class="label">${info.name}</div>
            <div class="value">${data.mean_damage.toFixed(2)}</div>
            <div class="sublabel">Mean Damage (Max: ${data.max_damage})</div>
        `;
        statsGrid.appendChild(card);
    }
}

function updateChart(results) {
    const ctx = document.getElementById('damage-chart').getContext('2d');

    // Find the maximum damage across all scenarios
    let maxDamage = 0;
    for (const { data } of Object.values(results)) {
        const scenarioMax = Math.max(...data.probabilities.map(p => p.damage));
        maxDamage = Math.max(maxDamage, scenarioMax);
    }

    // Create labels for all possible damage values
    const allLabels = Array.from({ length: maxDamage + 1 }, (_, i) => i.toString());

    // Prepare datasets for each hit rule
    const datasets = [];
    const barWidth = 0.8 / Object.keys(results).length;

    let index = 0;
    for (const [ruleType, { data, info }] of Object.entries(results)) {
        // Create a map of damage -> probability
        const probMap = new Map(data.probabilities.map(p => [p.damage, p.probability * 100]));

        // Fill in all damage values, using 0 for missing ones
        const chartData = allLabels.map(label => probMap.get(parseInt(label)) || 0);

        datasets.push({
            label: `${info.name} (Mean: ${data.mean_damage.toFixed(2)})`,
            data: chartData,
            backgroundColor: info.color,
            borderColor: info.color.replace('0.8', '1'),
            borderWidth: 2,
            borderRadius: 4,
            barPercentage: 0.9,
            categoryPercentage: 1,
        });

        index++;
    }

    if (chart) {
        chart.data.labels = allLabels;
        chart.data.datasets = datasets;
        chart.update();
    } else {
        chart = new Chart(ctx, {
            type: 'bar',
            data: {
                labels: allLabels,
                datasets: datasets
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                interaction: {
                    mode: 'index',
                    intersect: false,
                },
                plugins: {
                    legend: {
                        display: true,
                        position: 'top',
                        labels: {
                            boxWidth: 12,
                            padding: 10,
                            font: {
                                size: 11
                            }
                        }
                    },
                    title: {
                        display: true,
                        text: 'Damage Probability Distribution - Comparison',
                        font: {
                            size: 18,
                            weight: 'bold'
                        },
                        color: '#333'
                    },
                    tooltip: {
                        callbacks: {
                            label: function(context) {
                                return `${context.dataset.label.split(' (')[0]}: ${context.parsed.y.toFixed(2)}%`;
                            }
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        title: {
                            display: true,
                            text: 'Probability (%)',
                            font: {
                                size: 14,
                                weight: 'bold'
                            }
                        },
                        grid: {
                            color: 'rgba(0, 0, 0, 0.05)'
                        }
                    },
                    x: {
                        title: {
                            display: true,
                            text: 'Damage',
                            font: {
                                size: 14,
                                weight: 'bold'
                            }
                        },
                        grid: {
                            display: false
                        }
                    }
                }
            }
        });
    }
}

// Initialize when the page loads
initWasm();
