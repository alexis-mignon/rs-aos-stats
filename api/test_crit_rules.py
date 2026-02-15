#!/usr/bin/env python3
"""
Test suite to verify that crit rules always increase average damage.

This test ensures the API correctly implements the Age of Sigmar critical hit rules
and that they always provide a benefit compared to normal hit rules.
"""

import json
import subprocess
import sys


def call_api(params):
    """Call the API and return the result"""
    cmd = [
        'curl', '-s', '-X', 'POST', 'http://localhost:8001/api/calculate',
        '-H', 'Content-Type: application/json',
        '-d', json.dumps(params)
    ]
    result = subprocess.run(cmd, capture_output=True, text=True)
    return json.loads(result.stdout)


def test_crit_rules_increase_damage():
    """Test that all crit rules increase average damage compared to normal"""

    test_cases = [
        {
            "name": "Standard profile (10A, 3+/3+, -1 rend, 1D, 4+ save)",
            "params": {
                "attacks_characteristic": "10",
                "to_hit": 3,
                "to_wound": 3,
                "rend": 1,
                "damage_characteristic": "1",
                "save": 4,
                "ward": None
            }
        },
        {
            "name": "Elite profile (D6A, 2+/2+, -2 rend, D3D, 3+ save)",
            "params": {
                "attacks_characteristic": "D6",
                "to_hit": 2,
                "to_wound": 2,
                "rend": 2,
                "damage_characteristic": "D3",
                "save": 3,
                "ward": None
            }
        },
        {
            "name": "Horde profile (2D6A, 4+/4+, 0 rend, 1D, 5+ save)",
            "params": {
                "attacks_characteristic": "2D6",
                "to_hit": 4,
                "to_wound": 4,
                "rend": 0,
                "damage_characteristic": "1",
                "save": 5,
                "ward": None
            }
        },
        {
            "name": "With ward save (10A, 3+/3+, -1 rend, 2D, 4+ save, 5+ ward)",
            "params": {
                "attacks_characteristic": "10",
                "to_hit": 3,
                "to_wound": 3,
                "rend": 1,
                "damage_characteristic": "2",
                "save": 4,
                "ward": 5
            }
        },
        {
            "name": "Edge: Only crits hit (10A, 6+ to hit)",
            "params": {
                "attacks_characteristic": "10",
                "to_hit": 6,
                "to_wound": 3,
                "rend": 0,
                "damage_characteristic": "1",
                "save": 4,
                "ward": None
            }
        },
        {
            "name": "Edge: Very easy to hit (10A, 2+ to hit)",
            "params": {
                "attacks_characteristic": "10",
                "to_hit": 2,
                "to_wound": 3,
                "rend": 1,
                "damage_characteristic": "1",
                "save": 4,
                "ward": None
            }
        },
        {
            "name": "Edge: High damage (2D6A, D6 damage)",
            "params": {
                "attacks_characteristic": "2D6",
                "to_hit": 3,
                "to_wound": 3,
                "rend": 1,
                "damage_characteristic": "D6",
                "save": 4,
                "ward": None
            }
        },
        {
            "name": "Edge: No save (10A, 3+/3+, 7+ save)",
            "params": {
                "attacks_characteristic": "10",
                "to_hit": 3,
                "to_wound": 3,
                "rend": 0,
                "damage_characteristic": "2",
                "save": 7,
                "ward": None
            }
        }
    ]

    hit_rules = ["normal", "crit_auto_wound", "crit_mortal_wound", "crit_double_hit"]

    print("=" * 80)
    print("Testing: All crit rules increase average damage")
    print("=" * 80)

    all_pass = True
    total_tests = 0
    passed_tests = 0

    for test_case in test_cases:
        print(f"\n{test_case['name']}")
        print("-" * 80)

        results = {}
        for rule in hit_rules:
            params = test_case['params'].copy()
            params['hit_rule_type'] = rule
            result = call_api(params)
            mean = result['mean_damage']
            results[rule] = mean
            print(f"  {rule:25s}: {mean:.4f}")

        # Check if all crit rules have higher damage than normal
        normal_damage = results['normal']
        for rule in ['crit_auto_wound', 'crit_mortal_wound', 'crit_double_hit']:
            total_tests += 1
            if results[rule] <= normal_damage:
                print(f"  ❌ FAIL: {rule} ({results[rule]:.4f}) is not higher than normal ({normal_damage:.4f})")
                all_pass = False
            else:
                passed_tests += 1
                diff = results[rule] - normal_damage
                pct = (diff / normal_damage * 100) if normal_damage > 0 else float('inf')
                print(f"  ✓ {rule}: +{diff:.4f} (+{pct:.1f}%)")

    print("\n" + "=" * 80)
    print(f"Results: {passed_tests}/{total_tests} tests passed")

    if all_pass:
        print("✅ ALL TESTS PASSED: All crit rules increase average damage")
    else:
        print("❌ SOME TESTS FAILED: Some crit rules do not increase damage")

    # In pytest, signal failure via assertion instead of return value
    assert all_pass, "Some crit rules did not increase average damage compared to normal hits"


if __name__ == "__main__":
    # When run directly, execute via pytest to honor assertions
    import pytest
    raise SystemExit(pytest.main([__file__]))
