#!/usr/bin/env python3
"""
Test the build_standard_sequence helper function in Python bindings.
"""

import rs_aos_stats as aos


def test_build_standard_sequence():
    """Test that build_standard_sequence mirrors the per-attack pipeline."""

    test_cases = [
        ("normal", False, ["HitRule", "WoundRule", "SaveRule", "DamagesRule"]),
        ("normal", True, ["HitRule", "WoundRule", "SaveRule", "DamagesRule", "WardRule"]),
        ("crit_auto_wound", False, ["CritAutoWoundRule", "WoundRule", "SaveRule", "DamagesRule"]),
        ("crit_mortal_wound", True, ["CritMortalWoundRule", "WoundRule", "SaveRule", "DamagesRule", "WardRule"]),
        ("crit_double_hit", False, ["CritDoubleHitRule", "WoundRule", "SaveRule", "DamagesRule"]),
    ]

    for hit_rule_type, has_ward, expected_sequence in test_cases:
        # Build sequence using the helper
        sequence = aos.build_standard_sequence(hit_rule_type, has_ward)

        # Verify sequence structure
        rule_types = [type(r).__name__ for r in sequence]
        assert rule_types == expected_sequence, f"Expected {expected_sequence}, got {rule_types}"
        if has_ward:
            assert "WardRule" in rule_types, "Should include WardRule when has_ward=True"
            ward_idx = rule_types.index("WardRule")
            damages_idx = rule_types.index("DamagesRule")
            assert damages_idx < ward_idx, \
                f"DamagesRule (index {damages_idx}) must come before WardRule (index {ward_idx})"

    # Test error handling
    try:
        aos.build_standard_sequence("invalid_rule", False)
        assert False, "Should have raised ValueError"
    except ValueError:
        pass

    # Test that ward actually reduces damage (using new API)
    attack_stats = aos.AttackStats(10, 3, 3, 1, 2)
    defense_stats_with_ward = aos.DefenseStats(4, 4)  # 4+ save, 4+ ward
    defense_stats_no_ward = aos.DefenseStats(4, None)  # 4+ save, no ward
    config_with_ward = aos.CombatConfig(attack_stats, defense_stats_with_ward, None)
    config_no_ward = aos.CombatConfig(attack_stats, defense_stats_no_ward, None)

    result_no_ward = aos.compute_damages(config_no_ward, "normal")
    result_with_ward = aos.compute_damages(config_with_ward, "normal")

    mean_no_ward = sum(d * p for d, p in result_no_ward)
    mean_with_ward = sum(d * p for d, p in result_with_ward)

    assert mean_with_ward < mean_no_ward, \
        f"Ward save must reduce damage: {mean_with_ward:.4f} >= {mean_no_ward:.4f}"


if __name__ == "__main__":
    test_build_standard_sequence()
    print("All tests passed!")
